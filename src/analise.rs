

fn ends_with_one(value: usize) -> bool {
    let value_str = value.to_string();
    value_str.chars().last() == Some('1')
}


pub fn parser_data_to_sma_list(data: &[(f64, f64, f64)], base: usize) -> (Vec<f64>, Vec<f64>) {

    let sample_rate   : f32 = 200.0;
    let sensitivity   : f32 = 0.0001;

    let data = if ends_with_one(base) {
        &crate::analise_log_v::smooth_vector_components(
            data,
            sample_rate*((base-1) as f32),
            sample_rate, 
            sensitivity,
        )
    } else {
        data
    };

    let base = if ends_with_one(base) {1} else {base};

    let mut sma_vec = vec![0.];
    let mut sma_t = vec![0.];

    for i in base..data.len() {
        sma_t.push(i as f64 * 0.005);
        let cur_data = &data[i - base..i];
        let cur_sma_x: f64 = cur_data.iter().map(|(x, _, _)| x).sum();
        let cur_sma_y: f64 = cur_data.iter().map(|(_, y, _)| y).sum();
        let cur_sma_z: f64 = cur_data.iter().map(|(_, _, z)| z).sum();

        let scalar = f64::sqrt(
            cur_sma_x.powi(2) +
            cur_sma_y.powi(2) +
            cur_sma_z.powi(2)
        ) / base as f64;

        sma_vec.push(scalar);
    }

    (sma_t, sma_vec)
}

pub fn get_max_vec_data(t_acc_data: &(Vec<f64>, Vec<f64>)) -> (f64, f64) {
    let (max_i, max_vec) = t_acc_data.1
        .iter()
        .enumerate()
        .max_by(
            |prev, next|
                prev.1
                    .partial_cmp(next.1)
                    .unwrap_or(std::cmp::Ordering::Greater)
        )
        .unwrap_or((0,&0.));
    ((t_acc_data.0[max_i]).round(), *max_vec)
}







