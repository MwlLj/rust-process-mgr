// use reqwest::Client;
// use easy_http_request::DefaultHttpRequest;
use http_req::{request::Request, request::Method, uri::Uri};

const header_name: &str = "name";
const stop_all_url: &str = "/api/stop/all";
const stop_url: &str = "/api/stop";
const stop_by_alias_url: &str = "/api/stop/by/alias";

pub fn stop_all(addr: &str) -> Result<(), &str> {
    let mut url = String::new();
    url.push_str("http://");
    url.push_str(addr);
    url.push_str(stop_all_url);
    /*
    let client = match DefaultHttpRequest::delete_from_url_str(&url) {
        Ok(c) => c,
        Err(err) => {
            println!("err: {:?}", err);
            return Err("craete client error");
        }
    };
    */
    let uri: Uri = url.parse().unwrap();
    let mut writer = Vec::new();
    let response = match Request::new(&uri).method(Method::DELETE).send(&mut writer) {
        Ok(r) => r,
        Err(err) => {
            println!("err: {:?}", err);
            return Err("send delete request error");
        }
    };
    Ok(())
}

pub fn stop<'a>(addr: &'a str, name: &str) -> Result<(), &'a str> {
    let mut url = String::new();
    url.push_str("http://");
    url.push_str(addr);
    url.push_str(stop_url);
    let uri: Uri = url.parse().unwrap();
    let mut writer = Vec::new();
    let response = match Request::new(&uri).method(Method::DELETE)
    .header(header_name, name)
    .send(&mut writer) {
        Ok(r) => r,
        Err(err) => {
            println!("err: {:?}", err);
            return Err("send delete request error");
        }
    };
    Ok(())
}

pub fn stop_by_alias<'a>(addr: &'a str, name: &str) -> Result<(), &'a str> {
    let mut url = String::new();
    url.push_str("http://");
    url.push_str(addr);
    url.push_str(stop_by_alias_url);
    let uri: Uri = url.parse().unwrap();
    let mut writer = Vec::new();
    let response = match Request::new(&uri).method(Method::DELETE)
    .header(header_name, name)
    .send(&mut writer) {
        Ok(r) => r,
        Err(err) => {
            println!("err: {:?}", err);
            return Err("send delete request error");
        }
    };
    Ok(())
}
