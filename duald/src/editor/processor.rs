use std::{
    ops::{Bound, RangeBounds},
};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{HtmlElement, EventTarget};
use tap::*;

use crate::editor::Cursor;

pub trait TextProcessor: Sized {
    fn attach(editor: HtmlElement) -> Result<Self, JsValue>;
}

const CURSOR_CHANGE_EVENTS: [&'static str; 4] = [
    "mousedown",
    "mouseup",
    "keydown",
    "keyup",
];

pub struct DefaultProcessor {
    listener: Closure<Fn()>,
}

impl TextProcessor for DefaultProcessor {
    fn attach(editor: HtmlElement) -> Result<Self, JsValue> {
        // TODO Temporary thing for testing out the cursor location.
        let root = editor.owner_document().ok_or("Editor element does not have a owning document!")?;
        let event_target: EventTarget = root.into();
        let editor_ptr = editor.clone();
        let callback = Closure::wrap(Box::new(move || log::info!("A call back! Selected text: {:?}", Self::find_cursor(editor_ptr.clone()))) as Box<dyn Fn()>);
        for event in CURSOR_CHANGE_EVENTS.iter() {
            event_target.add_event_listener_with_callback(event, callback.as_ref().unchecked_ref())?;
        }

        Ok(Self {
            listener: callback,
        })
    }
}

impl DefaultProcessor {
    // TODO Consider moving this to the UI
    // TODO figure out how to count new lines (in Chrome)
    // TODO make sure this works in every browser (other than IE)
    // Source: https://stackoverflow.com/questions/4811822/get-a-ranges-start-and-end-offsets-relative-to-its-parent-container/4812022#4812022
    fn find_cursor(editor: HtmlElement) -> Option<Cursor> {
        // The reference code also has a `document`, but that's only in IE.
        let doc = editor.owner_document()
            .tap_none(|| log::warn!("Editor has no owning document."))?;
        // The `default_view` is called `parent_window` only in IE.
        let win = doc.default_view()
            .tap_none(|| log::warn!("Document has now visable window."))?;

        // Locate selection
        let sel = match win.get_selection() {
            Ok(sel @ Some(_))  => sel,
            Err(_) | Ok(None) => {
                log::warn!("Failed to get selection from window. Retrying with document.");
                let sel = match doc.get_selection() {
                    Ok(sel) => sel,
                    Err(e) => {
                        log::error!("The selection could not be detected on document either due to {:?}.", e);
                        None
                    },
                };
                sel
            },
        }?;

        // Convert Selection to Cursor
        if sel.range_count() > 0 {
            let sel_range = sel.get_range_at(0).ok()?;
            let ele_range = {
                // range is a pointer type, so we need to call the special function
                let range = sel_range.clone_range();
                range.select_node_contents(&editor)
                    .ok()?;
                range
            };

            let sel_end = {
                let sel_end = (sel_range.end_container(), sel_range.end_offset());
                let container = sel_end.0
                    .tap_err(|e| log::error!("Failed to get end container due to {:?}.", e))
                    .ok()?;
                let offset = sel_end.1
                    .tap_err(|e| log::error!("Failed to get end offset due to {:?}.", e))
                    .ok()?;
                (container, offset)
            };
            log::info!("End offset at {:?}.", sel_end.1);
            let end_offset = {
                ele_range.set_end(&sel_end.0, sel_end.1)
                    .tap_err(|e| log::error!("Failed to set range end selection end due to {:?}.", e))
                    .ok()?;
                ele_range.to_string().length()
            } as usize;
            
            if sel.is_collapsed() {
                Some(Cursor::Insert(end_offset))
            } else {
                let sel_start = {
                    let sel_start = (sel_range.start_container(), sel_range.start_offset());
                    let container = sel_start.0
                        .tap_err(|e| log::error!("Failed to get start container due to {:?}.", e))
                        .ok()?;
                    let offset = sel_start.1
                        .tap_err(|e| log::error!("Failed to get start offset due to {:?}.", e))
                        .ok()?;
                    (container, offset)
                };
                log::info!("Start offset at {:?}.", sel_start.1);
                let start_offset = {
                    ele_range.set_end(&sel_start.0, sel_start.1)
                        .tap_err(|e| log::error!("Failed to set range end to selection start due to {:?}.", e))
                        .ok()?;
                    ele_range.to_string().length()
                } as usize;
                Some(Cursor::Select((
                    Bound::Included(start_offset),
                    Bound::Included(end_offset),
                )))
            }
        } else {
            None
        }
    }
}