use web_sys::HtmlInputElement;
use yew::NodeRef;

use crate::seq_gen;

pub struct Shuffle {
    pub sequence: Vec<String>,
    pub length: u64,
    pub error: String,
    pub node_ref: NodeRef,
}

impl Default for Shuffle {
    fn default() -> Self {
        Self {
            sequence: seq_gen::shuffler(25),
            length: 25,
            error: String::new(),
            node_ref: NodeRef::default(),
        }
    }
}

impl Shuffle {
    pub fn generate_shuffle(&mut self) {
        let length_ref = &self.node_ref;
        let length_value = length_ref.cast::<HtmlInputElement>().unwrap().value();

        match length_value.parse::<u64>() {
            Ok(r) => {
                self.error.clear();
                self.length = r;
            },
            Err(e) => {
                if length_value.is_empty() {
                    self.length = 25;
                } else {
                    self.error = format!("Invalid input: {}", e);
                }
            }
        }

        self.sequence = seq_gen::shuffler(self.length);
    }
}