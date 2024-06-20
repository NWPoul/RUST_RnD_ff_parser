use gpmf_rs::{
    Gpmf,
    SensorData,
    SensorType,
    // FourCC,
};


use std::path::PathBuf;


use crate::ConfigValues;
use crate::utils::u_serv::remove_symbols;





#[derive(Debug)]
pub struct GPMFParsedData {
    pub device_name : String,
    pub start_time  : f64,
    pub end_time    : f64,
    pub max_acc_data: (f64, f64),
}

impl GPMFParsedData {
    pub fn get_description(&self) -> String {
        format!(
            "CAM: {} Freefall: {}s-{}s ({}s) Max Acc: {}",
            self.device_name,
            self.start_time,
            self.end_time,
            self.end_time - self.start_time,
            format_acc_datablock(&(self.max_acc_data.0, self.max_acc_data.1))
        )
    }
}


pub fn format_acc_datablock(acc_n_time: &(f64, f64)) -> String {
    format!("{:.2}m/s2  @ {}s", acc_n_time.0, acc_n_time.1)
}


pub fn get_device_info(gpmf: &Gpmf) -> String {
    let device_name = gpmf.device_name();
    let device_name = format!("'{:?}'", device_name);
    remove_symbols(&device_name, "[\"\\]")
}



fn avg_xyz(data: &SensorData) -> (f64, f64) {
    let s_xyz: f64 = data.fields.iter().map(|f| f.x + f.y + f.z).sum();
    (
        (s_xyz / (3.0 * data.fields.len() as f64)).abs(),
        data.timestamp.unwrap_or_default().as_seconds_f64().trunc(),
    )
}



pub fn parse_sensor_data(
    gpmf: &Gpmf,
    config_values: &ConfigValues,
    _src_file_path: &PathBuf,
) -> Result<GPMFParsedData, String> {
    let device_name = get_device_info(gpmf);

    let accel_data_list = gpmf.sensor(&SensorType::Accelerometer);

    let avg_accel_data_list: Vec<(f64,f64)> = accel_data_list.iter()
        .map(|data| avg_xyz(data))
        .collect();

    let max_avg_accel_data = avg_accel_data_list.iter()
        .max_by(
            |a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Greater)
        ).unwrap_or(&(0f64, 0f64))
        .to_owned();


    if max_avg_accel_data.0 < config_values.min_accel_trigger {
        let err_msg = format!(
            "CAM: {device_name} No deployment! (min acc required {}) detected: {}",
            config_values.min_accel_trigger,
            format_acc_datablock(&max_avg_accel_data)
        );
        return Err(err_msg);
    }

    let deployment_time   = max_avg_accel_data.1      + config_values.dep_time_correction;
    let target_start_time = 0f64.max( deployment_time + config_values.time_start_offset );
    let target_end_time   = deployment_time           + config_values.time_end_offset;

    let parsed_data = GPMFParsedData {
        device_name : device_name,
        start_time  : target_start_time,
        end_time    : target_end_time,
        max_acc_data: max_avg_accel_data,
    };

    // println!(
    //     "max_datablock: {:?} st_time: {:?} end_time: {:?} duration: {:?}\n",
    //     format_acc_datablock(&max_avg_accel_data),
    //     target_start_time,
    //     target_end_time,
    //     (target_end_time - target_start_time)
    // );

    Ok(parsed_data)
}


