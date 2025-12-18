// #![windows_subsystem = "windows"]
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

// Stretch style
use nwg::stretch::{
    geometry::{Rect, Size},
    style::{Dimension as D, FlexDirection, FlexWrap},
};

const PC_50: D = D::Percent(0.5);
const PC_33: D = D::Percent(0.33);
const PC_100: D = D::Percent(1.0);

const PT_10: D = D::Points(10.0);
const PT_5: D = D::Points(5.0);
const PADDING: Rect<D> = Rect {
    start: PT_10,
    end: PT_10,
    top: PT_10,
    bottom: PT_10,
};
const MARGIN: Rect<D> = Rect {
    start: PT_5,
    end: PT_5,
    top: PT_5,
    bottom: PT_5,
};

const SD_SIZE: Size<D> = Size {
    // width: D::Auto,
    width: D::Percent(0.33),
    height: D::Auto,
    // height: PC_33,
};

// #[derive(Default, NwgPartial)]
#[derive(Default, NwgUi)]
pub struct QCApp {
    // Window and layout
    // #[nwg_control(size: (300, 115), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_control(size: (600, 400), position: (300, 300), title: "Basic example", flags: "MAIN_WINDOW|VISIBLE")]
    #[nwg_events( OnInit: [QCApp::setup], OnWindowClose: [QCApp::say_goodbye] )]
    window: nwg::Window,

    // #[nwg_layout(parent: window, spacing: 1, margin: [40, 5, 30, 5])]
    // grid: nwg::GridLayout,

    //     // #[nwg_layout(parent: window, flex_direction: FlexDirection::Row, padding: PADDING, flex_wrap:FlexWrap::Wrap)]
    //     #[nwg_layout(parent: window, flex_direction: FlexDirection::Row, padding: PADDING, flex_wrap:FlexWrap::NoWrap)]
    //     #[nwg_layout_item(layout: window_layout,
    //     size:Size {
    //         // width: D::Auto,
    //
    //         width: D::Points(200.0),
    //                       height: D::Points(200.0),
    //     },
    //     )]
    //     // #[nwg_partial(parent: window_layout,    )]
    //     top_layout: nwg::FlexboxLayout,

    // #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Column, padding: PADDING,    flex_wrap:FlexWrap::Wrap)]
    #[nwg_layout(parent: window, flex_direction: FlexDirection::Column)]
    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Column, padding: PADDING,    flex_wrap:FlexWrap::NoWrap)]
    // #[nwg_layout(parent: window,)]
    window_layout: nwg::FlexboxLayout,
    // window_layout: nwg::DynLayout,
    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Row, padding: PADDING, flex_wrap:FlexWrap::Wrap)]
    #[nwg_layout(parent: window, flex_direction: FlexDirection::Row)]
    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Row, padding: PADDING, flex_wrap:FlexWrap::NoWrap)]
    // #[nwg_partial(parent: window_layout,    )]
    #[nwg_layout_item(layout: window_layout,
    // size:Size {
    //     width: D::Auto,
    //
    //     // width: D::Points(200.0),
    //                   height: D::Points(200.0),
    // },
    )]
    top_layout: nwg::FlexboxLayout,

    /*
     *
     *
     *    // #[nwg_layout(parent: window, flex_direction: FlexDirection::Row, padding: PADDING, flex_wrap:FlexWrap::Wrap)]
     *    #[nwg_layout(parent: window, flex_direction: FlexDirection::Row, padding: PADDING, flex_wrap:FlexWrap::NoWrap)]
     *    #[nwg_layout_item(layout: window_layout,
     *        // size:Size {
     *        //     // width: D::Auto,
     *        //
     *        //     width: D::Points(200.0),
     *        //                   height: D::Points(200.0),
     *        // },
     *    )]
     *        // #[nwg_partial(parent: window_layout,    )]
     *    top_layout: nwg::FlexboxLayout,
     *
     *
     */
    #[nwg_control(parent: window,)]
    // #[nwg_layout_item(layout: grid, row: 2, col: 0, row_span: 2)]
    #[nwg_layout_item(layout: window_layout,
    // size:Size {
    //     width: D::Auto,
    //     // height: D::Auto,
    //     height: D::Points(200.0),
    // },
    )]
    hello_button: nwg::Button,

    // // Controls
    // NO
    // #[nwg_partial(parent: window, )]
    // #[nwg_layout_item(layout: window_layout,
    // size:Size {
    //     width: D::Auto,
    //
    //     // height: D::Auto,
    //
    //
    //     height: D::Points(200.0),
    // },
    //                   )]
    // top: TopApp,

    // #[nwg_control(parent: window,               )]
    // #[nwg_layout_item(layout: window_layout,
    // size:Size {
    //     width: D::Auto,
    //
    //     // height: D::Auto,
    //
    //
    //     height: D::Points(200.0),
    //
    // })]
    // top_frame: nwg::Frame,
    // #[nwg_partial(parent: top_frame)]
    // top: TopApp,
    #[nwg_control]
    // #[nwg_control(parent: window, text: "Top")]
    // #[nwg_layout_item(layout: window_layout,
    #[nwg_layout_item(layout: top_layout,
    // flex_grow: 1.0,
    // size:Size {
    //     width: D::Auto,
    //     height: D::Points(200.0),
    // },
    size:SD_SIZE,
    // size:Size {
    //     // width: D::Auto,
    //     width: PC_100,
    //     // height: D::Auto,
    //     height: PC_100,
    // },
    )]
    // name_edit: nwg::TextInput,
    name_edit: nwg::ComboBox<&'static str>,

    #[nwg_control]
    // #[nwg_control(text: "toop",)]
    #[nwg_layout_item(layout: top_layout,    size:SD_SIZE)]
    // toop_edit: nwg::TextInput,
    toop_edit: nwg::ComboBox<&'static str>,

    #[nwg_control(size: (300, 200),)]
    #[nwg_events( OnResize:[QCApp::resize_clock_box])]
    #[nwg_layout_item(layout: top_layout,
    // aspect_ratio: Number,


    size:Size {
        // width: D::Auto,
    width: D::Percent(0.33),
                      height: D::Auto,
        // height: D::Points(200.0),
    },
    )]
    clock_frame: nwg::Frame,
    #[nwg_partial(parent: clock_frame,    )]
    clock_box: ClockBox,

    #[nwg_control]
    #[nwg_layout_item(layout: window_layout)]
    // liste_edit: nwg::ListView,
    liste_edit: nwg::TabsContainer,
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
    // size: Size { width: D::Auto, height: D::Auto }
    // )]
    // status: nwg::StatusBar,
    // run_time: time::Instant,
    // run_time: time::Time,
    // run_time: DateTime<Local> = Local::now(),
    // run_time: DateTime<Local>,
    // run_time: DateTime<Utc>,
}

impl QCApp {
    /// Initial application setup when the event queue just started.
    fn setup(&self) {
        {
            // self.run_time = time::Instant::now();
            // self.run_time = Local::now();
            // self.run_time = Utc::now();

            // self.window
            //     .set_text(&format!("SyncDraw - {}", data.instance_id));
            // self.status.set_text(
            //     0,
            //     &format!(
            //         "Current mode: {:?}; Instances linked: {}",
            //         data.mode,
            //         data.instances_count()
            //     ),
            // );
        }
        // self.clock_box.init();

        self.window.set_visible(true);
    }

    // fn resize_clock_box(&mut self) {
    // fn resize_clock_box(&Rc<self>) {
    // fn resize_clock_box(&Rc<>) {
    // fn resize_clock_box(self &Rc::<QCApp>) {

    fn resize_clock_box(&self) {
        // fn resize_clock_box(&self, event: &EventData) {
        // found reference `&Rc<QCApp>`
        // self.clock_box.resize()
        // Rc::get_mut(&mut value)
        // self.get_mut().clock_box.resize();
        let (w, h) = self.clock_frame.size();

        // self.clock_box.resize(w, h);

        self.clock_box.set_size(w, h);

        // MUT_SELF
        // Rc::get_mut(self)?.clock_box.resize()
    }

    /*
         *    fn say_hello(&self) {
         *        nwg::modal_info_message(
         *            &self.window,
         *            "Hello",
         *            // &format!("Hello {}", self.name_edit.text()),
         *            // &format!("Hello {}", self.produc_name.text()), //TODO
         *            &format!("Hello {}", self.produc_name.selection_string_or_text(),),
         *        );
         *
         *        println!("WM_NOTIFY {} ", self.produc_name.selection_string_or_text());
         *        std::io::stdout().flush().unwrap();
         *
         *        self.produc_name.push("test")
         *
         *        // set_collection
    }*/

    fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }

    // fn tick(&self) {
    //     println!("TICK");
    //     std::io::stdout().flush().unwrap();
    //     let run_time = Utc::now();
    //
    //     let future_time = run_time
    //         .checked_add_signed(TimeDelta::seconds(20))
    //         .unwrap_or(DateTime::UNIX_EPOCH);
    //     // .unwrap_or(DateTime::MIN_UTC);
    //
    //     // println!("{}", self.run_time.elapsed().as_secs());
    //     // println!("{}", Local::now().
    //     println!(
    //         "{} {}",
    //         run_time.time().format("%S").to_string(),
    //         future_time.time().format("%S").to_string(),
    //     );
    //     std::io::stdout().flush().unwrap();
    //     self.hello_button
    //         .set_text(&run_time.time().format("%S").to_string());
    // }
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
