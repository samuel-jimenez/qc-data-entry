// #![windows_subsystem = "windows"]
// #![allow(unused_imports)]
#![allow(unused)]
#![allow(dead_code)]
// #![warn(unused_imports)]
// #![warn(unused)]

/*!
 *   Displays main entry window
 */

extern crate native_windows_derive as nwd;

mod clock;
mod constants;
mod fr_product_view;
mod number_edit_fixed;
mod number_units_edit;
mod ob_product_view;
mod qc_product_view;
mod qr;
mod range;
mod wb_product_view;

use std::sync::Arc;

use log::{error, info};
use log_result::ResultLog;
use nwd::NwgUi;
use nwg::{
    taffy::{
        geometry::Size,
        style::{Dimension as D, FlexDirection},
        style_helpers::auto,
    },
    NativeUi, ShortcutUi,
};
use qc_data_entry::{
    init_logger, DataEntryConfig, LotList, ProductLine, ProductLot, QcTesterList, DB,
};

use crate::{
    clock::ClockBox, constants::*, fr_product_view::FRPanelView,
    number_units_edit::NumberUnitsEdit, ob_product_view::OBPanelView, qr::QRJson, range::*,
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
#[nwg_shortcuts(Shift+LBracket: [QCApp::catch_qr_start])]
pub struct QCApp {
    // Resources
    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("MAINICON"))]
    window_icon: nwg::Icon,

    // Window and layout
    #[nwg_control(size: (1400, 800), position: (300, 300), title: "QC Data Entry", flags: "MAIN_WINDOW", icon:  Some(&data.window_icon))]
    #[nwg_events( OnInit: [QCApp::setup], OnWindowClose: [QCApp::say_goodbye])]
    window: nwg::Window,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    window_layout: nwg::FlexboxLayout,
    // window_layout: nwg::DynLayout, // ToDO
    //
    #[nwg_layout(parent: window, flex_direction: FlexDirection::Row)]
    #[nwg_layout_item(layout: window_layout, size:ROW_10)]
    toolbar_button_layout: nwg::FlexboxLayout,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Row)]
    #[nwg_layout_item(layout: window_layout, size:ROW_25)]
    top_layout: nwg::FlexboxLayout,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    #[nwg_layout_item(layout: top_layout, size:COL_1_3)]
    field_0_layout: nwg::FlexboxLayout,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    #[nwg_layout_item(layout: top_layout, size:COL_1_3)]
    field_1_layout: nwg::FlexboxLayout,

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
    #[nwg_control(visible:  false)]
    #[nwg_shortcuts(Shift+RBracket: [QCApp::catch_qr_end])]
    #[nwg_events(     OnKeyPress: [QCApp::test_qr])]
    qr_catcher: nwg::TextInput,

    // #[nwg_control(label: "Product", focus: true)]
    #[nwg_control(label: "Product", focus: true, upcase: true)]
    #[nwg_layout_item(layout: field_0_layout)]
    #[nwg_events( OnComboBoxSelection:[QCApp::prod_sel],OnComboxBoxInput:[QCApp::prod_inp])]
    product_name_field: nwg::LabeledCombo<ProductLine>,

    #[nwg_control(label: "Customer Name", upcase: true)]
    #[nwg_layout_item(layout: field_1_layout)]
    #[nwg_events( OnKeyPress: [QCApp::do_key(SELF,EVT_DATA)],     OnSysKeyPress: [QCApp::do_key(SELF,EVT_DATA)], )]
    customer_name_field: nwg::LabeledCombo<&'static str>,

    #[nwg_control(label: "Lot Number", upcase: true)]
    #[nwg_layout_item(layout: field_0_layout)]
    lot_name_field: nwg::LabeledCombo<ProductLot>,

    #[nwg_control(label: "Sample Point", upcase: true)]
    #[nwg_layout_item(layout: field_1_layout)]
    sample_name_field: nwg::LabeledCombo<&'static str>,

    #[nwg_control(label: "Tester", upcase: true)]
    #[nwg_layout_item(layout: field_0_layout)]
    tester_name_field: nwg::LabeledCombo<QcTesterList>,

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
    #[nwg_layout_item(layout: window_layout, size:ROW_50)]
    tabs_container: nwg::TabsContainer,

    #[nwg_control(text: "Water Based")]
    panel_wb: nwg::Tab,

    #[nwg_control(text: "Oil Based" )]
    panel_ob: nwg::Tab,

    #[nwg_control(text: "Friction Reducer" )]
    panel_fr: nwg::Tab,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Row)]
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

    #[nwg_partial(parent: panel_ob)]
    product_ob: OBPanelView,

    #[nwg_partial(parent: panel_fr)]
    product_fr: FRPanelView,

    config: Arc<DataEntryConfig>,
    qc_db: Arc<DB>,
}

impl QCApp {
    /// Initial application setup when the event queue just started.
    fn setup(&self) {
        // Do this here to avoid handling layout twice
        self.window.set_visible(true);

        // info!("Using config: {}", config.config_path.display());
        // info!("Using config: {}", config_path.display());
        // info!("Using config: {}", config_msg);

        error!("Bright red error");
        info!("This only appears in the log file");

        self.product_name_field
            .set_collection(ProductLine::select_product_info_all(&*self.qc_db));

        self.tester_name_field
            .set_collection(QcTesterList::select_qc_tester_all(&*self.qc_db));

        // self.lot_name_field
        //     .set_collection(ProductLot::select_product_lot_product(&*self.qc_db));
        // select_product_lot

        // log_file, err := os.OpenFile(config.LOG_FILE, os.O_RDWR|os.O_CREATE|os.O_APPEND, 0666)
        // if err != nil {
        // 	log.Fatalf("Crit: error opening file: %v", err)
        // }
        // defer log_file.Close()
        // log.Println("Info: Logging to logfile:", config.LOG_FILE)
        //
        // log.SetOutput(log_file)
        // log.Println("Info: Using config:", config.Main_config.ConfigFileUsed())
        //
    }

    fn catch_qr_start(&self) -> bool {
        self.qr_catcher.set_focus();
        self.qr_catcher.set_text("");
        true
    }
    fn test_qr(&self) {
        println!("test_qr:{}", self.qr_catcher.text());
    }

    // fn catch_qr_end(&self) -> Result<()>{
    fn catch_qr_end(&self) -> serde_json::Result<()> {
        // fn catch_qr_end(&self) -> Result<(), Box<dyn std::error::Error>> {

        println!("END:{}", self.qr_catcher.text());
        let qr_json: QRJson =
            serde_json::from_str(&format!("{{{}}}", self.qr_catcher.text())).error()?;

        // log.Println("debug: ReadFromScanner: ", qr_json)
        // err := json.Unmarshal([]byte(qr_json), &product)
        // if err == nil {
        // 	view.product_panel.PopQRData(product)
        // } else {
        // 	log.Printf("error: [%s]: %q\n", "qr_json_mainWindow.keygrab", err)
        // }
        self.pop_qr_data(qr_json);

        self.qr_catcher.set_text("");
        self.tester_name_field.set_focus();
        Ok(())
    }

    fn pop_qr_data(&self, qr_json: QRJson) {
        println!("pop_qr_data:{}", self.qr_catcher.text());
    }

    //TODO add todAY button

    fn prod_sel(&self) {
        self.lot_name_field.set_collection(
            self.product_name_field.collection()[self.product_name_field.selection().unwrap()]
                .select_product_lot(&*self.qc_db),
        );
    }

    fn prod_inp(&self) {
        error!(
            "ProductLine type: {}",
            self.product_name_field.selection_string_or_text()
        );
    }

    fn resize_clock_box(&self) {
        let (w, h) = self.clock_frame.size();
        self.clock_box.set_size(w, h);
    }

    fn do_rang(&self) {
        self.product_wb.click();
        // self.product_fr.click();
    }

    fn do_clcik(&self) {
        self.customer_name_field.push("test")
    }

    //     fn print_char(data: &nwg::EventData) {
    //     println!("{:?}", data.on_char());
    // }
    fn do_char(&self, data: &nwg::EventData) {
        println!("{:?}", data.on_char());
    }
    fn do_key(&self, data: &nwg::EventData) {
        println!("{:?}", data.on_key());
    }

    fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
        self.config.save().unwrap();
    }
}

fn main() {
    //load config
    let config = Arc::new(DataEntryConfig::load());

    // // log to file
    init_logger(config.log_file.clone()).unwrap();

    info!("Using config: {}", config.config_path().display());

    //open_db
    let qc_db = Arc::new(DB::new(config.db_file()).error().unwrap());

    //
    // //setup print goroutine
    // threads.PRINT_QUEUE = make(chan string, 4)
    // defer close(threads.PRINT_QUEUE)
    // go threads.Do_print_queue(threads.PRINT_QUEUE)
    //
    // //setup status_bar goroutine
    // threads.STATUS_QUEUE = make(chan string, 16)
    // defer close(threads.STATUS_QUEUE)
    // go threads.Do_status_queue(threads.STATUS_QUEUE)
    // self.config.save().unwrap();

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let app_ui = QCApp::build_ui(QCApp {
        config,
        qc_db,
        ..Default::default()
    })
    .expect("Failed to build UI");

    app_ui.dispatch_thread_events();

    // config.save().unwrap();
}
