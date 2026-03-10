use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use nwg::{taffy::FlexDirection, EventData, KeyPress, RawEventHandler, Setters, DELETE};

/// Fixed point data entry
#[derive(Default)]
pub struct FixedNumEdit {
    pub field: nwg::LabeledEdit,
    precision: usize,
    width: usize,
    dec_pos: usize,
    max_val: f32,
    // last_key: std::cell::Cell<char>,
    last_key: Rc<Cell<char>>,

    handler0: RefCell<Option<RawEventHandler>>,
}

nwg::subclass_control_layout!(
    FixedNumEdit,
    LabeledEdit,
    field,
    FlexboxLayout,
    field.layout
);

const PASTE: char = '\u{0}';
const ASCII_DELETE: char = '\u{7f}';
const ASCII_BACKSPACE: char = '\u{8}';

impl FixedNumEdit {
    pub fn builder<'a>() -> FixedNumEditBuilder<'a> {
        FixedNumEditBuilder::default()
    }

    pub fn parse(&self) -> Result<f32, std::num::ParseFloatError> {
        // fixed size

        let mut selected = self.selection();
        let cursor = self.selection().start as usize;
        // note: selected.start == selected.end so we must depend on text length, cursor position and key pressed
        let mut remangle = false;

        let mut text = self.text();
        text.retain(|c| match c {
            '0'..='9' | '.' => true,
            _ => false,
        });

        if let Some(mut dec_pos) = text.find('.') {
            let dec_pos_last = text.rfind('.').unwrap();
            if dec_pos_last != dec_pos {
                // we have multiple decimal points
                if dec_pos == cursor - 1 || dec_pos_last == cursor - 1 {
                    //  we just typed a decimal
                    dec_pos = cursor - 1; // WLOG

                    let mut rest = text.split_off(dec_pos);
                    text = text.replace(".", "");
                    rest = rest.replace(".", "");

                    let mut new_text = String::with_capacity(self.width);

                    if text.len() > self.dec_pos {
                        // we need to put the decimal where it belongs
                        new_text.push_str(&text[text.len() - self.dec_pos..]);
                    } else {
                        new_text.push_str(&text);
                    }
                    new_text.push('.');
                    new_text.push_str(&rest);
                    text = new_text;

                    selected.start = self.dec_pos as u32 + 1;
                    selected.end = self.dec_pos as u32 + 1;
                } else {
                    // remove them all
                    text = text.replace(".", "");
                    remangle = true;
                }
            } else {
                // only one decimal point
                if text.len() > self.width {
                    //text inserted, not overwritten

                    // move the decimal out of my way
                    if cursor > self.dec_pos && cursor - self.dec_pos <= text.len() - self.width {
                        selected.start += 1;
                        selected.end += 1;
                    }

                    if self.dec_pos < dec_pos {
                        // add to beginning
                        let mut new_text = String::with_capacity(self.width);
                        new_text.push_str(&text[..self.dec_pos]);
                        new_text.push('.');
                        new_text.push_str(&text[self.dec_pos..dec_pos]);
                        new_text.push_str(&text[dec_pos + 1..]);
                        text = new_text;
                    } else if cursor == text.len() {
                        // add to end
                        let start_idx = if self.dec_pos == dec_pos { 1 } else { 0 }; // truncate from front if buffer is full
                        let mut new_text = String::with_capacity(self.width);
                        new_text.push_str(&text[start_idx..dec_pos]);
                        new_text.push_str(&text[dec_pos + 1..dec_pos + 2]);
                        new_text.push('.');
                        new_text.push_str(&text[dec_pos + 2..]);
                        text = new_text;
                    }
                } else if text.len() < self.width && dec_pos < self.dec_pos {
                    if text.len() > self.dec_pos {
                        // we need to put the decimal where it belongs
                        let mut new_text = String::with_capacity(self.width);
                        new_text.push_str(&text[..dec_pos]);
                        new_text.push_str(&text[dec_pos + 1..self.dec_pos + 1]);
                        new_text.push('.');
                        new_text.push_str(&text[self.dec_pos + 1..]);
                        text = new_text;
                    }
                } else {
                    // pasted
                    text.remove(dec_pos);
                    remangle = true;
                }
            }
        } else if text.len() > self.dec_pos {
            // no decimal
            // but enough space

            let mut new_text = String::with_capacity(self.width);
            // move the decimal out of my way
            if cursor != self.dec_pos
                || match self.last_key.get() {
                    ASCII_DELETE => {
                        new_text.push_str(&text[..self.dec_pos]);
                        new_text.push('.');
                        new_text.push_str(&text[self.dec_pos + 1..]);
                        false
                    }
                    ASCII_BACKSPACE => {
                        new_text.push_str(&text[..self.dec_pos - 1]);
                        new_text.push_str(&text[self.dec_pos..self.dec_pos + 1]);
                        new_text.push('.');
                        new_text.push_str(&text[self.dec_pos + 1..]);
                        selected.start -= 1;
                        selected.end -= 1;
                        false
                    }
                    _ => true,
                }
            {
                new_text.push_str(&text[..self.dec_pos]);
                new_text.push('.');
                new_text.push_str(&text[self.dec_pos..]);
            }
            text = new_text;
        } else {
            remangle = true;
        }
        if remangle {
            if text.len() > self.dec_pos {
                // no decimal
                // but enough space for one
                let mut new_text = String::with_capacity(self.width);
                new_text.push_str(&text[..self.dec_pos]);
                new_text.push('.');
                new_text.push_str(&text[self.dec_pos..]);
                text = new_text;
            } else {
                // text is too small for a decimal
                text.reserve_exact(self.dec_pos - text.len());
                text.push_str(&"0".repeat(self.dec_pos - text.len()));
            }
        }

        text.truncate(self.width);

        let mut value: f32 = text.parse()?;

        let width = self.width;
        self.set_text(&format!("{:0>width$.*}", self.precision, value));

        self.set_selection(selected);
        Ok(value)
    }

    //
    fn bind_raw_handler(&self) {
        use winapi::um::winuser::{WM_CHAR, WM_KEYDOWN, WM_PASTE};
        let handler_id = 0x10000; // handler ids equal or smaller than 0xFFFF are reserved by NWG
        let callback_last_key = self.last_key.clone();
        let handler = nwg::bind_raw_event_handler(
            &self.handle,
            handler_id,
            move |hwnd, msg, w, l| match msg {
                WM_KEYDOWN => {
                    match w as u32 {
                        DELETE => callback_last_key.set(ASCII_DELETE),
                        _ => {}
                    }
                    None
                }
                WM_CHAR => match char::from_u32(w as u32).unwrap_or('?') {
                    key @ ('0'..='9' | '.' | ASCII_BACKSPACE) => {
                        // allowed characters
                        callback_last_key.set(key);
                        None
                    }
                    _ => Some(0),
                },
                WM_PASTE => {
                    // so we aren't anticipating keypresses
                    callback_last_key.set(PASTE);
                    None
                }
                _ => None,
            },
        );
        *self.handler0.borrow_mut() = Some(handler.unwrap());
    }
    // pub fn set_background_color(&self, color: Option<[u8; 3]>) {
    //     self.field
    // }
}

#[derive(Setters)]
pub struct FixedNumEditBuilder<'a> {
    #[setter(skip)]
    control_builder: nwg::LabeledEditBuilder<'a>,
    /// places after decimal
    precision: usize,
    /// places before decimal
    places: usize,
}

impl<'a> Default for FixedNumEditBuilder<'a> {
    fn default() -> Self {
        Self {
            control_builder: nwg::LabeledEdit::builder(),
            precision: 1,
            places: 1,
        }
    }
}

impl<'a> FixedNumEditBuilder<'a> {
    pub fn background_color(
        mut self,
        background_color: Option<[u8; 3]>,
    ) -> FixedNumEditBuilder<'a> {
        self.control_builder = self.control_builder.background_color(background_color);
        self
    }
    pub fn label(mut self, label: &'a str) -> FixedNumEditBuilder<'a> {
        self.control_builder = self.control_builder.label(label);
        self
    }

    pub fn parent<C: Into<nwg::ControlHandle>>(mut self, p: C) -> FixedNumEditBuilder<'a> {
        self.control_builder = self.control_builder.parent(p);
        self
    }

    pub fn build(self, control: &mut FixedNumEdit) -> Result<(), nwg::NwgError> {
        control.precision = self.precision;
        control.dec_pos = self.places;
        control.max_val = 10u32.pow(self.places as u32) as f32;
        control.width = self.places as usize + self.precision + 1;
        control.last_key = Rc::new(Cell::default());

        let mut text = String::with_capacity(control.width);
        text.push_str(&"0".repeat(self.places));
        text.push('.');
        text.push_str(&"0".repeat(self.precision));
        self.control_builder.text(&text).build(&mut control.field)?;
        control.bind_raw_handler();
        Ok(())
    }
}

impl Drop for FixedNumEdit {
    fn drop(&mut self) {
        let handler = self.handler0.borrow();
        if let Some(h) = handler.as_ref() {
            drop(nwg::unbind_raw_event_handler(h));
        }
    }
}
