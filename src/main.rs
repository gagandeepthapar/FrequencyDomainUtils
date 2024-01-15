use std::time;

mod filters;
mod transfer_function;

fn main() {
    let cut_off = 50;

    let start = time::Instant::now();
    let lpf = filters::LowPassFilter::new(cut_off.into());
    let create = start.elapsed().as_micros();

    lpf.plot_bode("media/LPF_bode.png".into());
    let lpf_bode = start.elapsed().as_micros();

    let tf = transfer_function::TransferFunction::new(vec![16.0], vec![16.0, 8.0 * 0.05, 1.0]);
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
