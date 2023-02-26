use actix_web::{http::Method, HttpMessage, HttpRequest, HttpResponse};
use queryst::parse;
use serde_json::{json, Value};
pub mod users;

#[derive(Debug)]
pub struct ApiRequest<T = Value> {
    _raw: HttpRequest,
    pub model: String,
    pub action: String,
    pub method: Method,
    pub error: Option<(String, u16)>,
    pub data: T,
    pub path: String,
}

impl ApiRequest {
    pub fn basic(req: HttpRequest, body: String) -> Self {
        let mut error = None;
        let model = req
            .match_info()
            .get("model")
            .unwrap_or_else(|| {
                error = Some(("No model given".to_string(), 400));
                "unknown"
            })
            .to_string();
        let action = req
            .match_info()
            .get("action")
            .unwrap_or_else(|| {
                error = Some(("No action given".to_string(), 400));
                "unknown"
            })
            .to_string();
        let path = req.path().to_string();
        let method = req.method().clone();
        let data = match method {
            Method::GET => {
                let query_string = req.query_string();
                parse(query_string).unwrap_or_else(|err| {
                    error = Some((format!("Error parsing: {:?}", err), 400));
                    Value::Null
                })
            }
            Method::POST => match req.content_type() {
                "application/json" => serde_json::from_str(&body).unwrap_or_else(|err| {
                    error = Some((format!("Error parsing: {:?}", err), 400));
                    Value::Null
                }),
                _ => {
                    error = Some((format!("Content Type \"{}\" not implemented", method), 501)); // might as well be 415 (Unsupported Media Type)
                    Value::Null
                }
            },
            Method::PUT => json!({
                "filename": path,
                "content": body,
                "mime": req.content_type(),
            }),
            Method::DELETE => Value::Null,
            _ => {
                error = Some(("Not implemented".to_string(), 501));
                Value::Null
            }
        };

        println!(
            "{} {} {} {} {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            method,
            path,
            model,
            action
        );

        Self {
            _raw: req,
            model,
            action,
            method,
            error,
            data,
            path,
        }
    }
}

pub fn respond(data: Result<Value, (String, u16)>) -> HttpResponse {
    match data {
        Ok(data) => HttpResponse::Ok().json(json!({ "status": "ok", "data":data })),
        Err(err) => {
            let error = json!({
                    "status": "error",
                    "error":err.0
            });
            match err.1 {
                400 => return HttpResponse::BadRequest().json(error),
                401 => return HttpResponse::Unauthorized().json(error),
                402 => return HttpResponse::PaymentRequired().json(error),
                403 => return HttpResponse::Forbidden().json(error),
                404 => return HttpResponse::NotFound().json(error),
                405 => return HttpResponse::MethodNotAllowed().json(error),
                406 => return HttpResponse::NotAcceptable().json(error),
                407 => return HttpResponse::ProxyAuthenticationRequired().json(error),
                408 => return HttpResponse::RequestTimeout().json(error),
                409 => return HttpResponse::Conflict().json(error),
                410 => return HttpResponse::Gone().json(error),
                411 => return HttpResponse::LengthRequired().json(error),
                412 => return HttpResponse::PreconditionFailed().json(error),
                413 => return HttpResponse::PayloadTooLarge().json(error),
                414 => return HttpResponse::UriTooLong().json(error),
                415 => return HttpResponse::UnsupportedMediaType().json(error),
                416 => return HttpResponse::RangeNotSatisfiable().json(error),
                417 => return HttpResponse::ExpectationFailed().json(error),
                422 => return HttpResponse::UnprocessableEntity().json(error),
                429 => return HttpResponse::TooManyRequests().json(error),
                431 => return HttpResponse::RequestHeaderFieldsTooLarge().json(error),
                451 => return HttpResponse::UnavailableForLegalReasons().json(error),

                500 => return HttpResponse::InternalServerError().json(error),
                501 => return HttpResponse::NotImplemented().json(error),
                502 => return HttpResponse::BadGateway().json(error),
                503 => return HttpResponse::ServiceUnavailable().json(error),
                504 => return HttpResponse::GatewayTimeout().json(error),
                505 => return HttpResponse::VersionNotSupported().json(error),
                506 => return HttpResponse::VariantAlsoNegotiates().json(error),
                507 => return HttpResponse::InsufficientStorage().json(error),
                508 => return HttpResponse::LoopDetected().json(error),
                _ => return HttpResponse::InternalServerError().json(error),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_mapping() {
        for i in 400..600 {
            let response = respond(Err(("".to_string(), i)));
            assert_ne!(response.status().as_u16(), 200);
        }
    }
}
