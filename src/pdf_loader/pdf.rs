extern crate lopdf;

use std::io::Read;


use lopdf::*;
use pdf_extract::*;

pub struct Collector {}

impl Collector {
    pub fn process_document<R: Read>(source: R) -> String {
        let doc = Document::load_from(source).unwrap();
        // print_metadata(&doc);
        extract_text(&doc).unwrap()
        // String::from("test")
    }
}
