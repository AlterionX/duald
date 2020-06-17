use std::{
    ops::{Bound, RangeBounds},
};
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;
use tap::*;

use crate::editor::Cursor;

pub trait TextProcessor: Sized {
    fn attach(editor: HtmlElement) -> Result<Self, JsValue>;
}

pub struct DefaultProcessor;

impl TextProcessor for DefaultProcessor {
    fn attach(_: HtmlElement) -> Result<Self, JsValue> {
        Ok(Self)
    }
}

impl DefaultProcessor {
    // Source: https://stackoverflow.com/questions/4811822/get-a-ranges-start-and-end-offsets-relative-to-its-parent-container/4812022#4812022
    fn find_cursor(editor: HtmlElement) -> Option<Cursor> {
        // The reference code also has a `document`, but that's only in IE.
        let doc = editor.owner_document()?;
        // The `default_view` is called `parent_window` only in IE.
        let win = doc.default_view()?;

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
                let range = sel_range.clone();
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