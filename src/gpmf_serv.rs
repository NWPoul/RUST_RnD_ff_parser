use gpmf_rs::{
    Gpmf,
    SensorData,
    SensorType,
    // FourCC,
};


use std::path::PathBuf;

use crate::ConfigValues;

use crate::analise::{
    get_max_vec_data,
    get_sma_list,
};



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
    // remove_symbols(&device_name, "[\"\\]")
    device_name
}


pub fn gpmf_xyz_to_tuples_list(data: &SensorData) -> Vec<(f64, f64, f64)> {
    let t_xyz: Vec<(f64, f64, f64)> = data.fields.iter().map(|f| (f.x,f.y,f.z)).collect();
    t_xyz
}

pub fn gpmf_get_plain_xyz_tubles_list(accel_data_list: &Vec<SensorData>) -> Vec<(f64, f64, f64)> {
    let det_log_acc_vec_list = accel_data_list
        .iter()
        .flat_map(|data| gpmf_xyz_to_tuples_list(data))
        .collect::<Vec<(f64, f64, f64)>>();
    det_log_acc_vec_list
}









pub fn get_sensor_data(
    gpmf: &Gpmf,
    _config_values: &ConfigValues,
    _src_file_path: &PathBuf,
) -> (Vec<(f64, f64, f64)>, (Vec<f64>, Vec<f64>)) {
    let accel_data_list = gpmf.sensor(&SensorType::Accelerometer);

    let plain_xyz_list = gpmf_get_plain_xyz_tubles_list(&accel_data_list);
    let sma_list = get_sma_list(&plain_xyz_list, 50);

    (
        plain_xyz_list,
        sma_list,
    )
}



pub fn parse_sensor_data(
    gpmf: &Gpmf,
    config_values: &ConfigValues,
    src_file_path: &PathBuf,
) -> Result<(f64, f64), String> {


    let (
        plain_xyz_list,
        sma_list,
    ) = get_sensor_data(
        gpmf, config_values, src_file_path,
    );

    let (max_acc_time, max_acc_value) = get_max_vec_data(sma_list);

    // crate::file_sys_serv::save_log_to_txt(&v_agr_accel_data_list, src_file_path);
    crate::file_sys_serv::save_det_log_to_txt(&plain_xyz_list, src_file_path);

    crate::analise::gnu_plot_xyz(&plain_xyz_list);




    if max_acc_value < config_values.min_accel_trigger {
        let err_msg = format!(
            "No deployment detected (min acc required is {:?})! max_datablock: {:?}\n",
            config_values.min_accel_trigger, max_acc_value
        );
        eprintln!("{err_msg}");
        return Err(err_msg);
    }




    let deployment_time = max_acc_time + config_values.dep_time_correction;

    let mut target_start_time = deployment_time + config_values.time_start_offset;
    if target_start_time < 0.0 {
        target_start_time = 0.0;
    };

    let target_end_time = deployment_time + config_values.time_end_offset;

    println!(
        "max_datablock: {:?} st_time: {:?} end_time: {:?} duration: {:?}\n",
        max_acc_value,
        target_start_time,
        target_end_time,
        (target_end_time - target_start_time)
    );

    Ok((target_start_time, target_end_time))
}


