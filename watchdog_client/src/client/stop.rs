// use reqwest::Client;
use easy_http_request::DefaultHttpRequest;

const stop_all_url: &str = "/api/stop/all";

pub fn stop_all(addr: &str) -> Result<(), &str> {
    let mut url = String::new();
    url.push_str("http://");
    url.push_str(addr);
    url.push_str(stop_all_url);
    let client = match DefaultHttpRequest::delete_from_url_str(&url) {
        Ok(c) => c,
        Err(err) => {
            println!("err: {:?}", err);
            return Err("craete client error");
        }
    };
    let response = match client.send() {
        Ok(r) => r,
        Err(err) => {
            println!("err: {:?}", err);
            return Err("send delete request error");
        }
    };
    println!("{:?}", response);
    Ok(())
}
