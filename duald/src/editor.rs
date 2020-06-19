use std::{
    ops::Bound,
    collections::HashMap,
};
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;
use tap::*;

mod ui;
mod processor;

use ui::DefaultUI;
pub use ui::EditorUI;

use processor::DefaultProcessor;
pub use processor::TextProcessor;

type BoundTup<T> = (Bound<T>, Bound<T>);

#[derive(Debug)]
enum Cursor {
    Select(BoundTup<usize>),
    Insert(usize),
    // TODO Overwrite,
}

pub struct Editor<UI: EditorUI, Processor: TextProcessor> {
    ui: UI,
    processor: Processor,
}

pub const PRIMARY_CSS_CLASS: &'static str = "duald_editor";

impl<UI: EditorUI, Processor: TextProcessor> Editor<UI, Processor> {
    pub fn init(editor: HtmlElement) -> Result<Self, JsValue> {
        log::info!("Attaching editor to element: {}", editor.outer_html());
        let doc = editor.owner_document()
            .ok_or_else(|| "Requested editor does not have an owning document. There is no way attach action listeners.")?;

        // Attach listeners here.
        let processor = Processor::attach(editor.clone())?;
        // Initialize UI. (Perhaps remove contenteditable on the outer div, add a toolbar, and create an inner div that has contenteditable.)
        let ui = UI::init(editor.clone())?;
        // TODO create actual editor
        Ok(Self {
            ui,
            processor,
        })
    }
}

pub type DefaultEditor = Editor<DefaultUI, DefaultProcessor>;
