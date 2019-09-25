use reqwest::Client;

const restart_all_url: &str = "/api/restart/all";

pub fn restart_all(addr: &str) -> Result<(), &str> {
    let mut url = String::new();
    url.push_str("http://");
    url.push_str(addr);
    url.push_str(restart_all_url);
    let client = Client::new();
    let response = match client.put(&url).send() {
        Ok(r) => r,
        Err(err) => {
            println!("err: {}", err);
            return Err("send put request error");
        }
    };
    Ok(())
}
