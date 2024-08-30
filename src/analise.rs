
use crate::utils::u_serv::{Vector3d, Index3D};



fn calc_sma_spread_value(data_slice: &[f64]) -> (f64, f64) {
    let base = data_slice.len() as f64;
    let sma = data_slice.iter().sum::<f64>() / base;
    let spr = data_slice.iter().map(|&x| (x - sma).powi(2)).sum::<f64>() / (base - 1.0);
    (sma, spr)
}
pub fn data_to_sma_spread_data(data: &[f64], base: usize) -> (Vec<f64>, Vec<f64>) {
    let mut sma_vec_list:Vec<f64> = Vec::new();
    let mut spr_vec_list:Vec<f64> = Vec::new();

    for i in base..data.len() {
        let cur_data = &data[i - base..i];
        let cur_stat = calc_sma_spread_value(cur_data);
        sma_vec_list.push( cur_stat.0 );
        spr_vec_list.push( cur_stat.1 );
    }
    (sma_vec_list, spr_vec_list)
}


fn calc_sma_spread_for_axis(data_slice: &[Vector3d], axis: &Index3D) -> (f64, f64) {
    let base = data_slice.len() as f64;
    let sma = data_slice.iter().map(|vec| vec.get_axis_val_by_index(axis)).sum::<f64>() / base;
    let spr = data_slice.iter().map(|vec| (vec.get_axis_val_by_index(axis) - sma).powi(2)).sum::<f64>() / (base-1.0);
    (sma, spr)
}
fn calc_sma_spread_for_v3dmagnitude(data_slice: &[Vector3d]) -> (f64, f64) {
    let base = data_slice.len() as f64;
    let sma = data_slice.iter().map(|vec| vec.magnitude()).sum::<f64>() / base;
    let spr = data_slice.iter().map(|vec| (vec.magnitude() - sma).powi(2)).sum::<f64>() / (base-1.0);
    (sma, spr)
}

pub fn v3d_list_to_ts_sma_v3d_list(data: &[Vector3d], base: usize) -> (Vec<f64>, Vec<Vector3d>) {
    let mut sma_t = Vec::new();
    let mut sma_vec_list:Vec<Vector3d> = Vec::new();
    let mut spr_vec_list:Vec<Vector3d> = Vec::new();

    for i in base..data.len() {
        sma_t.push(i as f64 * 0.005);
        let cur_data = &data[i - base..i];
        let cur_stat_x = calc_sma_spread_for_axis(cur_data, &Index3D::X);
        let cur_stat_y = calc_sma_spread_for_axis(cur_data, &Index3D::Y);
        let cur_stat_z = calc_sma_spread_for_axis(cur_data, &Index3D::Z);

        sma_vec_list.push( Vector3d::new(cur_stat_x.0, cur_stat_y.0, cur_stat_z.0) );
        spr_vec_list.push( Vector3d::new(cur_stat_x.1, cur_stat_y.1, cur_stat_z.1) );
    }
    (sma_t, sma_vec_list)
    // abs_sma_xyz(sma_t, sma_vec_list)
}

pub fn v3d_list_to_magnitude_smaspr_list(data: &[Vector3d], base: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut sma_t   =  Vec::new();
    let mut sma_vec =  Vec::new();
    let mut spr_vec =  Vec::new();


    for i in base..data.len() {
        sma_t.push(i as f64 * 0.005);
        let cur_data = &data[i - base..i];
        let cur_stat = calc_sma_spread_for_v3dmagnitude(cur_data);
        sma_vec.push( cur_stat.0 );
        spr_vec.push( cur_stat.1 );
    }
    
    (sma_t, sma_vec, spr_vec)
}

pub fn v3d_list_to_scalar_sma_list(data: &[Vector3d], base: usize, reducer: impl Fn(&Vector3d) -> f64 ) -> (Vec<f64>, Vec<f64>) {
    let mut sma_t   =  Vec::new();
    let mut sma_vec =  Vec::new();

    let ts_sma_v3d_list = v3d_list_to_ts_sma_v3d_list(data, base);

    for (i, t) in ts_sma_v3d_list.0.iter().enumerate() {
        sma_t.push(*t);
        sma_vec.push(reducer(&ts_sma_v3d_list.1[i]));
    }
    (sma_t, sma_vec)
}
pub fn v3d_list_to_magnitude_sma_list(data: &[Vector3d], base: usize) -> (Vec<f64>, Vec<f64>) {
    let (sma_t, sma_vec) = v3d_list_to_scalar_sma_list(data, base, Vector3d::magnitude );
    (sma_t, sma_vec)
}
pub fn v3d_list_to_plainsum_sma_list(data: &[Vector3d], base: usize) -> (Vec<f64>, Vec<f64>) {
    let (sma_t, sma_vec) = v3d_list_to_scalar_sma_list(data, base, Vector3d::plain_sum );
    (sma_t, sma_vec)
}

pub fn calc_velocity_arr(acc_data: &[Vector3d], tick: &f64) -> (Vec<Vector3d>, Vec<f64>){
    let mut res =  vec![Vector3d::new(0.0, 0.0, 0.0)];
    let mut mag = vec![0.];

    for (i, acc_v3d) in acc_data.iter().enumerate() {
        let cur_add_to_velosity = acc_v3d.apply_for_all_axis(
            |val| val*tick
        );
        let new_velocity_v3d = res.last().unwrap().add_v3d(&cur_add_to_velosity);
        res.push(new_velocity_v3d.clone());
        mag.push(mag.last().unwrap() + cur_add_to_velosity.magnitude() - 9.81*tick);
    }
    (res, mag)
}

pub fn abs_sma_xyz(t: Vec<f64>, sma_xyz_data: Vec<Vector3d>) -> (Vec<f64>, Vec<Vector3d>) {
    // let mut abs_sma_xyz:Vec<Vector3d> =  Vec::new();
    let abs_sma_xyz =  sma_xyz_data.iter().map(
        |vector| vector.apply_for_all_axis(  f64::abs  )
    ).collect();
    (t, abs_sma_xyz)
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

