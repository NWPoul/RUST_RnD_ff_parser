use gpmf_rs::{
    Gpmf,
    SensorData,
    SensorType,
    // FourCC,
};


use std::path::PathBuf;

// use crate::utils;
use crate::ConfigValues;




fn avg_xyz(data: &SensorData) -> (f64, f64) {
    let s_xyz: f64 = data.fields.iter().map(|f| f.x + f.y + f.z).sum();
    (
        (s_xyz / (3.0 * data.fields.len() as f64)).abs(),
        data.timestamp.unwrap_or_default().as_seconds_f64().trunc(),
    )
}



pub fn get_device_info(gpmf: &Gpmf) {
    let device_name = gpmf.device_name();
    println!("device_name: {:?}", device_name);
}



pub fn parse_sensor_data(
    gpmf: &Gpmf,
    config_values: &ConfigValues,
    _src_file_path: &PathBuf,
) -> Result<(f64, f64), String> {
    let accel_data_list = gpmf.sensor(&SensorType::Accelerometer);

    let avg_accel_data_list = accel_data_list
        .iter()
        .map(|data| avg_xyz(data))
        .collect::<Vec<_>>();

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
        "max_datablock: {:?} st_time: {:?} end_time: {:?} duration: {:?}\n",
        max_avg_accel_data,
        target_start_time,
        target_end_time,
        (target_end_time - target_start_time)
    );

    Ok((target_start_time, target_end_time))
}


