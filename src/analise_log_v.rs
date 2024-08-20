


use dyn_smooth::DynamicSmootherEcoF32;
use std::f64;


pub const SAMPLE_RATE   : f32 = 50.0;
pub const BASE_FREQUENCY: f32 = 2.0;
pub const SENSITIVITY   : f32 = 0.5;

fn convert_to_f32(data: &[(f64, f64, f64)]) -> Vec<(f32, f32, f32)> {
    data.iter().map(|(x, y, z)| (*x as f32, *y as f32, *z as f32)).collect()
}


pub fn smooth_vector_components(
    data: &[(f64, f64, f64)],
    sample_rate: f32,
    base_frequency: f32,
    sensitivity: f32
) -> Vec<(f64, f64, f64)> {
    let mut smoothed_data = Vec::new();

    let data = &convert_to_f32(data);
    let mut smoother_x = DynamicSmootherEcoF32::new(base_frequency, sample_rate, sensitivity);
    let mut smoother_y = DynamicSmootherEcoF32::new(base_frequency, sample_rate, sensitivity);
    let mut smoother_z = DynamicSmootherEcoF32::new(base_frequency, sample_rate, sensitivity);
    // Apply smoothing to each component separately
    for &(x, y, z) in data {
        let smoothed_x = smoother_x.tick(x);
        let smoothed_y = smoother_y.tick(y);
        let smoothed_z = smoother_z.tick(z);
        smoothed_data.push((smoothed_x as f64, smoothed_y as f64, smoothed_z as f64));


        // Push the smoothed components into the output vector
        // smoothed_data.push((smoothed_x, smoothed_y, smoothed_z));
    }

    smoothed_data
}



use ordered_float::OrderedFloat;
use std::f32::NAN;

pub fn median_filter(rg: &[(f64, f64, f64)], window_size: usize) -> Vec<(f64, f64, f64)> {
    let n = rg.len();
    let half_window = window_size / 2;

    let mut result = Vec::with_capacity(n);

    for i in half_window..n-half_window {
        let start = i - half_window;
        let end = i + half_window;

        let mut sorted_x = rg[start..end]
            .into_iter()
            .filter(|&&(x, _, _)| !x.is_nan() && !x.is_infinite())
            .map(|&(x, _, _)| x)
            .collect::<Vec<f64>>().sort_by(|a, b| a.partial_cmp(b).unwrap());


        let median_x = sorted_x[half_window];
        result.push((median_x, rg[i].1, rg[i].2));

        let mut sorted_y = rg[start..end]
            .into_iter()
            .filter(|&(_, y, _)| !y.is_nan() && !y.is_infinite())
            .map(|&(_, y, _)| y)
            .collect::<Vec<f64>>();

        sorted_y.sort_unstable();

        let median_y = sorted_y[half_window];
        result.push((result[result.len()-1].0, median_y, result[result.len()-1].2));

        let mut sorted_z = rg[start..end]
            .into_iter()
            .filter(|&(_, _, z)| !z.is_nan() && !z.is_infinite())
            .map(|&(_, _, z)| z)
            .collect::<Vec<f64>>();

        sorted_z.sort_unstable();

        let median_z = sorted_z[half_window];
        result.push((result[result.len()-1].0, result[result.len()-1].1, median_z));
    }

    result
}