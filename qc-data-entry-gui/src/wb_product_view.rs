extern crate native_windows_derive as nwd;

use nwd::NwgPartial;
use nwg::{taffy::FlexDirection, KeyPress};

use crate::{
    constants::{COL_20, COL_80, GROUP_PADDING},
    RangeView, VISUAL_MARGIN,
};

#[derive(Default, NwgPartial)]
pub struct WBPanelView {
    #[nwg_layout(auto_spacing:None)]
    wb_layout: nwg::FlexboxLayout,

    #[nwg_control(flags: "VISIBLE")]
    // #[nwg_layout_item(layout: wb_layout)]
    #[nwg_layout_item(layout: wb_layout, size:COL_80)]
    wb_frame: nwg::Frame,
    #[nwg_partial(parent: wb_frame)]
    #[nwg_shortcuts((visual, ph, sg)  [A, D, NumpadSlash, NumpadTimes]: [WBPanelView::proc_nav_shortcut()])]
    product_wb: WBProductView,

    // #[nwg_partial(be: control)]
    #[nwg_partial_control]
    // #[nwg_layout_item(layout: wb_layout)]
    #[nwg_layout_item(layout: wb_layout, size:COL_20)]
    product_range: WBRangesView,
}
impl WBPanelView {
    fn proc_nav_shortcut() {}
}

#[derive(Default, NwgPartial)]
#[nwg_shortcuts((visual, ph, sg)  [W, S, NumpadMinus, NumpadPlus]: [WBProductView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
pub struct WBProductView {
    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Column, padding:GROUP_PADDING)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Visual Inspection")]
    #[nwg_layout_item(layout: frame_layout, margin:VISUAL_MARGIN)]
    visual: nwg::CheckBox,

    #[nwg_control(label: "pH")]
    #[nwg_layout_item(layout: frame_layout)]
    #[nwg_events(OnTextInput: [WBProductView::tick])]
    ph: nwg::LabeledEdit,

    #[nwg_control(label: "Specific Gravity")]
    #[nwg_layout_item(layout: frame_layout)]
    sg: nwg::LabeledEdit,
}

impl WBProductView {
    fn tick(&self) {
        self.sg.set_label(&self.ph.text());
    }
    fn proc_nav_shortcut(&self, combo: &nwg::KeyCombo, handle: &nwg::ControlHandle) {
        // match handle {
        //     self.visual.handle =>
        match combo {
            nwg::KeyCombo {
                key: KeyPress::W, ..
            }
            | nwg::KeyCombo {
                key: KeyPress::NumpadMinus,
                ..
            } => {
                if handle == &self.visual {
                    self.visual.set_focus()
                } else if handle == &self.ph {
                    self.visual.set_focus()
                } else if handle == &self.sg {
                    self.ph.set_focus()
                }
            }
            nwg::KeyCombo {
                key: KeyPress::S, ..
            }
            | nwg::KeyCombo {
                key: KeyPress::NumpadPlus,
                ..
            } => {
                if handle == &self.visual {
                    self.ph.set_focus()
                } else if handle == &self.ph {
                    self.sg.set_focus()
                } else if handle == &self.sg {
                    self.sg.set_focus()
                }
            }
            _ => {}
        }
        println!("woo");
    }
}

#[derive(Default, NwgPartial)]
pub struct WBRangesView {
    #[nwg_control]
    #[nwg_root]
    frame: nwg::Frame,

    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Column, padding:GROUP_PADDING)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Visual Inspection")]
    #[nwg_layout_item(layout: frame_layout,margin:VISUAL_MARGIN)]
    visual: nwg::Label,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    ph: RangeView,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    sg: RangeView,
}
