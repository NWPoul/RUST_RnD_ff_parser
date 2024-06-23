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



// fn avg_xyz(data: &SensorData) -> (f64, f64) {
//     let s_xyz: f64 = data.fields.iter().map(|f| f.x + f.y + f.z).sum();
//     (
//         (s_xyz / (3.0 * data.fields.len() as f64)).abs(),
//         data.timestamp.unwrap_or_default().as_seconds_f64().trunc(),
//     )
// }


pub fn xyz_to_tuples_list(data: &SensorData) -> Vec<(f64, f64, f64)> {
    let t_xyz: Vec<(f64, f64, f64)> = data.fields.iter().map(|f| (f.x,f.y,f.z)).collect();
    t_xyz
}

pub fn get_plain_xyz_tubles_list(accel_data_list: &Vec<SensorData>) -> Vec<(f64, f64, f64)> {
    let det_log_acc_vec_list = accel_data_list
        .iter()
        .flat_map(|data| xyz_to_tuples_list(data))
        .collect::<Vec<(f64, f64, f64)>>();
    det_log_acc_vec_list
}


pub fn get_sma_list(data: &[(f64, f64, f64)], base: usize) -> (Vec<f64>, Vec<f64>) {
    let mut sma_vec = vec![0.];
    let mut sma_t = vec![0.];

    for i in base..data.len() {
        let cur_data = &data[i - base..i];
        let cur_sma_x: f64 = cur_data.iter().map(|(x, _, _)| x).sum();
        let cur_sma_y: f64 = cur_data.iter().map(|(_, y, _)| y).sum();
        let cur_sma_z: f64 = cur_data.iter().map(|(_, _, z)| z).sum();
       
        sma_vec.push(
            f64::sqrt(cur_sma_x.powi(2) + cur_sma_y.powi(2) + cur_sma_z.powi(2)) / base as f64,
        );
        sma_t.push(i as f64 * 0.005);
    }

    (sma_t, sma_vec)
}

pub fn get_max_vec_data(data: (Vec<f64>, Vec<f64>)) -> (f64, f64) {
    let (max_t, max_vec) = data.1
        .iter()
        .enumerate()
        .max_by(
            |prev, next| prev.1.partial_cmp(next.1).unwrap_or(std::cmp::Ordering::Greater)
        )
        .unwrap_or((0,&0.));
    ((max_t as f64 * 0.005).round(), *max_vec)
}





pub fn parse_sensor_data(
    gpmf: &Gpmf,
    config_values: &ConfigValues,
    _src_file_path: &PathBuf,
) -> Result<GPMFParsedData, String> {
    let device_name = get_device_info(gpmf);

    let accel_data_list = gpmf.sensor(&SensorType::Accelerometer);



    let plain_xyz_list = get_plain_xyz_tubles_list(&accel_data_list);
    let sma_list = get_sma_list(&plain_xyz_list, 50);
    let (max_acc_time, max_acc_value) = get_max_vec_data(sma_list);

    // let avg_accel_data_list: Vec<(f64,f64)> = accel_data_list.iter()
    //     .map(|data| avg_xyz(data))
    //     .collect();

    // let max_avg_accel_data = avg_accel_data_list.iter()
    //     .max_by(
    //         |a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Greater)
    //     ).unwrap_or(&(0f64, 0f64))
    //     .to_owned();


    if max_acc_value < config_values.min_accel_trigger {
        let err_msg = format!(
            "CAM: {device_name} No deployment! (min acc required {}) detected: {}",
            config_values.min_accel_trigger,
            format_acc_datablock(&(max_acc_value, max_acc_time))
        );
        return Err(err_msg);
    }

    let deployment_time   = max_acc_time + config_values.dep_time_correction;
    let target_start_time = 0f64.max( deployment_time + config_values.time_start_offset );
    let target_end_time   = deployment_time           + config_values.time_end_offset;

    let parsed_data = GPMFParsedData {
        device_name : device_name,
        start_time  : target_start_time,
        end_time    : target_end_time,
        max_acc_data: (max_acc_value, max_acc_time),
    };

    Ok(parsed_data)
}


