use actix_web::{http::Method, HttpRequest};
use queryst::parse;
use serde_json::Value;
pub mod users;
pub struct ApiRequest<T = Value> {
    _raw: HttpRequest,
    pub model: String,
    pub action: String,
    pub method: Method,
    pub error: Option<String>,
    pub data: T,
}
impl ApiRequest {
    pub fn basic(req: HttpRequest) -> Self {
        let mut error: Option<String> = None;
        let model = req
            .match_info()
            .get("model")
            .unwrap_or_else(|| {
                error = Some("No model given".to_string());
                "unknown"
            })
            .to_string();
        let action = req
            .match_info()
            .get("action")
            .unwrap_or_else(|| {
                error = Some("No action given".to_string());
                "unknown"
            })
            .to_string();
        let method = req.method().clone();
        let data = match method {
            Method::GET => {
                let query_string = req.query_string();
                parse(query_string).unwrap_or_else(|err| {
                    error = Some(format!("Error parsing: {:?}", err));
                    Value::Null
                })
            }
            Method::POST => {
                error = Some("Not implemented".to_string());
                Value::Null
            }
            Method::DELETE => {
                error = Some("Not implemented".to_string());
                Value::Null
            }
            Method::PUT => {
                error = Some("Not implemented".to_string());
                Value::Null
            }
            _ => {
                error = Some("Not implemented".to_string());
                Value::Null
            }
        };
        Self {
            _raw: req,
            model,
            action,
            method,
            error,
            data,
        }
    }
}
