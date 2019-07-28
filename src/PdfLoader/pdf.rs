extern crate lopdf;

use std::collections::BTreeMap;
use std::fmt;
use std::io::Read;


use lopdf::content::Content;
use lopdf::{Document, Object, ObjectId};

const SPACE_THRESHOLD: i64 = 100;

#[derive(Default)]
pub struct Collector {
    text: String,
}

impl Collector {
    pub fn process_document<R: Read>(source: R) -> String {
        let document = Document::load_from(source).unwrap();
        let mut result = String::from("");
        let pages = document.get_pages();
		for (page_number, _) in pages {
            let text = document.extract_text(&[page_number]).unwrap();
            result += &format!("{},??", text);
        }
        result
    }
    
}


