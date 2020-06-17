use wasm_bindgen::{prelude::wasm_bindgen, JsValue, JsCast};
use web_sys::Document;
use tap::*;

mod editor;
mod logger;

#[cfg(debug_assertions)]
pub use logger::realtime_log_change;

use editor::{Editor, EditorUI, TextProcessor, DefaultEditor};

#[wasm_bindgen]
pub fn attach_default_editor_to(id: &str, doc: Option<Document>) -> Result<(), JsValue> {
    let editor: DefaultEditor = attach_to(id, doc)?;
    Ok(())
}

fn attach_to<UI: EditorUI, Processor: TextProcessor>(id: &str, doc: Option<Document>) -> Result<Editor<UI, Processor>, JsValue> {
    let elem = doc
        .tap_none(|| log::info!("No document provided. Attempting to use default document."))
        .or_else(|| web_sys::window()?.document())
        .ok_or_else(|| "No default document found. Duald is likely not running in a web browser environment.".to_string())
        .tap_err(|e| log::error!("Error, editor failed to init: {}", e))?
        .query_selector(format!("#{}.{}", id, editor::PRIMARY_CSS_CLASS).as_str())
        .tap_err(|e| log::error!("Error, editor failed to init: {:?}", e))?
        .ok_or_else(|| format!("Failed to located targeted editor element. Note that it is also required to give the targeted element the `{}` CSS class.", editor::PRIMARY_CSS_CLASS))
        .tap_err(|e| log::error!("Error, editor failed to init: {}", e))?
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| "Located element is not an html element.")
        .tap_err(|e| log::error!("Error, editor failed to init: {}", e))?;
    // TODO do something with the editor
    Editor::<UI, Processor>::init(elem)
}

// Note this function can't be named `init`, as `init` already exists in the wasm bindgen generated JS glue. Bug?
#[wasm_bindgen(start)]
pub fn setup() {
    logger::setup_logger();
    log::info!("Duald loaded.");
}
