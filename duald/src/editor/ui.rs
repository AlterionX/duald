use std::{
    ops::{Bound, RangeBounds},
    collections::HashMap,
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, HtmlElement, Range};
use tap::*;

pub trait EditorUI: Sized {
    fn init(editor: HtmlElement) -> Result<Self, JsValue>;
}

pub const CONTENT_CSS_CLASS: &'static str = "duald_editor_content";

pub struct DefaultUI;

impl EditorUI for DefaultUI {
    // TODO replace all editor styling with a style sheet
    fn init(editor: HtmlElement) -> Result<Self, JsValue> {
        // Make editor visible
        editor.set_content_editable("false");
        let editor_style = editor.style(); // Cache to avoid JS context swtich.
        editor_style.set_property("width", "70%")?;
        editor_style.set_property("border", "10px solid black")?;
        editor_style.set_property("flex-direction: column; display: flex;", "10px solid black")?;

        let doc = editor.owner_document().ok_or("Expected editor html element to have an owning document.")?;

        // Wrap content.
        let header: HtmlElement = doc.create_element("div")?.dyn_into()?; // TODO maybe this should be header?
        let content: HtmlElement = doc.create_element("div")?.dyn_into()?;
        while let Some(child) = editor.first_child() {
            editor.remove_child(&child)?;
            content.append_child(&child)?;
        }
        editor.append_child(&header)?;
        editor.append_child(&content)?;

        content.set_content_editable("true");
        content.set_class_name(CONTENT_CSS_CLASS);
        let content_style = content.style(); // Cache to avoid JS context switch.
        content_style.set_property("flex", "1")?;
        content_style.set_property("border", "5px solid red")?;

        Ok(Self {})
    }
}