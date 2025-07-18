use actix_web::{HttpRequest, dev::ServiceRequest, http::header::HeaderMap};

pub trait RequestHeader {
    fn headers(&self) -> &HeaderMap;

    fn get_header_value(&self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        if let Some(value) = self.headers().get(key) {
            let value_str = value.to_str()?.to_string();

            if value_str.is_empty() {
                return Ok(None);
            } else {
                return Ok(Some(value_str));
            }
        } else {
            return Ok(None);
        }
    }
}

impl RequestHeader for HttpRequest {
    fn headers(&self) -> &HeaderMap {
        self.headers()
    }
}

impl RequestHeader for ServiceRequest {
    fn headers(&self) -> &HeaderMap {
        self.headers()
    }
}
