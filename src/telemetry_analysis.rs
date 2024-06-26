
use crate::telemetry_parser_serv::parse_telemetry_from_file;
use crate::ConfigValues;



const SMA_BASE: usize = 50;




pub struct MaxAccData {
    acc : f64,
    time: f64,
}
pub struct FileTelemetryResult {
    pub device_name : String,
    pub start_time  : f64,
    pub end_time    : f64,
    pub max_acc_data: MaxAccData,
}
impl FileTelemetryResult {
    pub fn get_description(&self) -> String {
        format!(
            "CAM: {} Freefall: {}s-{}s ({}s) Max Acc: {}",
            self.device_name,
            self.start_time,
            self.end_time,
            self.end_time - self.start_time,
            format_acc_datablock(&self.max_acc_data)
        )
    }
}

pub fn format_camera_name(device_name: &str) -> String {
    device_name
        .replace(" HERO", "_")
        .replace(" Black", "")
        .replace(" ", "_")
}
pub fn format_acc_datablock(max_acc_data: &MaxAccData) -> String {
    format!("{:.2}m/s2  @ {}s", max_acc_data.acc, max_acc_data.time)
}



pub fn get_sma_list(data: &[(f64, f64, f64)], base: usize) -> Vec<f64> {
    let mut sma_vec = vec![0.];

    for i in base..data.len() {
        let cur_data = &data[i - base..i];
        let cur_sma_x: f64 = cur_data.iter().map(|(x, _, _)| x).sum();
        let cur_sma_y: f64 = cur_data.iter().map(|(_, y, _)| y).sum();
        let cur_sma_z: f64 = cur_data.iter().map(|(_, _, z)| z).sum();

        sma_vec.push(
            f64::sqrt(cur_sma_x.powi(2) + cur_sma_y.powi(2) + cur_sma_z.powi(2)) / base as f64,
        );
    }

    sma_vec
}


pub fn get_max_vec_data(data: Vec<f64>) -> MaxAccData {
    let (max_i, max_vec) = data
        .iter()
        .enumerate()
        .max_by(
            |prev, next| prev.1.partial_cmp(next.1).unwrap_or(std::cmp::Ordering::Greater)
        )
        .unwrap_or((0,&0.));
    MaxAccData{
        acc : *max_vec,
        time:  (max_i as f64 * 0.005).round(),
    }
}



pub fn get_result_metadata_for_file(
    input_file   : &str,
    config_values: &ConfigValues,
) -> Result<FileTelemetryResult, String> {
    let telemetry_data = parse_telemetry_from_file(input_file)?;
    let camera_name    = format_camera_name(&telemetry_data.cam_info);

    let telemetry_sma_acc_data = get_sma_list(&telemetry_data.acc_data, SMA_BASE);

    let max_acc_data = get_max_vec_data(telemetry_sma_acc_data);

    if max_acc_data.acc < config_values.min_accel_trigger {
        let err_msg = format!(
            "CAM: {} No deployment! (min acc required {}m/s2) detected: {}",
            camera_name,
            config_values.min_accel_trigger,
            format_acc_datablock(&max_acc_data)
        );
        return Err(err_msg);
    }

    let deployment_time   = max_acc_data.time         + config_values.dep_time_correction;
    let target_start_time = 0f64.max( deployment_time + config_values.time_start_offset );
    let target_end_time   = deployment_time           + config_values.time_end_offset;

    Ok(FileTelemetryResult{
        device_name : camera_name,
        start_time  : target_start_time,
        end_time    : target_end_time,
        max_acc_data: max_acc_data,
    })
}



