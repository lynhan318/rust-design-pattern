//Intent:
//Proxy is a structural design pattern that provides an object that acts
//as a substitute for a real service object used by a client.
//A proxy receives client requests, does some work (access control, caching, etc.) and then passes the request to a service object.

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Data {
    pub size: u64,
    pub content: Vec<u8>,
}

pub trait Download {
    fn get(&mut self, source: String) -> Data;
}

#[derive(Clone)]
pub struct File {
    pub mime_type: String,
}

impl Download for File {
    fn get(&mut self, source: String) -> Data {
        println!("Download content of source {}", source);
        Data {
            size: 20,
            content: vec![1, 2, 3, 4],
        }
    }
}

struct DownloadFileProxy {
    files: HashMap<String, Data>,
}

impl Download for DownloadFileProxy {
    fn get(&mut self, source: String) -> Data {
        match self.files.get(&source) {
            Some(data) => {
                println!("Hit cached {} => {:?}", &source, data);
                data.clone()
            }
            None => {
                let content = File {
                    mime_type: "Text".into(),
                }
                .get(source.clone());
                self.files.insert(source, content.clone());
                content
            }
        }
    }
}
impl DownloadFileProxy {
    fn new() -> Self {
        DownloadFileProxy {
            files: HashMap::new(),
        }
    }
}
pub fn demo_proxy() {
    let mut download_proxy = DownloadFileProxy::new();
    download_proxy.get("nextlint.com".into());
    download_proxy.get("google.com".into());
    download_proxy.get("nextlint.com".into());
}
