use reqwest::Client;

const stop_all_url: &str = "/api/stop/all";

pub fn stop_all(addr: &str) -> Result<(), &str> {
    let mut url = String::new();
    url.push_str("http://");
    url.push_str(addr);
    url.push_str(stop_all_url);
    let client = Client::new();
    let response = match client.delete(&url).send() {
        Ok(r) => r,
        Err(err) => {
            println!("err: {}", err);
            return Err("send delete request error");
        }
    };
    println!("{:?}", response);
    Ok(())
}
