
use crate::utils::u_serv::{Vector3d, Axis3d};


#[derive(Default)]
pub struct StatVals {
    pub sma: f64,
    pub spr: f64,
}

#[derive(Default, Debug, PartialEq)]
pub struct StatValsArr {
    pub sma: Vec<f64>,
    pub spr: Vec<f64>,
}







fn calc_elem_stat_vals(data: &[f64]) -> StatVals {
    let base = data.len() as f64;
    let sma  = data.iter().sum::<f64>() / base;
    let spr  = data.iter().map(|&x| (x - sma).powi(2)).sum::<f64>() / (base - 1.0);
    StatVals{sma, spr}
}

pub fn data_to_stat_vals_arr(data: &[f64], base: usize) -> StatValsArr {
    let base = std::cmp::min(data.len() / 2, base);

    let initial_stat = calc_elem_stat_vals(&data[0..base]);
    let mut stat_vecs = StatValsArr{
        sma: vec![initial_stat.sma],
        // spr: vec![initial_stat.spr],
        spr: data[0..base].iter().scan(0.0, |state, &x| {
            *state += x;
            Some(*state)
        }).collect()
    };

    // for i in (base + 1)..data.len() {
    //     let cur_data = &data[i - base..i];
    //     let cur_stat = calc_elem_stat_vals(cur_data);
    //     stat_vecs.sma.push( cur_stat.sma );
    //     stat_vecs.spr.push( cur_stat.spr );
    // }

    // let mut new_sma: Vec<f64> = Vec::new();
    // let step_stat_iter = data.iter()
    //     .step_by(base)
    //     .scan(0.0, |state, &x| {
    //         *state += x;
    //         Some(*state)
    //     });
    // let _new_stat_t: Vec<Vec<f64>> = step_stat_iter.map( |block_val| {
    //     let add_arr = vec![block_val; base];
    //     new_sma.extend_from_slice(&add_arr);
    //     add_arr
    // }).collect();



    for i in base..data.len() {
        let cur_sma = stat_vecs.sma.last().unwrap();
        let cur_spr = stat_vecs.spr.last().unwrap();

        let new_sma = cur_sma + ((data[i] - data[i - base]) / base as f64);
        let new_spr = cur_spr + data[i];
        // let new_spr = cur_spr + new_sma;

        stat_vecs.sma.push( new_sma );
        stat_vecs.spr.push( new_spr );
    }
    
    // dbg!(stat_vecs.sma.len() as f64 - new_sma.len() as f64);
    StatValsArr{
        sma: stat_vecs.sma,
        spr: stat_vecs.spr,
    }
    // stat_vecs
}



fn calc_stat_vals_for_axis(data_slice: &[Vector3d], axis: &Axis3d) -> StatVals {
    let base = data_slice.len() as f64;
    let sma  = data_slice.iter().map(|vec| vec.get_axis_val(axis)).sum::<f64>() / base;
    let spr  = data_slice.iter().map(|vec| (vec.get_axis_val(axis) - sma).powi(2)).sum::<f64>() / (base-1.0);
    StatVals{sma, spr}
}
fn calc_stat_vals_for_v3dmagnitude(data_slice: &[Vector3d]) -> StatVals {
    let base = data_slice.len() as f64;
    let sma  = data_slice.iter().map(|vec| vec.magnitude()).sum::<f64>() / base;
    let spr  = data_slice.iter().map(|vec| (vec.magnitude() - sma).powi(2)).sum::<f64>() / (base-1.0);
    StatVals{sma, spr}
}



pub fn v3d_list_to_ts_sma_v3d_list_old(data: &[Vector3d], base: usize) -> (Vec<f64>, Vec<Vector3d>) {
    let mut sma_t = Vec::new();
    let mut sma_vec_list:Vec<Vector3d> = Vec::new();
    let mut spr_vec_list:Vec<Vector3d> = Vec::new();

    for i in base..data.len() {
        sma_t.push(i as f64 * 0.005);
        let cur_data = &data[i - base..i];
        let cur_stat_x = calc_stat_vals_for_axis(cur_data, &Axis3d::X);
        let cur_stat_y = calc_stat_vals_for_axis(cur_data, &Axis3d::Y);
        let cur_stat_z = calc_stat_vals_for_axis(cur_data, &Axis3d::Z);

        sma_vec_list.push( Vector3d::new(cur_stat_x.sma, cur_stat_y.sma, cur_stat_z.sma) );
        spr_vec_list.push( Vector3d::new(cur_stat_x.spr, cur_stat_y.spr, cur_stat_z.spr) );
    }
    (sma_t, sma_vec_list)
    // abs_sma_xyz(sma_t, sma_vec_list)
}
pub fn v3d_list_to_ts_sma_v3d_list(data: &[Vector3d], base: usize) -> (Vec<f64>, Vec<Vector3d>) {
    let mut sma_t = Vec::new();
    let mut sma_vec_list:Vec<Vector3d> = Vec::new();
    let mut spr_vec_list:Vec<Vector3d> = Vec::new();

    // let mut axis_data: (Vec<f64>, Vec<f64>, Vec<f64>) = (Vec::new(), Vec::new(), Vec::new());
    // for axis in Vector3d::axis_iter() {
    //     // let j = Vector3d::AXIS[0];
    // }

    for i in base..data.len() {
        sma_t.push(i as f64 * 0.005);
        let cur_data = &data[i - base..i];
        let cur_stat_x = calc_stat_vals_for_axis(cur_data, &Axis3d::X);
        let cur_stat_y = calc_stat_vals_for_axis(cur_data, &Axis3d::Y);
        let cur_stat_z = calc_stat_vals_for_axis(cur_data, &Axis3d::Z);

        sma_vec_list.push( Vector3d::new(cur_stat_x.sma, cur_stat_y.sma, cur_stat_z.sma) );
        spr_vec_list.push( Vector3d::new(cur_stat_x.spr, cur_stat_y.spr, cur_stat_z.spr) );
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
        let cur_stat = calc_stat_vals_for_v3dmagnitude(cur_data);
        sma_vec.push( cur_stat.sma );
        spr_vec.push( cur_stat.spr );
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
    let (sma_t, sma_vec) = v3d_list_to_scalar_sma_list(data, base, Vector3d::sum_axis );
    (sma_t, sma_vec)
}

pub fn calc_velocity_arr(acc_data: &[Vector3d], tick: &f64) -> (Vec<Vector3d>, Vec<f64>){
    let mut res = vec![Vector3d::new(0.0, 0.0, 0.0)];
    let mut mag = vec![0.];

    for acc_v3d in acc_data.iter() {
        let cur_add_to_velosity = acc_v3d.apply_for_all_axis(
            |val| val*tick
        );

        let new_velocity_v3d = res.last().unwrap().v3add(&cur_add_to_velosity);
        res.push(new_velocity_v3d.clone());
        mag.push(mag.last().unwrap() + cur_add_to_velosity.magnitude() - 9.81*tick);
    }
    (res, mag)
}

pub fn abs_sma_xyz(t: Vec<f64>, sma_xyz_data: Vec<Vector3d>) -> (Vec<f64>, Vec<Vector3d>) {
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


