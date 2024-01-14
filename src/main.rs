use ndarray::Array;
use num;
use plotters::prelude::*;
use std::{f64::consts::PI, f64::INFINITY, time};

struct TransferFunction {
    // increasing powers of s
    numerator: Vec<f64>,

    // increasing powers of s
    denominator: Vec<f64>,
}

impl TransferFunction {
    pub fn new(numerator: Vec<f64>, denominator: Vec<f64>) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    // Frequency Domain Function
    fn plot_bode(&self, fname: &str, title: &str) {
        let (freq, mag, phase) = self.get_bode_data();

        let root_area = BitMapBackend::new(fname, (1024, 1024)).into_drawing_area();
        root_area.fill(&BLACK).unwrap();

        let root_area = root_area
            .titled(title, ("monospace", 60).into_font().color(&WHITE))
            .unwrap();
        let drawing_areas = root_area.split_evenly((2, 1));

        let upper = &drawing_areas[0];
        let lower = &drawing_areas[1];

        let x_axis = freq.clone();

        let fmin: f32 = freq[0] as f32;
        let fmax: f32 = freq[freq.len() - 1] as f32;
        let cx = fmin..fmax;

        let (mmin, mmax) = {
            let mut mi = INFINITY;
            let mut ma = -INFINITY;
            for m in mag.clone() {
                if m < mi {
                    mi = m;
                }

                if m > ma {
                    ma = m;
                }
            }
            (mi as f32, ma as f32)
        };

        let mut cc = ChartBuilder::on(&upper)
            .margin(5)
            .set_all_label_area_size(50)
            .caption("Gain [dB]", ("monospace", 40).into_font().color(&WHITE))
            .build_cartesian_2d(cx.clone().log_scale(), (mmin - 1.0)..(mmax + 1.0))
            .unwrap();

        cc.configure_mesh()
            .label_style(&WHITE)
            .axis_style(&WHITE)
            .bold_line_style(&WHITE)
            .light_line_style(RGBAColor {
                0: 189,
                1: 189,
                2: 189,
                3: 0.5,
            })
            .x_labels(20)
            .y_labels(5)
            .x_label_formatter(&|v| format!("{:.1}", v))
            .x_desc("Frequency [Hz]")
            .x_label_style(&WHITE)
            .y_label_formatter(&|v| format!("{:.1}", v))
            .y_label_style(&WHITE)
            .y_desc("Gain [dB]")
            .draw()
            .unwrap();

        let plot_style = ShapeStyle {
            color: Into::into(RED),
            stroke_width: 2,
            filled: true,
        };

        cc.draw_series(LineSeries::new(
            x_axis
                .clone()
                .iter()
                .enumerate()
                .map(|(ii, f)| ((*f / (2.0 * PI)) as f32, mag[ii] as f32)),
            plot_style,
        ))
        .unwrap();

        let (pmin, pmax) = {
            let mut mi = INFINITY;
            let mut ma = -INFINITY;
            for p in phase.clone() {
                if p < mi {
                    mi = p;
                }

                if p > ma {
                    ma = p;
                }
            }
            (mi as f32, ma as f32)
        };

        let mut cc = ChartBuilder::on(&lower)
            .margin(5)
            .set_all_label_area_size(50)
            .caption("Phase [deg]", ("monospace", 40).into_font().color(&WHITE))
            .build_cartesian_2d(cx.log_scale(), (pmin - 10.0)..(pmax + 10.0))
            .unwrap();

        cc.configure_mesh()
            .label_style(&WHITE)
            .axis_style(&WHITE)
            .bold_line_style(&WHITE)
            .light_line_style(RGBAColor {
                0: 189,
                1: 189,
                2: 189,
                3: 0.5,
            })
            .x_labels(20)
            .y_labels(5)
            .x_label_formatter(&|v| format!("{:.1}", v))
            .x_desc("Frequency [Hz]")
            .x_label_style(&WHITE)
            .y_label_formatter(&|v| format!("{:.1}", v))
            .y_label_style(&WHITE)
            .y_desc("Phase [deg]")
            .draw()
            .unwrap();

        let plot_style = ShapeStyle {
            color: Into::into(RED),
            stroke_width: 2,
            filled: true,
        };

        cc.draw_series(LineSeries::new(
            x_axis
                .iter()
                .enumerate()
                .map(|(ii, f)| ((*f / (2.0 * PI)) as f32, phase[ii] as f32)),
            plot_style,
        ))
        .unwrap();

        root_area.present().expect("Can't plot");
    }

    fn get_bode_data(&self) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let freq_range: Vec<f64> = Array::logspace(10.0, -3.0, 5.0, 1000).to_vec();
        let mut magnitude: Vec<f64> = Vec::with_capacity(freq_range.len());
        let mut phase: Vec<f64> = Vec::with_capacity(freq_range.len());

        for freq in freq_range.iter() {
            // calculate numerator with s := jw
            let lnum = TransferFunction::s2jw(&self.numerator, *freq);

            // calculate denominator with s := jw
            let lden = TransferFunction::s2jw(&self.denominator, *freq);

            // calc mag, phase at freq
            magnitude.push(20.0 * ((lnum / lden).norm().log10()));
            phase.push((lnum / lden).arg() as f64 * 180.0 / PI);
        }

        (freq_range, magnitude, phase)
    }

    fn s2jw(tf_part: &Vec<f64>, freq: f64) -> num::Complex<f64> {
        let lpart: num::Complex<f64> = tf_part
            .iter()
            .enumerate()
            .map(|(jj, &value)| {
                let cval = num::complex::Complex::new(value as f64, 0.0);
                let fval = num::complex::Complex::new(0.0, freq);

                cval * fval.powc(num::complex::Complex::new(jj as f64, 0.0))
            })
            .sum();

        lpart
    }
}

struct LowPassFilter {
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

    fn plot_bode(&self, fname: &str) {
        let lpf_title: String = format!("Low Pass Filter: {}Hz Cut-Off", self.cut_off / (2.0 * PI));
        self.transfer_function
            .plot_bode(fname, &lpf_title.to_owned());
    }
}

fn main() {
    let cut_off = 50;

    let start = time::Instant::now();
    let lpf = LowPassFilter::new(cut_off.into());
    let create = start.elapsed().as_micros();

    lpf.plot_bode("media/LPF_bode.png".into());
    let lpf_bode = start.elapsed().as_micros();

    let tf = TransferFunction::new(vec![16.0], vec![16.0, 8.0 * 0.05, 1.0]);
    let tf_create = start.elapsed().as_micros();

    tf.plot_bode("media/TF_bode.png", "Second Order System");
    let tf_plot = start.elapsed().as_micros();

    println!(
        "Total Time: {:.3}.{:.3}ms",
        (tf_plot / 1000),
        tf_plot % 1000
    );
    println!("LPF Create: {:.3}.{:.3}ms", create / 1000, create % 1000);
    println!(
        "LPF Plot: {:.3}.{:.3}ms",
        (lpf_bode - create) / 1000,
        (lpf_bode - create) % 1000
    );
    println!(
        "TF Create: {:.3}.{:.3}ms",
        (tf_create - lpf_bode) / 1000,
        (tf_create - lpf_bode) % 1000
    );
    println!(
        "TF Plot: {:.3}.{:.3}ms",
        (tf_plot - tf_create) / 1000,
        (tf_plot - tf_create) % 1000
    );
}
