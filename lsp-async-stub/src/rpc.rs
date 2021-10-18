use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub type RequestId = lsp_types::NumberOrString;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub jsonrpc: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RequestId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl Message {
    pub fn is_notification(&self) -> bool {
        self.method.is_some() && self.id.is_none()
    }

    pub fn is_response(&self) -> bool {
        self.method.is_none()
    }

    pub fn into_request(self) -> Request<serde_json::Value> {
        Request {
            jsonrpc: self.jsonrpc,
            method: self.method.unwrap(),
            id: self.id,
            params: self.params,
        }
    }

    pub fn into_response(self) -> Response<serde_json::Value> {
        Response {
            jsonrpc: self.jsonrpc,
            id: self.id.unwrap(),
            error: self.error,
            result: self.result,
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<T = ()> {
    pub jsonrpc: String,
    pub method: String,

    pub id: Option<RequestId>,
    pub params: Option<T>,
}

impl<T: Serialize + DeserializeOwned> Request<T> {
    pub fn new() -> Self {
        Self {
            jsonrpc: "2.0".into(),
            method: "".into(),
            id: None,
            params: None,
        }
    }

    pub fn with_method(self, method: &str) -> Self {
        Self {
            method: method.into(),
            ..self
        }
    }

    pub fn with_id(self, id: Option<RequestId>) -> Self {
        Self { id, ..self }
    }

    pub fn with_params(self, params: Option<T>) -> Self {
        Self { params, ..self }
    }

    pub fn into_message(self) -> Message {
        Message {
            jsonrpc: self.jsonrpc,
            method: Some(self.method),
            id: self.id,
            params: self.params.map(|p| serde_json::to_value(p).unwrap()),
            result: None,
            error: None,
        }
    }
}

impl Request<serde_json::Value> {
    pub fn into_params<P: DeserializeOwned>(self) -> Result<Request<P>, serde_json::Error> {
        match self.params {
            None => Ok(Request {
                id: self.id,
                jsonrpc: self.jsonrpc,
                method: self.method,
                params: None,
            }),
            Some(v) => match serde_json::from_value(v) {
                Ok(params) => Ok(Request {
                    id: self.id,
                    jsonrpc: self.jsonrpc,
                    method: self.method,
                    params: Some(params),
                }),
                Err(e) => Err(e),
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response<R = ()> {
    pub jsonrpc: String,

    pub id: RequestId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<R>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl<R> Response<R> {
    pub fn with_request_id(self, id: RequestId) -> Self {
        Response { id, ..self }
    }
}

impl<R: Serialize + DeserializeOwned> Response<R> {
    pub fn success(data: R) -> Self {
        Response {
            jsonrpc: "2.0".into(),
            id: lsp_types::NumberOrString::Number(0),
            result: Some(data),
            error: None,
        }
    }

    pub fn into_result(self) -> Result<R, Error> {
        if let Some(r) = self.result {
            Ok(r)
        } else {
            Err(self.error.unwrap())
        }
    }

    pub fn into_message(self) -> Message {
        Message {
            jsonrpc: self.jsonrpc,
            method: None,
            id: Some(self.id),
            params: None,
            result: self.result.map(|p| serde_json::to_value(p).unwrap()),
            error: self.error,
        }
    }
}

impl Response<serde_json::Value> {
    pub fn into_params<P: DeserializeOwned>(self) -> Response<P> {
        Response {
            jsonrpc: self.jsonrpc,
            id: self.id,
            result: self.result.map(|v| serde_json::from_value(v).unwrap()),
            error: self.error,
        }
    }
}

impl Response<()> {
    pub fn error(err: Error) -> Self {
        Response {
            jsonrpc: "2.0".into(),
            id: lsp_types::NumberOrString::Number(0),
            result: None,
            error: Some(err),
        }
    }
}

impl<E, R> From<Result<R, E>> for Response<R>
where
    R: Serialize + for<'r> Deserialize<'r>,
    E: Into<Error>,
{
    fn from(res: Result<R, E>) -> Self {
        match res {
            Ok(r) => Response {
                jsonrpc: "2.0".into(),
                id: lsp_types::NumberOrString::Number(0),
                result: Some(r),
                error: None,
            },
            Err(err) => Response {
                jsonrpc: "2.0".into(),
                id: lsp_types::NumberOrString::Number(0),
                result: None,
                error: Some(err.into()),
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RPC error ({}): {}", self.code, self.message)
    }
}

impl Error {
    pub fn new(message: &str) -> Self {
        Error {
            code: 0,
            message: message.into(),
            data: None,
        }
    }

    pub fn with_code(mut self, code: i32) -> Self {
        self.code = code;
        self
    }

    pub fn with_data(mut self, data: impl Serialize) -> Self {
        self.data = Some(serde_json::to_value(data).unwrap());
        self
    }

    pub fn parse() -> Error {
        Error {
            code: -32700,
            message: "Parse error".into(),
            data: None,
        }
    }

    pub fn invalid_request() -> Error {
        Error {
            code: -32600,
            message: "Invalid request".into(),
            data: None,
        }
    }

    pub fn method_not_found() -> Error {
        Error {
            code: -32601,
            message: "Method not found".into(),
            data: None,
        }
    }

    pub fn invalid_params() -> Error {
        Error {
            code: -32602,
            message: "Invalid params".into(),
            data: None,
        }
    }

    pub fn internal_error() -> Error {
        Error {
            code: -32603,
            message: "Internal error".into(),
            data: None,
        }
    }

    pub fn server_not_initialized() -> Error {
        Error {
            code: -32002,
            message: "Server not initialized".into(),
            data: None,
        }
    }

    pub fn request_cancelled() -> Error {
        Error {
            code: -32800,
            message: "Request cancelled".into(),
            data: None,
        }
    }

    pub fn content_modified() -> Error {
        Error {
            code: -32801,
            message: "Content modified".into(),
            data: None,
        }
    }

    pub fn server(code: i32) -> Error {
        if code < -32000 || code > -32099 {
            panic!("code must be between -32000 and -32099")
        }

        Error {
            code: -32603,
            message: "Server error".into(),
            data: None,
        }
    }
}

impl std::error::Error for Error {}
