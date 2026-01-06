pub mod rpc;
pub mod util;

use async_trait::async_trait;
use futures::{
    channel::oneshot, future::FusedFuture, lock::Mutex as AsyncMutex, sink::Sink, Future,
    FutureExt, SinkExt,
};
use handler::Handler;
use lsp_types::{
    notification::{self, Notification},
    request as req,
    request::Request,
    NumberOrString,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::HashMap,
    io, mem,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Poll, Waker},
};
use tracing::Instrument;

mod handler;

#[cfg(any(feature = "tokio-stdio", feature = "tokio-tcp"))]
pub mod listen;

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

        if let Some(w) = (*self.waker.lock().unwrap()).take() {
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

    pub fn as_err(&mut self) -> CancelTokenErr<'_> {
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

#[async_trait(?Send)]
pub trait ResponseWriter: Sized {
    async fn write_response<R: Serialize>(
        mut self,
        response: &rpc::Response<R>,
    ) -> Result<(), io::Error>;
}

#[async_trait(?Send)]
pub trait RequestWriter {
    async fn write_request<
        R: Request<Params = P>,
        P: Serialize + DeserializeOwned + core::fmt::Debug,
    >(
        &mut self,
        params: Option<R::Params>,
    ) -> Result<rpc::Response<R::Result>, io::Error>;

    async fn write_notification<
        N: Notification<Params = P>,
        P: Serialize + DeserializeOwned + core::fmt::Debug,
    >(
        &mut self,
        params: Option<N::Params>,
    ) -> Result<(), io::Error>;

    async fn cancel(&mut self) -> Result<(), io::Error>;
}

trait NewTrait: Future<Output = ()> {}
impl<T> NewTrait for T where T: Future<Output = ()> {}

type DeferredTasks = Arc<AsyncMutex<Vec<Pin<Box<dyn NewTrait>>>>>;

#[derive(Clone)]
pub struct Context<W: Clone> {
    inner: Arc<AsyncMutex<Inner<W>>>,
    cancel_token: CancelToken,
    last_req_id: Option<rpc::RequestId>, // For cancellation
    rw: Arc<AsyncMutex<Box<dyn MessageWriter>>>,
    world: W,
    deferred: DeferredTasks,
}

impl<W: Clone> std::ops::Deref for Context<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.world
    }
}

impl<W: Clone> Context<W> {
    pub async fn is_initialized(&self) -> bool {
        self.inner.lock().await.initialized
    }

    pub async fn is_shutting_down(&self) -> bool {
        self.inner.lock().await.shutting_down
    }

    pub fn world(&self) -> &W {
        &self.world
    }

    pub fn cancel_token(&mut self) -> &mut CancelToken {
        &mut self.cancel_token
    }

    /// Defer the execution of the future until after the
    /// handler returned (and response was sent if applicable).
    ///
    /// If sending a response fails, deferred futures
    /// won't be executed.
    pub async fn defer<F: Future<Output = ()> + 'static>(&self, fut: F) {
        self.deferred.lock().await.push(Box::pin(fut));
    }
}

#[async_trait(?Send)]
impl<W: Clone> RequestWriter for Context<W> {
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    async fn write_request<
        R: Request<Params = P>,
        P: Serialize + DeserializeOwned + core::fmt::Debug,
    >(
        &mut self,
        params: Option<R::Params>,
    ) -> Result<rpc::Response<R::Result>, io::Error> {
        let mut inner = self.inner.lock().await;
        let req_id = inner.next_request_id;
        inner.next_request_id += 1;

        let mut rw = self.rw.lock().await;

        let id = NumberOrString::Number(req_id);

        let request = rpc::Request::new()
            .with_id(id.clone().into())
            .with_method(R::METHOD)
            .with_params(params);

        let span = tracing::debug_span!("sending request", ?request);

        rw.send(request.into_message()).instrument(span).await?;

        self.last_req_id = Some(id.clone());

        let (send, recv) = oneshot::channel();
        inner.requests.insert(id, send);

        drop(inner);

        let res = recv.await.unwrap();

        tracing::trace!(response = ?res, "received response");

        self.last_req_id = None;

        Ok(res.into_params())
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    async fn write_notification<
        N: Notification<Params = P>,
        P: Serialize + DeserializeOwned + core::fmt::Debug,
    >(
        &mut self,
        params: Option<N::Params>,
    ) -> Result<(), io::Error> {
        let mut rw = self.rw.lock().await;
        rw.send(
            rpc::Request::new()
                .with_method(N::METHOD)
                .with_params(params)
                .into_message(),
        )
        .await
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

pub trait MessageWriter: Sink<rpc::Message, Error = io::Error> + Unpin {}
impl<T: Sink<rpc::Message, Error = io::Error> + Unpin> MessageWriter for T {}

struct Inner<W: Clone> {
    next_request_id: i32,
    initialized: bool,
    shutting_down: bool,
    handlers: HashMap<String, Box<dyn Handler<W>>>,
    tasks: HashMap<rpc::RequestId, Cancellation>,
    requests: HashMap<rpc::RequestId, oneshot::Sender<rpc::Response<serde_json::Value>>>,
}

impl<W: Clone> Inner<W> {
    fn task_done(&mut self, id: &rpc::RequestId) {
        if let Some(mut t) = self.tasks.remove(id) {
            t.cancel();
            tracing::trace!(?id, "task completed");
        }
    }
}

pub struct Server<W: Clone> {
    inner: Arc<AsyncMutex<Inner<W>>>,
}

impl<W: Clone> Server<W> {
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
            },
        }
    }

    pub fn handle_message(
        &self,
        world: W,
        message: rpc::Message,
        writer: impl MessageWriter + Clone + 'static,
    ) -> impl Future<Output = Result<(), io::Error>> {
        let inner = self.inner.clone();

        async move {
            if message.is_response() {
                Server::handle_response(inner, message.into_response()).await;
                Ok(())
            } else {
                Server::handle_request(inner, world, message.into_request(), writer).await
            }
        }
    }

    pub async fn is_shutting_down(&self) -> bool {
        self.inner.lock().await.shutting_down
    }

    #[tracing::instrument(level = tracing::Level::TRACE)]
    async fn handle_response(
        inner: Arc<AsyncMutex<Inner<W>>>,
        response: rpc::Response<serde_json::Value>,
    ) {
        if let Some(sender) = inner.lock().await.requests.remove(&response.id) {
            sender.send(response).ok();
        } else {
            tracing::error!(?response, "unexpected response")
        }
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip(data, writer))]
    async fn handle_request(
        inner: Arc<AsyncMutex<Inner<W>>>,
        data: W,
        request: rpc::Request<serde_json::Value>,
        mut writer: impl MessageWriter + Clone + 'static,
    ) -> Result<(), io::Error> {
        if &request.jsonrpc != "2.0" {
            tracing::error!("JSON-RPC message version is not 2.0");
            return writer
                .send(
                    rpc::Response::error(
                        rpc::Error::invalid_request()
                            .with_data("only JSON-RPC version 2.0 is accepted"),
                    )
                    .into_message(),
                )
                .await;
        }

        if request.id.is_some() {
            tracing::debug!(
                id = ?request.id.as_ref().unwrap(),
                method = %request.method,
                "request received"
            );
            let mut s = inner.lock().await;

            if s.shutting_down {
                tracing::warn!(
                    id = ?request.id.as_ref().unwrap(),
                    method = %request.method,
                    "received request while shutting down"
                );

                writer
                    .send(
                        rpc::Response::error(
                            rpc::Error::invalid_request().with_data("server is shutting down"),
                        )
                        .into_message(),
                    )
                    .await?;
                return Ok(());
            }

            if request.method == req::Shutdown::METHOD {
                tracing::info!(
                    id = ?request.id.as_ref().unwrap(),
                    method = %request.method,
                    "received shutdown request"
                );

                s.shutting_down = true;
            }

            let is_initialize = request.method == req::Initialize::METHOD;

            if !s.initialized && !is_initialize {
                tracing::error!(
                    id = ?request.id.as_ref().unwrap(),
                    method = %request.method,
                    "server not yet initialized"
                );

                writer
                    .send(rpc::Response::error(rpc::Error::server_not_initialized()).into_message())
                    .await?;
                return Ok(());
            }

            if s.handlers.contains_key(&request.method) {
                let mut handler = s.handlers.get_mut(&request.method).unwrap().clone();

                let id = request.id.clone().unwrap();

                // We expect the handler to run for a longer time
                drop(s);

                let ctx = Server::create_context(
                    inner.clone(),
                    Arc::new(AsyncMutex::new(Box::new(writer.clone()))),
                    data,
                    &request,
                )
                .await;

                let handler_span = tracing::trace_span!(
                    "request handler",
                    method = %request.method,
                );

                let method = request.method.clone();

                handler
                    .handle(ctx.clone(), request, Some(&mut writer))
                    .instrument(handler_span)
                    .await;

                let deferred = mem::take(&mut (*ctx.deferred.lock().await));

                for d in deferred {
                    let deferred_span = tracing::trace_span!(
                        "deferred task",
                        %method,
                    );

                    d.instrument(deferred_span).await
                }

                let mut s = inner.lock().await;

                s.task_done(&id);
                if is_initialize {
                    s.initialized = true;
                }
                drop(s);

                Ok(())
            } else if request.method == req::Shutdown::METHOD {
                // Shutting down without handler, everything should be OK.
                writer
                    .send(
                        rpc::Response::success(())
                            .with_request_id(request.id.unwrap())
                            .into_message(),
                    )
                    .await
            } else {
                tracing::error!(
                    method = %request.method,
                    "no request handler registered"
                );

                writer
                    .send(
                        rpc::Response::error(rpc::Error::method_not_found())
                            .with_request_id(request.id.unwrap())
                            .into_message(),
                    )
                    .await
            }
        } else {
            tracing::debug!(
                method = %request.method,
                "notification received"
            );

            if request.method == lsp_types::notification::Cancel::METHOD {
                if let Some(p) = request.params {
                    if let Ok(c) = serde_json::from_value::<lsp_types::CancelParams>(p) {
                        inner.lock().await.task_done(&c.id);
                    }
                }
                return Ok(());
            }

            let mut s = inner.lock().await;

            if s.handlers.contains_key(&request.method) {
                let mut handler = s.handlers.get_mut(&request.method).unwrap().clone();
                drop(s);

                let ctx = Server::create_context(
                    inner,
                    Arc::new(AsyncMutex::new(Box::new(writer))),
                    data,
                    &request,
                )
                .await;

                let handler_span = tracing::trace_span!(
                    "notification handler",
                    method = %request.method,
                );

                let method = request.method.clone();

                handler
                    .handle(ctx.clone(), request, None)
                    .instrument(handler_span)
                    .await;

                let deferred = mem::take(&mut (*ctx.deferred.lock().await));

                for d in deferred {
                    let deferred_span = tracing::trace_span!(
                        "deferred task",
                        %method,
                    );

                    d.instrument(deferred_span).await
                }
            } else {
                tracing::warn!(
                    method = %request.method,
                    "no notification handler registered"
                );
            }

            Ok(())
        }
    }

    async fn create_context<D>(
        inner: Arc<AsyncMutex<Inner<W>>>,
        rw: Arc<AsyncMutex<Box<dyn MessageWriter>>>,
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
            rw,
            deferred: Default::default(),
        }
    }
}

pub struct ServerBuilder<W: Clone + 'static> {
    inner: Inner<W>,
}

impl<W: Clone + 'static> ServerBuilder<W> {
    pub fn on_notification<N, F>(mut self, handler: fn(Context<W>, Params<N::Params>) -> F) -> Self
    where
        N: Notification + 'static,
        F: Future<Output = ()> + 'static,
    {
        self.inner.handlers.insert(
            N::METHOD.into(),
            Box::new(handler::NotificationHandler::<N, _, _>::new(handler)),
        );
        tracing::info!(method = N::METHOD, "registered notification handler");
        self
    }

    pub fn on_request<R, F>(mut self, handler: fn(Context<W>, Params<R::Params>) -> F) -> Self
    where
        R: Request + 'static,
        F: Future<Output = Result<R::Result, rpc::Error>> + 'static,
    {
        self.inner.handlers.insert(
            R::METHOD.into(),
            Box::new(handler::RequestHandler::<R, _, _>::new(handler)),
        );
        tracing::info!(method = R::METHOD, "registered request handler");
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
