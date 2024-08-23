use crate::utils::u_serv::{Vector3d, Vector3dIndex};



fn calc_sma_for_axis(data_slice: &[Vector3d], axis: &Vector3dIndex, base: usize) -> f64 {
    data_slice.iter().map(|vec| vec.get_axis_val_by_index(axis)).sum::<f64>() / base as f64
}

pub fn parser_data_to_t_sma_xyz_list(data: &[Vector3d], base: usize) -> (Vec<f64>, Vec<Vector3d>) {
    let mut sma_t = Vec::new();
    let mut sma_vec_list:Vec<Vector3d> = Vec::new();

    for i in base..data.len() {
        sma_t.push(i as f64 * 0.005);
        let cur_data = &data[i - base..i];
        let cur_sma_x = calc_sma_for_axis(cur_data, &Vector3dIndex::X, base);
        let cur_sma_y = calc_sma_for_axis(cur_data, &Vector3dIndex::Y, base);
        let cur_sma_z = calc_sma_for_axis(cur_data, &Vector3dIndex::Z, base);

        sma_vec_list.push( Vector3d::new(cur_sma_x, cur_sma_y, cur_sma_z) )
    }

    (sma_t, sma_vec_list)
}


pub fn parser_data_to_sma_list(data: &[Vector3d], base: usize) -> (Vec<f64>, Vec<f64>) {
    let mut sma_t   =  Vec::new();
    let mut sma_vec =  Vec::new();

    let t_smaxyz =  parser_data_to_t_sma_xyz_list(data, base);

    for (i, t) in t_smaxyz.0.iter().enumerate() {
        sma_t.push(*t);
        sma_vec.push(t_smaxyz.1[i].magnitude());
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

