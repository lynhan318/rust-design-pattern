// CONCEPTUAL EXAMPLE: Nginx Proxy

// A web server such as Nginx can act as a proxy for your application server:
//     It provides controlled access to your application server.
//     It can do rate limiting.
//     It can do request caching.
//  REFERENCE: https://refactoring.guru/design-patterns/proxy/rust/example

use std::collections::HashMap;

pub trait Server {
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String);
}

pub struct Application;
impl Server for Application {
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
        if url == "/app/status" && method == "GET" {
            return (200, "Ok".into());
        }
        if url == "/create/user" && method == "POST" {
            return (200, "User Created".into());
        }
        (404, "Not found".into())
    }
}

pub struct NginxServer {
    application: Application,
    max_allowed_requests: u32,
    rate_limiter: HashMap<String, u32>,
}
impl NginxServer {
    pub fn new() -> Self {
        Self {
            application: Application,
            max_allowed_requests: 2,
            rate_limiter: HashMap::default(),
        }
    }
    pub fn check_rate_limiting(&mut self, url: &str) -> bool {
        let rate = self.rate_limiter.entry(url.to_string()).or_insert(1);
        if *rate > self.max_allowed_requests {
            return false;
        }
        *rate += 1;
        true
    }
}
impl Server for NginxServer {
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
        if !self.check_rate_limiting(url) {
            return (403, "Not Allowed".into());
        }
        self.application.handle_request(url, method)
    }
}

pub fn demo() {
    let mut nginx_server = NginxServer::new();

    let app_status = "/app/status";
    let create_user = "/create/user";

    let (code, body) = nginx_server.handle_request(app_status, "GET");
    println!("Url:{}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = nginx_server.handle_request(app_status, "GET");
    println!("Url:{}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = nginx_server.handle_request(app_status, "GET");
    println!("Url:{}\nHttpCode: {}\nBody: {}\n", app_status, code, body);

    let (code, body) = nginx_server.handle_request(create_user, "GET");
    println!("Url:{}\nHttpCode: {}\nBody: {}\n", create_user, code, body);
}
