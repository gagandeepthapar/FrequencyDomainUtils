use std::f64::consts::PI;

use crate::transfer_function::TransferFunction;

pub struct LowPassFilter {
    cut_off: f64,
    filter_order: usize,
    transfer_function: TransferFunction,
    digital_coefficients: (Vec<f64>, Vec<f64>),
}

impl LowPassFilter {
    // Constructor
    pub fn new(cut_off_freq_hz: f64) -> Self {
        let cof = cut_off_freq_hz as f64 * 2.0 * PI;
        Self {
            cut_off: cof,
            filter_order: 1,
            transfer_function: TransferFunction::new(vec![cof], vec![cof, 1.0]),
            digital_coefficients: (Vec::new(), Vec::new()),
        }
    }

    pub fn plot_bode(&self, fname: &str) {
        let lpf_title: String = format!("Low Pass Filter: {}Hz Cut-Off", self.cut_off / (2.0 * PI));
        self.transfer_function
            .plot_bode(fname, &lpf_title.to_owned());
    }
}
