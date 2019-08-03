use tiny_http::{Server, Request, Response, Method, Header, StatusCode};
use base64;

const authorization: &str = "Authorization";

pub struct CAuthHandler {
}

impl CAuthHandler {
    pub fn handler<F>(&self, inPwd: &str, request: Request, f: F) -> bool
        where F: FnOnce(Request) {
        let auth = self.findHeader(&request.headers(), authorization);
        // println!("{:?}", auth);
        if *request.method() == Method::Get && auth == "" {
            let h = Header::from_bytes("WWW-Authenticate", r#"Basic realm="Dotcoo User Login""#).unwrap();
            let mut response = Response::from_data("");
            let mut response = response.with_status_code(401);
            response.add_header(h);
            request.respond(response);
            return false;
        }
        // println!("{:?}", auth);
        let v: Vec<&str> = auth.split(" ").collect();
        if v.len() < 2 {
            request.respond(Response::from_data("auth error"));
            return false;
        }
        if v[0] != "Basic" {
            request.respond(Response::from_data("auth is not Basic"));
            return false;
        }
        let bytes = match base64::decode(v[1]) {
            Ok(b) => b,
            Err(_) => {
                request.respond(Response::from_data("decode base64 error"));
                return false;
            }
        };
        let s = match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => {
                request.respond(Response::from_data("from utf8 error"));
                return false;
            }
        };
        let v: Vec<&str> = s.split(":").collect();
        if v.len() < 2 {
            request.respond(Response::from_data("split by : error"));
            return false;
        }
        if inPwd != v[1] {
            request.respond(Response::from_string("password error"));
            return false;
        } else {
            f(request);
            return true;
        }
        return false;
    }
}

impl CAuthHandler {
    fn findHeader(&self, headers: &[Header], key: &'static str) -> String {
        let mut value = String::new();
        for item in headers {
            if item.field.equiv(key) {
                value = item.value.as_str().to_string();
                break;
            }
        }
        value
    }
}

impl CAuthHandler {
    pub fn new() -> CAuthHandler {
        CAuthHandler{
        }
    }
}
