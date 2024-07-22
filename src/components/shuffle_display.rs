//! Handles shuffle generation

use gloo::utils::document;
use web_sys::{HtmlInputElement, Node};
use yew::{function_component, Html, NodeRef, Properties};

use crate::{preferences, seq_gen};

pub struct Shuffle {
    pub sequence: Vec<String>,
    pub length: u64,
    pub error: String,
    pub node_ref: NodeRef,
}

impl Default for Shuffle {
    fn default() -> Self {
        let length = preferences::get_length();
        
        Self {
            sequence: seq_gen::shuffler(length),
            length,
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
                preferences::set_length(r);
            },
            Err(e) => {
                if length_value.is_empty() {
                    self.length = preferences::get_length();
                } else {
                    self.error = format!("Invalid input: {}", e);
                }
            }
        }

        self.sequence = seq_gen::shuffler(self.length);
    }
}

#[derive(Properties, PartialEq)]
pub struct ShuffleDisplayProps {
    pub shuffle: Vec<Vec<String>>,
    pub dark: bool,
}

#[function_component]
pub fn ShuffleDisplay(props: &ShuffleDisplayProps) -> Html {
    let shuffle = &props.shuffle;

    let node = {
        let table = document().create_element("table").unwrap();
        table.class_list().add_1("shuffle-table").unwrap();

        if props.dark {
            table.class_list().add_1("dark").unwrap();
        }

        let table_body = document().create_element("tbody").unwrap();

        shuffle.iter().for_each(|row_data| {
            let row = document().create_element("tr").unwrap();

            row_data.iter().for_each(|cell_data| {
                let cell = document().create_element("td").unwrap();
                
                cell.class_list().add_1("shuffle-cell").unwrap();

                if props.dark {
                    cell.class_list().add_1("dark").unwrap();
                }

                cell.append_child(&document().create_text_node(cell_data))
                    .unwrap();
                row.append_child(&cell).unwrap();
            });

            table_body.append_child(&row).unwrap();
        });

        table.append_child(&table_body).unwrap();

        let node: Node = table.into();
        Html::VRef(node)
    };

    node
}