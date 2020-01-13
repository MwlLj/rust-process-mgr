// use reqwest::Client;
// use easy_http_request::DefaultHttpRequest;
use http_req::{request::Request, request::Method, uri::Uri};
// use urldecode;
use urlencoding;

const header_name: &str = "name";
const restart_all_url: &str = "/api/restart/all";
const restart_url: &str = "/api/restart";
const restart_by_alias_url: &str = "/api/restart/by/alias";

pub fn restart_all(addr: &str) -> Result<(), &str> {
    let mut url = String::new();
    url.push_str("http://");
    url.push_str(addr);
    url.push_str(restart_all_url);
    // let client = Client::new();
    /*
    let client = match DefaultHttpRequest::put_from_url_str(&url) {
        Ok(c) => c,
        Err(err) => {
            println!("err: {:?}", err);
            return Err("craete client error");
        }
    };
    */
    let uri: Uri = url.parse().unwrap();
    let mut writer = Vec::new();
    let response = match Request::new(&uri).method(Method::PUT).send(&mut writer) {
        Ok(r) => r,
        Err(err) => {
            println!("err: {:?}", err);
            return Err("send put request error");
        }
    };
    Ok(())
}

pub fn restart<'a>(addr: &'a str, name: &str) -> Result<(), &'a str> {
    let mut url = String::new();
    url.push_str("http://");
    url.push_str(addr);
    url.push_str(restart_url);
    let uri: Uri = url.parse().unwrap();
    let mut writer = Vec::new();
    let response = match Request::new(&uri).method(Method::PUT)
    .header(header_name, name)
    .send(&mut writer) {
        Ok(r) => r,
        Err(err) => {
            println!("err: {:?}", err);
            return Err("send put request error");
        }
    };
    Ok(())
}

pub fn restart_by_alias<'a>(addr: &'a str, name: &str) -> Result<(), &'a str> {
    let mut url = String::new();
    url.push_str("http://");
    url.push_str(addr);
    url.push_str(restart_by_alias_url);
    // let url = urldecode::decode(url);
    let uri: Uri = url.parse().unwrap();
    let mut writer = Vec::new();
    let n = urlencoding::encode(name);
    let response = match Request::new(&uri).method(Method::PUT)
    .header(header_name, &n)
    .send(&mut writer) {
        Ok(r) => r,
        Err(err) => {
            println!("err: {:?}", err);
            return Err("send put request error");
        }
    };
    Ok(())
}
