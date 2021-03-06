use crate::templates::html;
use crate::process::ProcessStatus;
use crate::core::dispatch::CDispatch;

use tiny_http::{Request, Response};

pub struct CIndexHandler {
}

impl CIndexHandler {
    pub fn handler(dispatch: &CDispatch, request: Request) {
        let mut content = String::new();
        content.push_str(html::htmlStartDefine);
        loop {
            let statuses = match dispatch.getAllRunStatus() {
                Ok(s) => s,
                Err(err) => {
                    println!("get all runStatus error, err: {}", err);
                    break;
                }
            };
            for status in statuses {
                match status.status {
                    ProcessStatus::Stoped
                        | ProcessStatus::Unknow => {
                        content.push_str("obj.state = 'stopped';");
                        content.push_str("obj.description = 'unknow';");
                    },
                    ProcessStatus::Failed(desc) => {
                        content.push_str("obj.state = 'failed';");
                        content.push_str("obj.description = 'failed: ");
                        content.push_str(&desc);
                        content.push_str("';");
                    },
                    ProcessStatus::QuickExit => {
                        content.push_str("obj.state = 'quick exit';");
                        content.push_str("obj.description = 'quick exit';");
                    },
                    _ => {
                        content.push_str("obj.state = 'running';");
                        // desc display
                        content.push_str("obj.description = '");
                        // pid
                        content.push_str("pid: ");
                        content.push_str(&status.pid.to_string());
                        content.push_str(", runtime: ");
                        content.push_str(&status.runTime);
                        content.push_str("';");
                    }
                }
                // name display
                content.push_str(&format!("obj.name = '{}';", status.name));
                content.push_str(&format!("obj.alias = '{}';", status.alias));
                // display
                content.push_str("create(obj);");
            }
            break;
        }
        content.push_str(html::htmlEndDefine);
        let response = Response::from_data(content);
        request.respond(response);
    }
}

impl CIndexHandler {
    pub fn new() -> CIndexHandler {
        CIndexHandler{
        }
    }
}
