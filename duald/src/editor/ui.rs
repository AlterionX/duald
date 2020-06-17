use std::{
    ops::{Bound, RangeBounds},
    collections::HashMap,
};
use wasm_bindgen::JsValue;
use web_sys::{HtmlElement, Range};
use tap::*;

pub trait EditorUI: Sized {
    fn init(editor: HtmlElement) -> Result<Self, JsValue>;
}

pub const CONTENT_CSS_CLASS: &'static str = "duald_editor_content";

pub struct DefaultUI;

impl EditorUI for DefaultUI {
    fn init(_: HtmlElement) -> Result<Self, JsValue> {
        Ok(Self {})
    }
}