// use reqwest::Client;
use easy_http_request::DefaultHttpRequest;

const restart_all_url: &str = "/api/restart/all";

pub fn restart_all(addr: &str) -> Result<(), &str> {
    let mut url = String::new();
    url.push_str("http://");
    url.push_str(addr);
    url.push_str(restart_all_url);
    // let client = Client::new();
    let client = match DefaultHttpRequest::put_from_url_str(&url) {
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
            return Err("send put request error");
        }
    };
    Ok(())
}
