extern crate base64;
extern crate lopdf;

use wasm_bindgen::prelude::*;
use web_sys::console;

use base64::{encode, decode};
use lopdf::content::Content;
use lopdf::{Document, Object, ObjectId};
use std::fs::File;
use std::io::{Write, Read, Seek, SeekFrom};

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
  let buf = &decode(&base64_str).unwrap()[..];
  let document = Document::load_from(buf).unwrap();
  let pages = document.get_pages();
  document.version
}
