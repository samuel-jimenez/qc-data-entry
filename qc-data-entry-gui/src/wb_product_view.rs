extern crate native_windows_derive as nwd;

use log::error;
use nwd::NwgPartial;
use nwg::{taffy::FlexDirection, ControlHandle, EventData, KeyPress, Setters};
use qc_data_entry::{QCProductStandard, SampleInfo, SampledProduct};

use crate::{
    constants::{COL_20, COL_30, COL_70, COL_80, GROUP_PADDING},
    number_edit_fixed::FixedNumEdit,
    RangeView, VISUAL_MARGIN,
};

#[derive(Default, NwgPartial)]
pub struct WBPanelView {
    #[nwg_layout(auto_spacing:None)]
    wb_layout: nwg::FlexboxLayout,

    #[nwg_control(flags: "VISIBLE")]
    // #[nwg_layout_item(layout: wb_layout)]
    #[nwg_layout_item(layout: wb_layout, size:COL_70)]
    wb_frame: nwg::Frame,
    #[nwg_partial(parent: wb_frame)]
    #[nwg_shortcuts((visual, ph, sg)  [A, D, NumpadSlash, NumpadTimes]: [WBPanelView::proc_nav_shortcut()])]
    // #[nwg_events((ph, sg) OnChar: [FixedNumEdit::press_key(TARGET, EVT_DATA)], (ph, sg) OnTextInput: [FixedNumEdit::parse(TARGET)])]
    // #[nwg_events((ph, sg) OnChar: [FixedNumEdit::type_key(TARGET, EVT_DATA)], (ph, sg) OnKeyPress: [FixedNumEdit::press_key(TARGET, EVT_DATA)], (ph, sg) OnTextInput: [WBPanelView::check_data_entry(SELF,TARGET)])]
    #[nwg_events((ph, sg) OnTextInput: [WBPanelView::check_data_entry(SELF,TARGET,HANDLE)])]
    product_wb: WBProductView,

    // #[nwg_partial(be: control)]
    #[nwg_partial_control]
    // #[nwg_layout_item(layout: wb_layout)]
    #[nwg_layout_item(layout: wb_layout, size:COL_30)]
    product_range: WBRangesView,
}
impl WBPanelView {
    fn proc_nav_shortcut() {}
    fn check_data_entry(
        &self,
        field: &FixedNumEdit,
        handle: &ControlHandle,
    ) -> Result<(), std::num::ParseFloatError> {
        let val = field.parse()?;
        let ok_p = if handle == &self.product_wb.ph {
            self.product_range.ph.check(val)
        } else if handle == &self.product_wb.sg {
            self.product_range.sg.check(val)
        } else {
            false
        };

        if ok_p {
            field.set_border_color(None);
        } else {
            field.set_border_color(Some([0xff, 0, 0]));
        }

        Ok(())
    }

    pub fn click(&self) {
        self.product_range
            .sg
            // .set(&(vec![None, Some(5.1), None]).into());
            // .set(&vec![None, Some(5.1), None].into());
            .set(&Some(vec![Some(0.1), Some(5.1), Some(0.1115)].into()));
        // .set(&vec![Some(0.1), Some(5.1), Some(0.1115)].into());
        // .set(vec![None, Some(5.1), None].into());
        // .set(*vec![None, Some(5.1), None].into());
    }

    pub(crate) fn update_product(&self, qc_product: &QCProductStandard) -> () {
        self.product_range.ph.set(&qc_product.ph);
        self.product_range.sg.set(&qc_product.sg);
    }

    pub(crate) fn get_samples(&self, sample_info: SampleInfo) -> Vec<SampledProduct> {
        let mut sample: SampledProduct = sample_info.into();
        sample.visual = self.product_wb.visual.check_state() == nwg::CheckBoxState::Checked;

        sample.ph = self.product_wb.ph.parse().ok();
        sample.sg = self.product_wb.sg.parse().ok();
        vec![sample]
    }
}

#[derive(Default, NwgPartial)]
#[nwg_shortcuts((visual, ph, sg)  [W, S, NumpadMinus, NumpadPlus]: [WBProductView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
// #[nwg_events((ph, sg) OnChar: [FixedNumEdit::press_key(TARGET, EVT_DATA)], (ph, sg) OnTextInput: [FixedNumEdit::parse(TARGET)])]
pub struct WBProductView {
    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Column, padding:GROUP_PADDING)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Visual Inspection")]
    #[nwg_layout_item(layout: frame_layout, margin:VISUAL_MARGIN)]
    visual: nwg::CheckBox,

    #[nwg_control(label: "pH", places: 1, precision: 2)]
    // #[nwg_control(label: "mass", places: 3, precision: 2)]
    #[nwg_layout_item(layout: frame_layout)]
    // #[nwg_events(OnChar: [FixedNumEdit::press_key(TARGET, EVT_DATA)], OnTextInput: [FixedNumEdit::parse(TARGET)])]
    ph: FixedNumEdit,

    #[nwg_control(label: "Specific Gravity", places: 1, precision: 4)]
    // #[nwg_control(label: "Specific Gravity", places: 1, precision: 4, background_color:Some([0xff,0,0]))]
    // #[nwg_control(label: "Specific Gravity", places: 1, precision: 4, background_color:Some([0,0,0xff]))]
    #[nwg_layout_item(layout: frame_layout)]
    // #[nwg_events(OnChar: [FixedNumEdit::press_key(TARGET, EVT_DATA)], OnTextInput: [FixedNumEdit::parse(TARGET)])]
    sg: FixedNumEdit,
}

impl WBProductView {
    fn parse(&self, field: &FixedNumEdit) -> Result<f32, std::num::ParseFloatError> {
        field.parse()
        // field.parse_sg()
    }

    fn proc_nav_shortcut(&self, combo: &nwg::KeyCombo, handle: &nwg::ControlHandle) {
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
