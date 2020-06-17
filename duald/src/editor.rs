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
    buffer: String,
    spans: Vec<BoundTup<usize>>,
    cursor: Option<Cursor>,
    // TODO figure out how to prevent browser's contenteditable from working
    html_span_map: HashMap<BoundTup<usize>, BoundTup<usize>>,

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
        Err("Temp error to avoid constructing Self".into())
        // TODO the following lines should replace unimplemented
        // Ok(Self {
        //     buffer: stripped_html.0,
        //     spans: stripped_html.1,
        //     cursor: None,
        //     html_span_map: mapped_html,

        //     ui,
        //     processor,
        // })
    }
}

pub type DefaultEditor = Editor<DefaultUI, DefaultProcessor>;
