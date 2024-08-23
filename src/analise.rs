use crate::utils::u_serv::Vector3d;



pub fn parser_data_to_t_sma_xyz_list(data: &[Vector3d], base: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut sma_t = vec![0.];
    let mut sma_x_vec = vec![0.];
    let mut sma_y_vec = vec![0.];
    let mut sma_z_vec = vec![0.];

    // fn sub_calc_sma_for_axis(data_slice: &[Vector_3d], axis: usize) -> f64 {
    //     data_slice.iter().map(|vec| vec[axis]).sum()
    // }

    for i in base..data.len() {
        sma_t.push(i as f64 * 0.005);
        let cur_data = &data[i - base..i];
        
        let cur_sma_x: f64 = cur_data.iter().map(|vector| vector.x).sum();
        let cur_sma_y: f64 = cur_data.iter().map(|vector| vector.y).sum();
        let cur_sma_z: f64 = cur_data.iter().map(|vector| vector.z).sum();


        sma_x_vec.push(cur_sma_x / base as f64);
        sma_y_vec.push(cur_sma_y / base as f64);
        sma_z_vec.push(cur_sma_z / base as f64);
    }

    (sma_t, sma_x_vec, sma_y_vec, sma_z_vec)
}


pub fn parser_data_to_sma_list(data: &[Vector3d], base: usize) -> (Vec<f64>, Vec<f64>) {
    let mut sma_t   = vec![0.];
    let mut sma_vec = vec![0.];

    let t_smaxyz =  parser_data_to_t_sma_xyz_list(data, base);

    for (i, t) in t_smaxyz.0.iter().enumerate() {
        sma_t.push(*t);

        let scalar = f64::sqrt(
            t_smaxyz.1[i].powi(2) +
            t_smaxyz.2[i].powi(2) +
            t_smaxyz.3[i].powi(2)
        );

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







