// #![windows_subsystem = "windows"]
#![allow(unused_imports)]
#![allow(unused)]
// #![allow(dead_code)]
// #![warn(unused_imports)]
// #![warn(unused)]

/*!
 *   A very simple application that shows your name in a message box.
 *   Unlike `basic_d`, this example uses layout to position the controls in the window
 */

extern crate native_windows_derive as nwd;

mod clock;
use clock::ClockBox;

use std::cell::RefCell;
use std::cmp::min;
use std::io::Write;
use std::rc::Rc;
// use std::time;

// use chrono::{DateTime, Local, TimeDelta, Utc,seconds};
use chrono::{DateTime, TimeDelta, Utc};
use nwd::{NwgPartial, NwgUi};
use nwg::{CharEffects, CharFormat, EventData, NativeUi};

// Flexbox style
use nwg::taffy::{
    geometry::{Rect, Size},
    style::{Dimension as D, FlexDirection, FlexWrap},
};

const PC_10: D = D::percent(0.1);
const PC_25: D = D::percent(0.25);
const PC_50: D = D::percent(0.5);
const PC_33: D = D::percent(0.33);
const PC_67: D = D::percent(0.67);
const PC_100: D = D::percent(1.0);

const PT_10: D = D::length(10.0);
const PT_5: D = D::length(5.0);
const PADDING: Rect<D> = Rect {
    left: PT_10,
    right: PT_10,
    top: PT_10,
    bottom: PT_10,
};
const MARGIN: Rect<D> = Rect {
    left: PT_5,
    right: PT_5,
    top: PT_5,
    bottom: PT_5,
};

const LAEBL_SIZE: Size<D> = Size {
    // width: D::auto(),
    width: D::percent(0.13),
    height: D::auto(),
    // height: PC_33,
};

const SD_SIZE: Size<D> = Size {
    // width: D::auto(),
    width: D::percent(0.2),
    height: D::auto(),
    // height: PC_33,
};

const ROW_1_3: Size<D> = Size {
    width: D::auto(),
    height: PC_33,
};
const ROW_2_3: Size<D> = Size {
    width: D::auto(),
    height: PC_67,
};

const COL_1_3: Size<D> = Size {
    width: PC_33,
    height: D::auto(),
};
const COL_2_3: Size<D> = Size {
    width: PC_67,
    height: D::auto(),
};

const ROW_10: Size<D> = Size {
    width: D::auto(),
    height: PC_10,
};

const ROW_25: Size<D> = Size {
    width: D::auto(),
    height: PC_25,
};

const ROW_50: Size<D> = Size {
    width: D::auto(),
    height: PC_50,
};

const COL_10: Size<D> = Size {
    width: PC_10,
    height: D::auto(),
};

// #[derive(Default, NwgPartial)]
#[derive(Default, NwgUi)]
pub struct QCApp {
    // Window and layout
    // #[nwg_control(size: (300, 115), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_control(size: (1400, 800), position: (300, 300), title: "QC Data Entry", flags: "MAIN_WINDOW|VISIBLE")]
    #[nwg_events( OnInit: [QCApp::setup], OnWindowClose: [QCApp::say_goodbye] )]
    window: nwg::Window,

    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Column, padding: PADDING,    flex_wrap:FlexWrap::Wrap)]
    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Column, padding: PADDING,    flex_wrap:FlexWrap::NoWrap)]
    // #[nwg_layout(parent: window,)]
    window_layout: nwg::FlexboxLayout,
    // window_layout: nwg::DynLayout, // ToDO
    //
    #[nwg_layout(parent: window, flex_direction: FlexDirection::Row)]
    // #[nwg_layout_item(layout: window_layout)]
    // #[nwg_layout_item(layout: window_layout, size:COL_1_3)] TODO
    #[nwg_layout_item(layout: window_layout, size:ROW_10)]
    toolbar_button_layout: nwg::FlexboxLayout,

    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Row, padding: PADDING, flex_wrap:FlexWrap::Wrap)]
    #[nwg_layout(parent: window, flex_direction: FlexDirection::Row)]
    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Row, padding: PADDING, flex_wrap:FlexWrap::NoWrap)]
    // #[nwg_partial(parent: window_layout,    )]
    // #[nwg_layout_item(layout: window_layout,
    // // size:Size {
    // //     width: D::auto(),
    // //
    // //     // width: D::length(200.0),
    // //                   height: D::length(200.0),
    // // },
    // )]
    #[nwg_layout_item(layout: window_layout, size:ROW_25)]
    top_layout: nwg::FlexboxLayout,

    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Column, align_items: stretch::style::AlignItems::FlexStart)]
    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    #[nwg_layout_item(layout: top_layout, size:LAEBL_SIZE)]
    label_0_layout: nwg::FlexboxLayout,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    #[nwg_layout_item(layout: top_layout, size:SD_SIZE)]
    field_0_layout: nwg::FlexboxLayout,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    #[nwg_layout_item(layout: top_layout, size:LAEBL_SIZE)]
    label_1_layout: nwg::FlexboxLayout,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    #[nwg_layout_item(layout: top_layout, size:SD_SIZE)]
    field_1_layout: nwg::FlexboxLayout,
    /*
    #[nwg_layout(parent: window, flex_direction: FlexDirection::Row)]
    // #[nwg_layout_item(layout: window_layout)]
    // #[nwg_layout_item(layout: window_layout, size:COL_1_3)] TODO
    #[nwg_layout_item(layout: window_layout, size:ROW_10)]
    toolbar_button_layout: nwg::FlexboxLayout,*/

    // TODO clean
    /*
     product_text := "Product"
     sample_text := "Sample Point" // TODO
     customer_text := "Customer Name"
     tester_text := "Tester"

     ranges_text := "Ranges"
     inventory_text := "Inventory"
     reprint_text := "Reprint"
     inbound_text := "Inbound"
     sample_button_text := "Sample"
     whups_text := "Whups"

     release_button_text := "Release"
     today_button_text := "Today"

     inbound_lot_text := "Inbound Lot"
     internal_text := "Internal"
     container_text := "Container"

    */
    // // Controls
    #[nwg_control(text: "Product", v_align: VTextAlign::Top)]
    #[nwg_layout_item(layout: label_0_layout)]
    product_name_label: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: field_0_layout)]
    product_name_field: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "Customer Name", v_align: VTextAlign::Top)]
    #[nwg_layout_item(layout: label_1_layout)]
    customer_name_label: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: field_1_layout)]
    customer_name_field: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "Lot Number", v_align: VTextAlign::Top)]
    #[nwg_layout_item(layout: label_0_layout)]
    lot_name_label: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: field_0_layout)]
    lot_name_field: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "Sample Point", v_align: VTextAlign::Top)]
    #[nwg_layout_item(layout: label_1_layout)]
    sample_name_label: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: field_1_layout)]
    sample_name_field: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "Tester", v_align: VTextAlign::Top)]
    #[nwg_layout_item(layout: label_0_layout)]
    tester_name_label: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: field_0_layout)]
    tester_name_field: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "", v_align: VTextAlign::Top)]
    #[nwg_layout_item(layout: label_1_layout)]
    blank_name_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: field_1_layout)]
    blank_name_field: nwg::Label,

    #[nwg_control]
    #[nwg_events( OnResize:[QCApp::resize_clock_box])]
    #[nwg_layout_item(layout: top_layout, size:COL_1_3)]
    clock_frame: nwg::Frame,
    #[nwg_partial(parent: clock_frame)]
    clock_box: ClockBox,

    #[nwg_control(text: "Ranges")]
    #[nwg_layout_item(layout: toolbar_button_layout,    )]
    #[nwg_events( OnButtonClick:[QCApp::do_clcik])]
    hello_button: nwg::Button,

    #[nwg_control(text: "Inventory")]
    #[nwg_layout_item(layout: toolbar_button_layout)]
    button_inventory: nwg::Button,

    #[nwg_control(text: "Reprint")]
    #[nwg_layout_item(layout: toolbar_button_layout)]
    button_reprint: nwg::Button,

    #[nwg_control(text: "Inbound")]
    #[nwg_layout_item(layout: toolbar_button_layout)]
    button_inbound: nwg::Button,

    #[nwg_control(text: "Whups")]
    #[nwg_layout_item(layout: toolbar_button_layout)]
    button_whups: nwg::Button,

    #[nwg_control]
    // #[nwg_layout_item(layout: window_layout)]
    #[nwg_layout_item(layout: window_layout, size:ROW_50)] //
    tabs_container: nwg::TabsContainer,

    #[nwg_control(text: "Water Based")]
    // #[nwg_layout_item(layout: window_layout)]
    // #[nwg_layout_item(layout: window_layout, size:ROW_50)] //
    panel_wb: nwg::Tab,

    #[nwg_control(text: "Oil Based" )]
    // #[nwg_layout_item(layout: window_layout)]
    // #[nwg_layout_item(layout: window_layout, size:ROW_50)] //
    panel_ob: nwg::Tab,

    #[nwg_control(text: "Friction Reducer" )]
    // #[nwg_layout_item(layout: window_layout)]
    // #[nwg_layout_item(layout: window_layout, size:ROW_50)] //
    panel_fr: nwg::Tab,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Row)]
    // #[nwg_layout_item(layout: window_layout)]
    // #[nwg_layout_item(layout: window_layout, size:COL_1_3)] TODO
    #[nwg_layout_item(layout: window_layout, size:ROW_10)]
    action_button_layout: nwg::FlexboxLayout,

    // #[nwg_control(text: "Submit")]
    // #[nwg_layout_item(layout: action_button_layout)]
    // submit_button: nwg::Button,
    //
    // #[nwg_control(text: "Clear")]
    // #[nwg_layout_item(layout: action_button_layout)]
    // clear_button: nwg::Button,
    //
    // #[nwg_control(text: "Log")]
    // #[nwg_layout_item(layout: action_button_layout)]
    // log_button: nwg::Button,
    #[nwg_control(parent: window, text: "Submit")]
    #[nwg_layout_item(layout: action_button_layout)]
    submit_button: nwg::Button,

    #[nwg_control(parent: window, text: "Clear")]
    #[nwg_layout_item(layout: action_button_layout)]
    clear_button: nwg::Button,

    #[nwg_control(parent: window, text: "Log")]
    #[nwg_layout_item(layout: action_button_layout)]
    log_button: nwg::Button,
    // #[nwg_control(popup: true)]
    // #[nwg_control(parent: window, text: "&Edit")]
    // main_menu: nwg::Menu,
    //
    // #[nwg_control(parent: main_menu, text: "Paste Items")]
    // // #[nwg_events(OnMenuItemSelected: [QCApp::paste_items])]
    // listbox_menu_paste: nwg::MenuItem,
    // Status bar
    // #[nwg_control(parent: window, font: Some(&data.font))]
    // #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    // #[nwg_layout_item(layout: window_layout,
    // // margin: MARGIN,
    // // flex_grow: 2.0,
    // size: Size { width: D::auto(), height: D::auto() }
    // )]
    // status: nwg::StatusBar,
}

impl QCApp {
    /// Initial application setup when the event queue just started.
    fn setup(&self) {
        self.window.set_visible(true);
    }

    fn resize_clock_box(&self) {
        let (w, h) = self.clock_frame.size();
        self.clock_box.set_size(w, h);
    }

    fn do_clcik(&self) {
        self.product_name_field.push("test")
    }

    fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = QCApp::build_ui(Default::default()).expect("Failed to build UI");
    // _app.status.set_text(
    //     0,
    //     &format!("Current mode: {:?}; Instances linked: {}", 5, 7,),
    // );
    nwg::dispatch_thread_events();
}
