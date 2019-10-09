use http_req::{request::Request, request::Method, uri::Uri};
use json_pretty::PrettyFormatter;

const all_process_status_url: &str = "/api/all/process/status";

pub fn all_process_status(addr: &str) -> Result<(), &str> {
    let mut url = String::new();
    url.push_str("http://");
    url.push_str(addr);
    url.push_str(all_process_status_url);
    let uri: Uri = url.parse().unwrap();
    let mut writer = Vec::new();
    let response = match Request::new(&uri).method(Method::GET).send(&mut writer) {
        Ok(r) => r,
        Err(err) => {
            println!("err: {:?}", err);
            return Err("send get request error");
        }
    };
    println!("process status:");
    let f = PrettyFormatter::from_str(&String::from_utf8(writer).unwrap());
    println!("{}", f.pretty());
    Ok(())
}
