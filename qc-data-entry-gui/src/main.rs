// #![windows_subsystem = "windows"]
// #![allow(unused_imports)]
// #![allow(unused)]
// #![allow(dead_code)]
// #![warn(unused_imports)]
// #![warn(unused)]

/*!
 *   A very simple application that shows your name in a message box.
 *   Unlike `basic_d`, this example uses layout to position the controls in the window
 */

extern crate native_windows_derive as nwd;

mod clock;
mod constants;
mod convert;
mod formats;
mod fr_product_view;
mod number_units_edit;
mod ob_product_view;
mod qc_product_view;
mod range;
mod wb_product_view;

use nwd::NwgUi;
use nwg::{
    taffy::{
        geometry::Size,
        style::{Dimension as D, FlexDirection},
        style_helpers::auto,
    },
    NativeUi,
};

use crate::{
    clock::ClockBox, constants::*, fr_product_view::FRPanelView,
    number_units_edit::NumberUnitsEdit, ob_product_view::OBPanelView, range::*,
    wb_product_view::WBPanelView,
};

const PC_10: D = D::percent(0.1);
const PC_25: D = D::percent(0.25);
const PC_50: D = D::percent(0.5);
const PC_33: D = D::percent(0.33);

const COL_1_3: Size<D> = Size {
    width: PC_33,
    height: auto(),
};

const ROW_10: Size<D> = Size {
    width: auto(),
    height: PC_10,
};

const ROW_25: Size<D> = Size {
    width: auto(),
    height: PC_25,
};

const ROW_50: Size<D> = Size {
    width: auto(),
    height: PC_50,
};

#[derive(Default, NwgUi)]
pub struct QCApp {
    // Resources
    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("MAINICON"))]
    window_icon: nwg::Icon,

    // Window and layout
    #[nwg_control(size: (1400, 800), position: (300, 300), title: "QC Data Entry", flags: "MAIN_WINDOW", icon:  Some(&data.window_icon))]
    #[nwg_events( OnInit: [QCApp::setup], OnWindowClose: [QCApp::say_goodbye] )]
    window: nwg::Window,

    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Column, padding: PADDING)]
    // window_layout: nwg::FlexboxLayout,
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
    // //     width: auto(),
    // //
    // //     // width: D::length(200.0),
    // //                   height: D::length(200.0),
    // // },
    // )]
    #[nwg_layout_item(layout: window_layout, size:ROW_25)]
    top_layout: nwg::FlexboxLayout,

    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Column, align_items: stretch::style::AlignItems::FlexStart)]
    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    // #[nwg_layout_item(layout: top_layout, size:LAEBL_SIZE)]
    // label_0_layout: nwg::FlexboxLayout,
    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    #[nwg_layout_item(layout: top_layout, size:COL_1_3)]
    field_0_layout: nwg::FlexboxLayout,

    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    // #[nwg_layout_item(layout: top_layout, size:LAEBL_SIZE)]
    // label_1_layout: nwg::FlexboxLayout,
    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    #[nwg_layout_item(layout: top_layout, size:COL_1_3)]
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
    #[nwg_control(label: "Product", focus: true)]
    #[nwg_layout_item(layout: field_0_layout)]
    product_name_field: nwg::LabeledCombo<&'static str>,

    #[nwg_control(label: "Customer Name")]
    #[nwg_layout_item(layout: field_1_layout)]
    customer_name_field: nwg::LabeledCombo<&'static str>,

    #[nwg_control(label: "Lot Number")]
    #[nwg_layout_item(layout: field_0_layout)]
    lot_name_field: nwg::LabeledCombo<&'static str>,

    #[nwg_control(label: "Sample Point")]
    #[nwg_layout_item(layout: field_1_layout)]
    sample_name_field: nwg::LabeledCombo<&'static str>,

    #[nwg_control(label: "Tester")]
    #[nwg_layout_item(layout: field_0_layout)]
    tester_name_field: nwg::LabeledCombo<&'static str>,

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
    #[nwg_layout_item(layout: toolbar_button_layout)]
    #[nwg_events( OnButtonClick:[QCApp::do_rang])]
    hello_button: nwg::Button,

    #[nwg_control(text: "Inventory")]
    #[nwg_layout_item(layout: toolbar_button_layout)]
    #[nwg_events( OnButtonClick:[QCApp::do_clcik])]
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

    #[nwg_control(parent: window, text: "Submit")]
    #[nwg_layout_item(layout: action_button_layout)]
    submit_button: nwg::Button,

    #[nwg_control(parent: window, text: "Clear")]
    #[nwg_layout_item(layout: action_button_layout)]
    clear_button: nwg::Button,

    #[nwg_control(parent: window, text: "Log")]
    #[nwg_layout_item(layout: action_button_layout)]
    log_button: nwg::Button,

    #[nwg_partial(parent: panel_wb)]
    product_wb: WBPanelView,
    // product_wb: FRPanelView,
    #[nwg_partial(parent: panel_ob)]
    product_ob: OBPanelView,

    #[nwg_partial(parent: panel_fr)]
    product_fr: FRPanelView,
}

impl QCApp {
    /// Initial application setup when the event queue just started.
    fn setup(&self) {
        // Do this here to avoid handling layout twice
        self.window.set_visible(true);
    }

    fn resize_clock_box(&self) {
        let (w, h) = self.clock_frame.size();
        self.clock_box.set_size(w, h);
    }

    fn do_rang(&self) {
        self.product_fr.click();
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
