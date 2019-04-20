extern crate sysinfo;

use tower_web::*;

use sysinfo::{ProcessExt, SystemExt, System};

use tower_web::ServiceBuilder;
use tower_web::view::Handlebars;
use tokio::prelude::*;

use std::process::Command;
use std::default::Default;

use super::super::config::Process;

#[derive(Debug, Response)]
struct ProcessResponse {
    processList: String,
}

#[derive(Clone, Debug)]
pub struct ProcessHandler {
    processes: Vec<Process>
}

impl_web! {
    impl ProcessHandler {
        #[get("/")]
        #[content_type("html")]
        #[web(template = "process_handler")]
        fn process_handler(&self) -> Result<ProcessResponse, ()> {
            let mut content = String::new();
            let system = System::new();
            for item in &self.processes {
                // state display
                if system.get_process_by_name(&item.name).len() == 0 {
                    content.push_str("obj.state = 'stopped';");
                } else {
                    content.push_str("obj.state = 'running';");
                }
                // name display
                content.push_str(&format!("obj.name = '{}';", item.name));
                // display
                content.push_str("create(obj);");
            }
            Ok(ProcessResponse {
                processList: content,
            })
        }
    }
}

pub struct CServer {
    processes: Vec<Process>
}

impl CServer {
    pub fn start(self, host: &str, port: u32) {
        let mut server = String::new();
        server.push_str(host);
        server.push_str(":");
        server.push_str(&port.to_string());

        let addr = server.parse().expect("Invalid address");
        println!("Listening on http://{}", addr);

        ServiceBuilder::new()
            .resource(ProcessHandler {
                processes: self.processes
            })
            .serializer(Handlebars::new())
            .run(&addr)
            .unwrap();
    }

    pub fn new(processes: Vec<Process>) -> CServer {
        let server = CServer{
            processes: processes
        };
        server
    }
}
