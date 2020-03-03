package client

import (
    "net/http"
    "io/ioutil"
    "log"
)

const (
    restart_all_url = "/api/restart/all"
)

func restart_all(addr string) {
    req, err := http.NewRequest("PUT", url, nil)
    if err != nil {
        log.Fatalln("net request error, err:", err)
    }
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
