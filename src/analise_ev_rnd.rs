
use rustfft::{FftPlanner, num_complex::Complex};



#[derive(Debug)]
pub struct MaxAccData {
    acc : f64,
    time: f64,
}



pub fn get_max_vec_data(data: &[f64]) -> MaxAccData {
    let (max_i, max_vec) = data
        .iter()
        .enumerate()
        .max_by(
            |prev, next| prev.1.partial_cmp(next.1).unwrap_or(std::cmp::Ordering::Greater)
        )
        .unwrap_or((0,&0.));
    MaxAccData{
        acc : *max_vec,
        time:  (max_i as f64 * 0.005).round(),
    }
}




pub fn stft(data: &[f64], window_size: usize, hop_size: usize) -> Vec<Vec<Complex<f64>>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(window_size);
    let mut output = Vec::new();

    // Ensure hop_size is at least 1
    let effective_hop_size = std::cmp::max(1, hop_size);

    for i in (0..data.len()).step_by(effective_hop_size) {
        let mut window = vec![Complex::new(0.0, 0.0); window_size];
        for (j, &sample) in data[i..].iter().take(window_size).enumerate() {
            window[j] = Complex::new(sample, 0.0);
        }
        fft.process(&mut window);
         // Exclude the 0 Hz component by skipping the first element
        output.push(window[1..].to_vec());
    }

    output
}

pub fn stft_result_analise(sma_data: &[f64], sma_base: usize) {
    let window_size  = 200; 
    let hop_size     = 200; 
    let sample_rate  = 200.0; 
    let freq_resolution = sample_rate / window_size as f64;
    let time_resolution = hop_size as f64 / sample_rate;

    let old_result = get_max_vec_data(sma_data);

    let stft_result = stft(sma_data, window_size, hop_size);
    let mut top_5: Vec<(f64, usize, f64)> = Vec::with_capacity(5);

    for (time_idx, spectrum) in stft_result.iter().enumerate() {
        let mut max_magnitude_in_spectrum = 0.0;
        let mut max_frequency_in_spectrum = 0.0;

        for (freq_idx, &complex_val) in spectrum.iter().enumerate() {
            let magnitude = complex_val.norm();
            if magnitude > max_magnitude_in_spectrum {
                max_magnitude_in_spectrum = magnitude;
                max_frequency_in_spectrum = freq_idx as f64 * freq_resolution;
            }
        }

        if top_5.len() < 5 {
            top_5.push((max_magnitude_in_spectrum, time_idx, max_frequency_in_spectrum));
            top_5.sort_by_key(|&(_, time, _)| time);
        } else if time_idx > top_5[4].1 {
            top_5[4] = (max_magnitude_in_spectrum, time_idx, max_frequency_in_spectrum);
            top_5.sort_by_key(|&(_, time, _)| time);
        }
    }

    for (i, (magnitude, time_idx, frequency)) in top_5.iter().enumerate() {
        println!("sma_base: {}", sma_base);
        println!("OLD_RESULT: Time {}, MaxAcc {}", old_result.time, old_result.acc);
        println!("NEW_RESULT: Top {}: Time: {} seconds, Magnitude: {}, Frequency: {} Hz", 
                i + 1, *time_idx as f64 * time_resolution, magnitude, frequency);
        };
        println!("");
}

