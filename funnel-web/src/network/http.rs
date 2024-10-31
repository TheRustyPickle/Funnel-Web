use ehttp::{Headers, Request};
use std::sync::{Arc, Mutex};

use crate::core::RequestStatus;

const SERVER_URL: &str = "https://127.0.0.1:8081/code";

pub fn get_ws_password(password: String, status: Arc<Mutex<RequestStatus>>) {
    let new_header = Headers::new(&[("X-Secret-Code", &password)]);
    let request = Request {
        headers: new_header,
        ..Request::post(SERVER_URL, Vec::new())
    };
    *status.lock().unwrap() = RequestStatus::Pending;
    ehttp::fetch(request, move |response| match response {
        Ok(resp) => {
            if resp.ok {
                let text = resp.text().unwrap();
                *status.lock().unwrap() = RequestStatus::Gotten(text.to_string());
            } else {
                *status.lock().unwrap() =
                    RequestStatus::Failed(String::from("Failed to get ws password"));
            }
        }
        Err(e) => {
            *status.lock().unwrap() = RequestStatus::Failed(e);
        }
    });
}
