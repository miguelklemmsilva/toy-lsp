use std::collections::HashMap;

use log::warn;

use super::hover::{HoverParams, HoverResponse};

pub struct State {
    documents: HashMap<String, String>,
}

impl State {
    pub fn new() -> Self {
        State {
            documents: HashMap::new(),
        }
    }

    pub fn open_document(&mut self, uri: String, contents: String) {
        self.documents.insert(uri, contents);
    }

    pub fn update_document(&mut self, uri: String, contents: String) {
        self.documents.insert(uri, contents);
    }

    pub fn hover(&self, hover_params: HoverParams) -> HoverResponse {
        let uri =hover_params.text_document.uri;
        let document_result = self.documents.get(&uri);
        let document = match document_result {
            Some(document) => document,
            None => {
                warn!("Did not find uri: {}", uri);
                return HoverResponse::new(String::new());
            }
        };
        HoverResponse::new(format!("heyyy girlyyy 😜, so here's the tea: {}, {} - Andrea", uri, document.len()))
    }
}
