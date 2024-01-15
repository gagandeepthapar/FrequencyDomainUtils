mod filters;
mod transfer_function;

use std::time;

fn main() {
    // Low Pass Filter
    let start = time::Instant::now();

    let cut_off = 50;
    let lpf = filters::LowPassFilter::new(cut_off.into());
    lpf.plot_bode("media/LPF_bode.png".into());

    let dur = start.elapsed().as_micros();
    println!("Low Pass Filter: {:.3}.{:.3}ms", dur / 1000, dur % 1000);

    // second order transfer function
    let start = time::Instant::now();

    let tf = transfer_function::TransferFunction::new(vec![16.0], vec![16.0, 8.0 * 0.05, 1.0]);
    tf.plot_bode("media/TF_bode.png", "Second Order System");

    let dur = start.elapsed().as_micros();
    println!("Second Order TF: {:.3}.{:.3}ms", dur / 1000, dur % 1000);

    // Integrator
    let start = time::Instant::now();

    let integrator = transfer_function::TransferFunction::default();
    integrator.plot_bode("media/integrator_bode.png", "Integrator");

    let dur = start.elapsed().as_micros();
    println!("Integrator: {:.3}.{:.3}ms", dur / 1000, dur % 1000);
}
