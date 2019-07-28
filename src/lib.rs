extern crate base64;
extern crate tempfile;
extern crate lopdf;

mod PdfLoader;

use wasm_bindgen::prelude::*;
use web_sys::console;

use base64::{encode, decode};
use PdfLoader::pdf::Collector;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn pdf_check(base64_str: String) -> String {
  let mut buffer = Vec::<u8>::new();
  base64::decode_config_buf(&base64_str, base64::STANDARD, &mut buffer).unwrap();
  Collector::process_document(buffer.as_slice())
}
