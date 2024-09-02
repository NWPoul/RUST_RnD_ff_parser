
use crate::utils::u_serv::{Vector3d, Index3D};

#[derive(Default)]
pub struct StatVals {
    pub sma: f64,
    pub spr: f64,
}

#[derive(Default, Debug, PartialEq)]
pub struct StatVecs {
    pub sma: Vec<f64>,
    pub spr: Vec<f64>,
}







fn calc_elem_stat_vals(data_slice: &[f64]) -> StatVals {
    let base = data_slice.len() as f64;
    let sma  = data_slice.iter().sum::<f64>() / base;
    let spr  = data_slice.iter().map(|&x| (x - sma).powi(2)).sum::<f64>() / (base - 1.0);
    StatVals{sma, spr}
}

pub fn data_to_stat_vecs_new(data: &[f64], base: usize) -> StatVecs {
    let initial_stat = calc_elem_stat_vals(&data[0..base]);
    let mut stat_vecs = StatVecs{
        sma: vec![initial_stat.sma],
        spr: vec![initial_stat.spr],
    };

    for i in (base + 1)..data.len() {
        let cur_data = &data[i - base..i];
        let cur_stat = calc_elem_stat_vals(cur_data);
        stat_vecs.sma.push( cur_stat.sma );
        stat_vecs.spr.push( cur_stat.spr );
    }
    stat_vecs
}

pub fn data_to_stat_vecs(data: &[f64], base: usize) -> StatVecs {
    let mut stat_vecs = StatVecs::default();

    for i in base..data.len() {
        let cur_data = &data[i - base..i];
        let cur_stat = calc_elem_stat_vals(cur_data);
        stat_vecs.sma.push( cur_stat.sma );
        stat_vecs.spr.push( cur_stat.spr );
    }
    println!("CHECK NEW SMA{:?}",
        data_to_stat_vecs_new(data, base) == stat_vecs
    );
    stat_vecs
}


fn calc_stat_vals_for_axis(data_slice: &[Vector3d], axis: &Index3D) -> StatVals {
    let base = data_slice.len() as f64;
    let sma  = data_slice.iter().map(|vec| vec.get_axis_val_by_index(axis)).sum::<f64>() / base;
    let spr  = data_slice.iter().map(|vec| (vec.get_axis_val_by_index(axis) - sma).powi(2)).sum::<f64>() / (base-1.0);
    StatVals{sma, spr}
}
fn calc_stat_vals_for_v3dmagnitude(data_slice: &[Vector3d]) -> StatVals {
    let base = data_slice.len() as f64;
    let sma  = data_slice.iter().map(|vec| vec.magnitude()).sum::<f64>() / base;
    let spr  = data_slice.iter().map(|vec| (vec.magnitude() - sma).powi(2)).sum::<f64>() / (base-1.0);
    StatVals{sma, spr}
}



pub fn v3d_list_to_ts_sma_v3d_list(data: &[Vector3d], base: usize) -> (Vec<f64>, Vec<Vector3d>) {
    let mut sma_t = Vec::new();
    let mut sma_vec_list:Vec<Vector3d> = Vec::new();
    let mut spr_vec_list:Vec<Vector3d> = Vec::new();

    for i in base..data.len() {
        sma_t.push(i as f64 * 0.005);
        let cur_data = &data[i - base..i];
        let cur_stat_x = calc_stat_vals_for_axis(cur_data, &Index3D::X);
        let cur_stat_y = calc_stat_vals_for_axis(cur_data, &Index3D::Y);
        let cur_stat_z = calc_stat_vals_for_axis(cur_data, &Index3D::Z);

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




#[test]
fn test_sma_calculation() {
    let test_data: Vec<f64> = vec![
        1., 2., 3., 4., 5.,
        10., 20., 30., 40., 50.,
        110., 210., 301., 410., 501.,
    ];

    let base = 7 as usize;
    let calc     = data_to_stat_vecs(&test_data, base);
    let new_calc = data_to_stat_vecs_new(&test_data, base);
    
    // println!("calc {:?} newcalc{:?} eq {:?}", calc, new_calc, new_calc == calc);

    assert_eq!(new_calc, calc);
}