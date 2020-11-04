use async_trait::async_trait;
use futures::{
    channel::oneshot, future::FusedFuture, lock::Mutex as AsyncMutex, sink::Sink, Future,
    FutureExt, SinkExt,
};
use lsp_types::{
    notification::{self, Notification},
    request as req,
    request::Request,
    NumberOrString,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::HashMap,
    io,
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Poll, Waker},
};

pub mod rpc;

#[derive(Debug, Clone, Default)]
struct Cancellation {
    cancelled: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl Cancellation {
    pub fn token(&self) -> CancelToken {
        CancelToken {
            cancelled: self.cancelled.clone(),
            waker_set: Arc::new(AtomicBool::new(false)),
            waker: self.waker.clone(),
        }
    }

    pub fn cancel(&mut self) {
        self.cancelled.store(true, Ordering::SeqCst);

        if let Some(w) = std::mem::replace(&mut *self.waker.lock().unwrap(), None) {
            w.wake();
        }
    }
}

#[derive(Debug, Clone)]
pub struct CancelToken {
    cancelled: Arc<AtomicBool>,
    waker_set: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl CancelToken {
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    pub fn as_err(&mut self) -> CancelTokenErr {
        CancelTokenErr(self)
    }
}

impl Future for CancelToken {
    type Output = ();

    #[allow(unused_mut)] // differs between compiler versions
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        if self.cancelled.load(Ordering::SeqCst) {
            Poll::Ready(())
        } else {
            if !self.waker_set.load(Ordering::SeqCst) {
                *self.waker.lock().unwrap() = Some(cx.waker().clone());
            }

            Poll::Pending
        }
    }
}

impl FusedFuture for CancelToken {
    fn is_terminated(&self) -> bool {
        false
    }
}

pub struct CancelTokenErr<'t>(&'t mut CancelToken);

impl Future for CancelTokenErr<'_> {
    type Output = Result<(), rpc::Error>;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        match self.0.poll_unpin(cx) {
            Poll::Ready(_) => Poll::Ready(Err(rpc::Error::request_cancelled())),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl FusedFuture for CancelTokenErr<'_> {
    fn is_terminated(&self) -> bool {
        false
    }
}

#[async_trait]
pub trait ResponseWriter: Sized {
    async fn write_response<R: Serialize + Send + Sync>(
        mut self,
        response: &rpc::Response<R>,
    ) -> Result<(), io::Error>;
}

#[async_trait]
pub trait RequestWriter {
    async fn write_request<R: Request<Params = P>, P: Serialize + DeserializeOwned + Send + Sync>(
        &mut self,
        params: Option<R::Params>,
    ) -> Result<rpc::Response<R::Result>, io::Error>;

    async fn write_notification<
        N: Notification<Params = P>,
        P: Serialize + DeserializeOwned + Send + Sync,
    >(
        &mut self,
        params: Option<N::Params>,
    ) -> Result<(), io::Error>;

    async fn cancel(&mut self) -> Result<(), io::Error>;
}

pub struct ResponseWriterBuffer {
    res: Option<rpc::Response<serde_json::Value>>,
}

impl ResponseWriterBuffer {
    fn write_response<R: Serialize>(&mut self, response: rpc::Response<R>) {
        let res = match response.result {
            None => None,
            Some(r) => Some(serde_json::to_value(r).unwrap()),
        };

        self.res = Some(rpc::Response {
            jsonrpc: response.jsonrpc,
            id: response.id,
            result: res,
            error: response.error,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Context<W: Clone + Send + Sync> {
    inner: Arc<AsyncMutex<Inner<W>>>,
    cancel_token: CancelToken,
    last_req_id: Option<rpc::RequestId>, // For cancellation
    world: W,
}

impl<W: Clone + Send + Sync> Context<W> {
    pub async fn is_initialized(&self) -> bool {
        self.inner.lock().await.initialized
    }

    pub async fn is_shutting_down(&self) -> bool {
        self.inner.lock().await.shutting_down
    }

    pub fn world(&mut self) -> &mut W {
        &mut self.world
    }

    pub fn cancel_token(&mut self) -> &mut CancelToken {
        &mut self.cancel_token
    }
}

#[async_trait]
impl<W: Clone + Send + Sync> RequestWriter for Context<W> {
    async fn write_request<
        R: Request<Params = P>,
        P: Serialize + DeserializeOwned + Send + Sync,
    >(
        &mut self,
        params: Option<R::Params>,
    ) -> Result<rpc::Response<R::Result>, io::Error> {
        let mut inner = self.inner.lock().await;
        let req_id = inner.next_request_id;
        inner.next_request_id += 1;

        if let Some(rw) = &mut inner.rw {
            let id = NumberOrString::Number(req_id);

            rw.send(
                rpc::Request::new()
                    .with_id(id.clone().into())
                    .with_method(R::METHOD)
                    .with_params(params)
                    .into_message(),
            )
            .await
            .unwrap();

            self.last_req_id = Some(id.clone());

            let (send, recv) = oneshot::channel();
            inner.requests.insert(id, send);

            drop(inner);

            let res = recv.await.unwrap();
            self.last_req_id = None;

            return Ok(res.into_params());
        }

        Err(io::Error::new(io::ErrorKind::Other, "not supported"))
    }

    async fn write_notification<
        N: Notification<Params = P>,
        P: Serialize + DeserializeOwned + Send + Sync,
    >(
        &mut self,
        params: Option<N::Params>,
    ) -> Result<(), io::Error> {
        let mut inner = self.inner.lock().await;

        if let Some(rw) = &mut inner.rw {
            rw.send(
                rpc::Request::new()
                    .with_method(N::METHOD)
                    .with_params(params)
                    .into_message(),
            )
            .await
            .unwrap();

            return Ok(());
        }

        Err(io::Error::new(io::ErrorKind::Other, "not supported"))
    }

    async fn cancel(&mut self) -> Result<(), io::Error> {
        if let Some(id) = Option::take(&mut self.last_req_id) {
            self.write_notification::<notification::Cancel, _>(Some(lsp_types::CancelParams { id }))
                .await
        } else {
            Ok(())
        }
    }
}

pub trait MessageSink: Sink<rpc::Message, Error = io::Error> + Send + Sync + Unpin {}
impl<T: Sink<rpc::Message, Error = io::Error> + Send + Sync + Unpin> MessageSink for T {}

struct Inner<W: Clone + Send + Sync> {
    next_request_id: u64,
    initialized: bool,
    shutting_down: bool,
    handlers: HashMap<String, Box<dyn Handler<W>>>,
    tasks: HashMap<rpc::RequestId, Cancellation>,
    requests: HashMap<rpc::RequestId, oneshot::Sender<rpc::Response<serde_json::Value>>>,
    rw: Option<Box<dyn MessageSink>>,
}

impl<W: Clone + Send + Sync> Inner<W> {
    fn task_done(&mut self, id: &rpc::RequestId) {
        if let Some(mut t) = self.tasks.remove(id) {
            t.cancel();
        }
    }
}

pub struct Server<W: Clone + Send + Sync> {
    inner: Arc<AsyncMutex<Inner<W>>>,
}

impl<W: Clone + Send + Sync> Server<W> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerBuilder<W> {
        ServerBuilder {
            inner: Inner {
                next_request_id: 0,
                initialized: false,
                shutting_down: false,
                handlers: HashMap::new(),
                tasks: HashMap::new(),
                requests: HashMap::new(),
                rw: None,
            },
        }
    }

    pub fn handle_message(
        &self,
        world: W,
        message: rpc::Message,
        response_writer: impl ResponseWriter,
    ) -> impl Future<Output = ()> {
        let inner = self.inner.clone();
        async move {
            if message.is_response() {
                Server::handle_response(inner, message.into_response()).await
            } else {
                Server::handle_request(inner, world, message.into_request(), response_writer).await
            }
        }
    }

    fn handle_response(
        inner: Arc<AsyncMutex<Inner<W>>>,
        response: rpc::Response<serde_json::Value>,
    ) -> impl Future<Output = ()> {
        async move {
            if let Some(sender) = inner.lock().await.requests.remove(&response.id) {
                sender.send(response).ok();
            }
        }
    }

    fn handle_request(
        inner: Arc<AsyncMutex<Inner<W>>>,
        data: W,
        request: rpc::Request<serde_json::Value>,
        response_writer: impl ResponseWriter,
    ) -> impl Future<Output = ()> {
        async move {
            if &request.jsonrpc != "2.0" {
                response_writer
                    .write_response(&rpc::Response::error(
                        rpc::Error::invalid_request()
                            .with_data("only JSON-RPC version 2.0 is accepted"),
                    ))
                    .await
                    .unwrap();
                return;
            }

            if request.id.is_some() {
                let mut s = inner.lock().await;

                if s.shutting_down {
                    response_writer
                        .write_response(&rpc::Response::error(
                            rpc::Error::invalid_request().with_data("server is shutting down"),
                        ))
                        .await
                        .unwrap();
                    return;
                }

                if request.method == req::Shutdown::METHOD {
                    s.shutting_down = true;
                }

                let is_initialize = request.method == req::Initialize::METHOD;

                if !s.initialized && !is_initialize {
                    response_writer
                        .write_response(&rpc::Response::error(rpc::Error::server_not_initialized()))
                        .await
                        .unwrap();
                    return;
                }

                if s.handlers.contains_key(&request.method) {
                    let mut handler = s.handlers.get_mut(&request.method).unwrap().clone();

                    let id = request.id.clone().unwrap();

                    // We expect the handler to run for a longer time
                    drop(s);

                    let mut w = ResponseWriterBuffer { res: None };
                    let ctx = Server::create_context(inner.clone(), data, &request).await;
                    handler.handle(ctx, request, Some(&mut w)).await;

                    let mut s = inner.lock().await;

                    s.task_done(&id);
                    if is_initialize {
                        s.initialized = true;
                    }
                    drop(s);

                    if let Some(r) = w.res {
                        response_writer.write_response(&r).await.unwrap();
                    }
                } else {
                    response_writer
                        .write_response(&rpc::Response::error(rpc::Error::method_not_found()))
                        .await
                        .unwrap();
                }
            } else {
                if request.method == lsp_types::notification::Cancel::METHOD {
                    if let Some(p) = request.params {
                        if let Ok(c) = serde_json::from_value::<lsp_types::CancelParams>(p) {
                            inner.lock().await.task_done(&c.id);
                        }
                    }
                    return;
                }

                let mut s = inner.lock().await;

                if s.handlers.contains_key(&request.method) {
                    let mut handler = s.handlers.get_mut(&request.method).unwrap().clone();
                    drop(s);

                    let ctx = Server::create_context(inner, data, &request).await;
                    handler.handle(ctx, request, None).await;
                }
            }
        }
    }

    async fn create_context<D>(
        inner: Arc<AsyncMutex<Inner<W>>>,
        world: W,
        req: &rpc::Request<D>,
    ) -> Context<W> {
        let cancel = Cancellation::default();
        let cancel_token = cancel.token();

        if let Some(id) = &req.id {
            inner.lock().await.tasks.insert(id.clone(), cancel);
        }

        Context {
            cancel_token,
            world,
            inner,
            last_req_id: None,
        }
    }
}

pub struct ServerBuilder<W: Clone + Send + Sync> {
    inner: Inner<W>,
}

impl<W: Clone + Send + Sync> ServerBuilder<W> {
    pub fn handler(mut self, handler: impl Handler<W> + 'static) -> Self {
        self.inner
            .handlers
            .insert(handler.method().into(), Box::new(handler));
        self
    }

    pub fn request_writer<RW: MessageSink + 'static>(mut self, writer: RW) -> Self {
        self.inner.rw = Some(Box::new(writer));
        self
    }

    pub fn build(self) -> Server<W> {
        Server {
            inner: Arc::new(AsyncMutex::new(self.inner)),
        }
    }
}

pub struct Params<P>(Option<P>);

impl<P> Params<P> {
    pub fn optional(self) -> Option<P> {
        self.0
    }

    pub fn required(self) -> Result<P, rpc::Error> {
        match self.0 {
            None => Err(rpc::Error::invalid_params().with_data("params are required")),
            Some(p) => Ok(p),
        }
    }
}

impl<P> From<Option<P>> for Params<P> {
    fn from(p: Option<P>) -> Self {
        Self(p)
    }
}

#[async_trait]
pub trait Handler<W: Clone + Send + Sync>: Send {
    fn method(&self) -> &'static str;

    async fn handle(
        &mut self,
        context: Context<W>,
        message: rpc::Request<serde_json::Value>,
        writer: Option<&mut ResponseWriterBuffer>,
    );

    fn box_clone(&self) -> Box<dyn Handler<W>>;
}

pub struct RequestHandler<R, F, W>
where
    R: Request,
    F: Future<Output = Result<R::Result, rpc::Error>> + Send,
    W: Clone + Send + Sync,
{
    f: fn(Context<W>, Params<R::Params>) -> F,
    t: PhantomData<W>,
}

impl<R, F, W> Clone for RequestHandler<R, F, W>
where
    R: Request,
    F: Future<Output = Result<R::Result, rpc::Error>> + Send,
    W: Clone + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            f: self.f,
            t: Default::default(),
        }
    }
}

impl<R, F, W> RequestHandler<R, F, W>
where
    R: Request,
    F: Future<Output = Result<R::Result, rpc::Error>> + Send,
    W: Clone + Send + Sync,
{
    pub fn new(f: fn(Context<W>, Params<R::Params>) -> F) -> Self {
        Self {
            f,
            t: Default::default(),
        }
    }
}

#[async_trait]
impl<R, F, P, W> Handler<W> for RequestHandler<R, F, W>
where
    R: Request<Params = P> + 'static,
    P: Send + Serialize + DeserializeOwned + 'static,
    F: Future<Output = Result<R::Result, rpc::Error>> + Send + 'static,
    W: Clone + Send + Sync + 'static,
{
    fn method(&self) -> &'static str {
        R::METHOD
    }

    async fn handle(
        &mut self,
        context: Context<W>,
        message: rpc::Request<serde_json::Value>,
        writer: Option<&mut ResponseWriterBuffer>,
    ) {
        let req_id = message.id.clone();
        let req = match message.into_params::<R::Params>() {
            Ok(r) => r,
            Err(e) => {
                if let Some(w) = writer {
                    w.write_response(
                        rpc::Response::error(rpc::Error::invalid_params().with_data(e.to_string()))
                            .with_request_id(req_id.unwrap()),
                    );
                }

                return;
            }
        };

        let call_result = (self.f)(context, req.params.into()).await;

        if let Some(w) = writer {
            let res = rpc::Response::from(call_result)
            .with_request_id(req.id.unwrap());
            w.write_response(res);
        }
    }

    fn box_clone(&self) -> Box<dyn Handler<W>> {
        Box::new((*self).clone())
    }
}

impl<W: Clone + Send + Sync> Clone for Box<dyn Handler<W>> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct NotificationHandler<N, F, W>
where
    N: Notification,
    F: Future + Send,
    W: Clone + Send + Sync,
{
    f: fn(Context<W>, Params<N::Params>) -> F,
    t: PhantomData<W>,
}

impl<N, F, W> NotificationHandler<N, F, W>
where
    N: Notification,
    F: Future + Send,
    W: Clone + Send + Sync,
{
    pub fn new(f: fn(Context<W>, Params<N::Params>) -> F) -> Self {
        Self {
            f,
            t: Default::default(),
        }
    }
}

impl<N, F, W> NotificationHandler<N, F, W>
where
    N: Notification,
    F: Future + Send,
    W: Clone + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            f: self.f,
            t: Default::default(),
        }
    }
}

#[async_trait]
impl<N, F, P, W> Handler<W> for NotificationHandler<N, F, W>
where
    N: Notification<Params = P> + 'static,
    P: Send + Serialize + DeserializeOwned + 'static,
    F: Future + Send + 'static,
    W: Clone + Send + Sync + 'static,
{
    fn method(&self) -> &'static str {
        N::METHOD
    }

    async fn handle(
        &mut self,
        context: Context<W>,
        message: rpc::Request<serde_json::Value>,
        _writer: Option<&mut ResponseWriterBuffer>,
    ) {
        let req = match message.into_params::<N::Params>() {
            Ok(r) => r,
            Err(_) => return,
        };

        (self.f)(context, req.params.into()).await;
    }

    fn box_clone(&self) -> Box<dyn Handler<W>> {
        Box::new((*self).clone())
    }
}
