use gpmf_rs::{
    Gpmf, FourCC, SensorData, SensorType
};


use crate::utils;
use crate::ConfigValues;



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


pub fn get_device_info(gpmf: &Gpmf) {
    let device_name = gpmf.device_name();
    let device_id   = gpmf.device_id().unwrap();

    let optional_u32    : Option<u32>    = (&device_id).into();
    let optional_four_cc: Option<FourCC> = (&device_id).into();
    let optional_string : Option<String> = (&device_id).into();

    println!("device_name: {:?}", device_name);
    println!("device_id:
        u32: {:?}
        FourCC: {:?}
        string: {:?}\n",
        optional_u32.unwrap_or_default(),
        optional_four_cc.unwrap_or_default(),
        optional_string.unwrap_or_default()
    );
}



pub fn parse_sensor_data(
    gpmf: &Gpmf,
    config_values: &ConfigValues
) -> Result<(f64, f64), Result<(), std::io::Error>> {
    let sensor_data_list = gpmf.sensor(&SensorType::Accelerometer);
    let max_accel_data_list = sensor_data_list
        .iter()
        .map(|data| max_xyz(data))
        .collect::<Vec<_>>();
    let max_accel_data =
        max_accel_data_list.iter().fold(
            (0., 0.),
            |acc, val| {
                if val.0 > acc.0 {
                    *val
                } else {
                    acc
                }
            },
        );
    if max_accel_data.0 < config_values.min_accel_trigger {
        println!(
            "No deployment detected (min acc required is {:?})! max_datablock: {:?}\n",
            config_values.min_accel_trigger,
            max_accel_data
        );
        return Err(Ok(()));
    }
    let deployment_time = max_accel_data.1 + config_values.dep_time_correction;

    let mut target_start_time = deployment_time + config_values.time_start_offset;
    if target_start_time < 0.0 {
        target_start_time = 0.0;
    };

    let target_end_time   = deployment_time + config_values.time_end_offset;

    println!(
        "max_datablock: {:?} st_time: {:?} end_time: {:?} duration: {:?}\n",
        max_accel_data, target_start_time, target_end_time, (target_end_time - target_start_time)
    );

    Ok((target_start_time, target_end_time))
}


