extern crate base64;
extern crate tempfile;
extern crate lopdf;

use wasm_bindgen::prelude::*;
use web_sys::console;

use base64::{encode, decode};
use lopdf::content::Content;
use lopdf::{Document, Object, ObjectId};
use tempfile::tempfile;
use std::fs::File;
use std::io::{Write, Read, BufReader, Cursor};

// ¯\_(ツ)_/¯
const SPACE_THRESHOLD: i64 = 100;



// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen]
pub fn pdf_check(base64_str: String) -> String {
  let mut buffer = Vec::<u8>::new();
  base64::decode_config_buf(&base64_str, base64::STANDARD, &mut buffer).unwrap();
  format!("{:?}", buf)
  let doc = Document::load_from(buffer.as_slice()).unwrap();
  doc.version
}
