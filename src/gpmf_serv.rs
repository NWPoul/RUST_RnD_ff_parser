use gpmf_rs::{
    Gpmf,
    SensorData,
    SensorType,
    // FourCC,
};


use std::path::PathBuf;

use crate::utils;
use crate::ConfigValues;








fn xyz_to_vec_tuple(data: &SensorData) -> Vec<(f64, f64, f64)> {
    let t_xyz: Vec<(f64, f64, f64)> = data.fields.iter().map(|f| (f.x,f.y,f.z)).collect();
    t_xyz
}


fn avg_xyz(data: &SensorData) -> (f64, f64) {
    let s_xyz: f64 = data.fields.iter().map(|f| f.x + f.y + f.z).sum();
    (
        (s_xyz / (3.0 * data.fields.len() as f64)).abs(),
        data.timestamp.unwrap_or_default().as_seconds_f64().trunc(),
    )
}

fn v_sum(v1: (f64, f64, f64), v2: (f64, f64, f64)) -> (f64, f64, f64) {
    (
        v1.0 + v2.0,
        v1.1 + v2.1,
        v1.2 + v2.2,
    )
}

fn _v_avg(v1: (f64, f64, f64), v2: (f64, f64, f64)) -> (f64, f64, f64) {
    (
        (v1.0 + v2.0) / 2.0,
        (v1.1 + v2.1) / 2.0,
        (v1.2 + v2.2) / 2.0,
    )
}

fn agr_v_avg(data: &SensorData) -> (f64, f64) {
    let (x, y, z) = data.fields.iter().fold(
        (0., 0., 0.),
        |prev, cur| {
            v_sum(prev, (cur.x, cur.y, cur.z))
        }
    );
    (
        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt(),
        data.timestamp.unwrap_or_default().as_seconds_f64().trunc(),
    )
}



fn max_xyz(data: &SensorData) -> (f64, f64) {
    let (x, y, z) = data.fields.iter().fold((0., 0., 0.), |acc, f| {
        (
            utils::abs_max(acc.0, f.x),
            utils::abs_max(acc.1, f.y),
            utils::abs_max(acc.2, f.z)
        )
    });
    (
        x.max(y).max(z),
        data.timestamp.unwrap_or_default().as_seconds_f64().trunc()
    )
}

fn max_skv_xyz(data: &SensorData) -> (f64, f64) {
    let total_squared_magnitude = data.fields.iter().fold(0., |acc, f| {
        let cur_skv = (f.x.powi(2) + f.y.powi(2) + f.z.powi(2)).sqrt();
        if cur_skv > acc {cur_skv} else {acc}
    });
    (
        total_squared_magnitude,
        data.timestamp.unwrap_or_default().as_seconds_f64().trunc(),
    )
}

fn _vec_skv_xyz(data: &SensorData) -> Vec<f64> {
    data.fields
        .iter()
        .map(|f| (f.x.powi(2) + f.y.powi(2) + f.z.powi(2)).sqrt())
        .collect()
}



pub fn get_device_info(gpmf: &Gpmf) {
    let device_name = gpmf.device_name();
    println!("device_name: {:?}", device_name);
    // let device_id   = gpmf.device_id().unwrap();
    // let optional_u32    : Option<u32>    = (&device_id).into();
    // let optional_four_cc: Option<FourCC> = (&device_id).into();
    // let optional_string : Option<String> = (&device_id).into();
    // println!("device_id: u32: {:?} FourCC: {:?} string: {:?}\n",
    //     optional_u32.unwrap_or_default(),
    //     optional_four_cc.unwrap_or_default(),
    //     optional_string.unwrap_or_default()
    // );
}



pub fn parse_sensor_data(
    gpmf: &Gpmf,
    config_values: &ConfigValues,
    src_file_path: &PathBuf,
) -> Result<(f64, f64), String> {
    let accel_data_list = gpmf.sensor(&SensorType::Accelerometer);

    let avg_accel_data_list = accel_data_list
        .iter()
        .map(|data| avg_xyz(data))
        .collect::<Vec<_>>();

    let v_agr_accel_data_list = accel_data_list
        .iter()
        .map(|data| agr_v_avg(data))
        .collect::<Vec<_>>();

    // let max_accel_data_list = accel_data_list
    //     .iter()
    //     .map(|data| max_xyz(data))
    //     .collect::<Vec<_>>();

    let max_log_acc_data_list = accel_data_list
        .iter()
        .map(|data| (max_xyz(data).1, max_xyz(data).0, max_skv_xyz(data).0, agr_v_avg(data).0/200.0))
        .collect::<Vec<_>>();


    let det_log_acc_vec_list = accel_data_list
        .iter()
        .flat_map(|data| xyz_to_vec_tuple(data))
        .collect::<Vec<_>>();

    crate::file_sys_serv::save_log_to_txt(&max_log_acc_data_list, src_file_path);
    // crate::file_sys_serv::save_log_to_txt(&v_agr_accel_data_list, src_file_path);
    crate::file_sys_serv::save_det_log_to_txt(&det_log_acc_vec_list, src_file_path);

    crate::analise::gnu_plot_test(&det_log_acc_vec_list);



    let max_avg_accel_data =
        avg_accel_data_list.iter().fold(
            (0., 0.),
            |acc, val| {
                if val.0 > acc.0 {
                    *val
                } else {
                    acc
                }
            },
        );

    if max_avg_accel_data.0 < config_values.min_accel_trigger {
        let err_msg = format!(
            "No deployment detected (min acc required is {:?})! max_datablock: {:?}\n",
            config_values.min_accel_trigger, max_avg_accel_data
        );
        eprintln!("{err_msg}");
        return Err(err_msg);
    }




    let deployment_time = max_avg_accel_data.1 + config_values.dep_time_correction;

    let mut target_start_time = deployment_time + config_values.time_start_offset;
    if target_start_time < 0.0 {
        target_start_time = 0.0;
    };

    let target_end_time = deployment_time + config_values.time_end_offset;

    println!(
        "\nmax_datablock: {:?} st_time: {:?} end_time: {:?} duration: {:?}\n",
        max_avg_accel_data,
        target_start_time,
        target_end_time,
        (target_end_time - target_start_time)
    );

    Ok((target_start_time, target_end_time))
}


