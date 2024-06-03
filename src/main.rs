// #![allow(unused)] // For beginning only.
// use crate::prelude::*;

mod error;
mod prelude;
mod utils;

// use std::path::Path;
// use std::io::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
// use std::io::Read;
// use std::time::Duration;
// use toml::from_str;

// use serde::Deserialize;
use config::{Config, File as Cfg_file};
// use std::path::PathBuf;

use gpmf_rs::{
    Gpmf, FourCC, SensorData, SensorType
};



const DEF_DIR    : &str        = ".";
const DEF_POSTFIX: &str        = "_FFCUT";
const DEF_PROMPT_FLIGHT: bool  = false;

const DEP_TIME_CORRECTION: f64 = 2.0;
const TIME_START_OFFSET  : f64 = -60.0;
const TIME_END_OFFSET    : f64 = 3.0;

const MIN_ACCEL_TRIGGER  : f64 = 50.0;

struct ConfigValues {
    srs_dir_path: String,
    dest_dir_path: String,
    ffmpeg_dir_path: String,
    output_file_postfix: String,
    dep_time_correction: f64,
    time_start_offset: f64,
    time_end_offset: f64,
    min_accel_trigger: f64,
}

fn __promt_to_exit(msg: &str) {
    println!("{}\nPress 'enter' to exit...\n", {msg});
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}


// impl Default for Config {
//     fn default() -> Self {
//         Self {
//             ff_duration:         FF_DURATION,
//             dep_time_correction: DEP_TIME_CORRECTION,
//             time_start_offset:   TIME_START_OFFSET,
//             time_end_offset:     TIME_END_OFFSET,
//             min_accel_trigger:   MIN_ACCEL_TRIGGER,
//         }
//     }
// }



fn get_config_values() -> ConfigValues {
    let mut settings = Config::default();

    if let Err(e) = settings.merge(Cfg_file::with_name("config.toml")) {
        println!("Failed to load configuration file: {}", e);
        println!("default configuration used");
    }
    println!("Config loaded from file");

    let srs_dir_path = settings
        .get::<String>("srs_dir_path")
        .unwrap_or(DEF_DIR.into());
    let dest_dir_path = settings
        .get::<String>("dest_dir_path")
        .unwrap_or(DEF_DIR.into());
    let ffmpeg_dir_path = settings
        .get::<String>("ffmpeg_dir_path")
        .unwrap_or(DEF_DIR.into());
    let output_file_postfix = settings
        .get::<String>("output_file_postfix")
        .unwrap_or(DEF_POSTFIX.into());

    let dep_time_correction = settings
        .get::<f64>("dep_time_correction")
        .unwrap_or(DEP_TIME_CORRECTION);
    let time_start_offset = settings
        .get::<f64>("time_start_offset")
        .unwrap_or(TIME_START_OFFSET);
    let time_end_offset = settings
        .get::<f64>("time_end_offset")
        .unwrap_or(TIME_END_OFFSET);
    let min_accel_trigger = settings
        .get::<f64>("min_accel_trigger")
        .unwrap_or(MIN_ACCEL_TRIGGER);

    println!("Dir paths\n src: {}\n dest: {}\n ffmpeg: {}",
        srs_dir_path, dest_dir_path, ffmpeg_dir_path);
    println!("Dep Time Correction: {}", dep_time_correction);
    println!("Time Start Offset: {}", time_start_offset);
    println!("Time End Offset: {}", time_end_offset);
    println!("Min Accel Trigger: {}", min_accel_trigger);
    println!("Output postfix: {}", output_file_postfix);

    println!("");

    ConfigValues {
        srs_dir_path,
        dest_dir_path,
        ffmpeg_dir_path,
        dep_time_correction,
        time_start_offset,
        time_end_offset,
        min_accel_trigger,
        output_file_postfix,
    }
}

fn m_max(f_new: f64, f_prev: f64) -> f64 {
    if f_new.abs() > f_prev.abs() {
        f_new.abs()
    } else {
        f_prev.abs()
    }
}

fn max_of_floats(numbers: &[f64]) -> f64 {
    numbers.iter().copied().fold(f64::NAN, f64::max)
}

fn max_xyz(data: &SensorData) -> (f64, f64) {
    let (x, y, z) = data.fields.iter().fold((0., 0., 0.), |acc, f| {
        (m_max(f.x, acc.0), m_max(f.y, acc.1), m_max(f.z, acc.2))
    });
    (
        x.max(y).max(z),
        data.timestamp.unwrap_or_default().as_seconds_f64().trunc()
    )
}



fn get_device_info(gpmf: &Gpmf) {
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




fn get_src_file_path(srs_dir_path: &str) -> Option<PathBuf> {
    let paths = fs::read_dir(srs_dir_path)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();
            path.extension()
                .and_then(|ext| ext.to_str().map(|s| s.to_lowercase() == "mp4"))
                .unwrap_or(false)
        })
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    if !paths.is_empty() {
        Some(paths[0].to_owned())
    } else {
        None
    }
}



fn get_output_filename(src_file_path: &PathBuf, dest_dir_path: &str, output_file_postfix: &str) -> PathBuf {
    let dest_dir_path = PathBuf::from(dest_dir_path);
    let output_file_name = format!(
        "{}{}.mp4",
        src_file_path.file_stem().unwrap().to_str().unwrap(),
        output_file_postfix
    );
    let output_file_path = dest_dir_path.join(output_file_name);

    println!("output_file_path: {:?}", output_file_path);

    if output_file_path.exists() {
        println!("NEW output_file_path: {:?}", output_file_path);
        // let original_extension = output_file_path.extension().unwrap_or_default();
        // let new_extension = format!(".copy.{}", original_extension.to_str().unwrap());
        // let new_file_name = PathBuf::from(output_file_path.file_name().unwrap()).with_extension(new_extension);
        // output_file_path.set_file_name(new_file_name);
    }

    output_file_path
}


fn parse_sensor_data_list(
    sensor_data_list: Vec<SensorData>,
    config_values: &ConfigValues
) -> Result<(f64, f64), Result<(), std::io::Error>> {
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
        target_start_time = 0.0
    }
    let target_end_time = deployment_time + config_values.time_end_offset;
    let target_duration = target_end_time - target_start_time;
    println!(
        "max_datablock: {:?} st_time: {:?} end_time: {:?} duration: {:?}\n",
        max_accel_data, target_start_time, target_end_time, target_duration
    );
    Ok((target_start_time, target_duration))
}






fn main() -> std::io::Result<()> {
    let config_values = get_config_values();

    let src_file_path = match get_src_file_path(&config_values.srs_dir_path) {
        Some(path) => path,
        None => {
            __promt_to_exit("NO MP4 FILES FOUND!");
            return Ok(());
        }
    };

    let gpmf = Gpmf::new(&src_file_path, false)?;

    get_device_info(&gpmf);


    let sensor_data_list = gpmf.sensor(&SensorType::Accelerometer);
    let (target_start_time, target_duration) = match parse_sensor_data_list(
        sensor_data_list,
        &config_values
    ) {
        Ok(value) => value,
        Err(value) => return value,
    };


    let output_file_path = get_output_filename(
        &src_file_path,
        &config_values.dest_dir_path,
        &config_values.output_file_postfix
    );



    // __promt_to_exit("FFMPEG DISABLED");
    // return Ok(());

    match Command::new(format!("{}{}", config_values.ffmpeg_dir_path, "/ffmpeg"))
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        // .arg("-n")
        .args(["-i", src_file_path.to_str().unwrap()])
        .args(["-ss", target_start_time.to_string().as_str()])
        .args(["-t", target_duration.to_string().as_str()])
        .args(["-c", "copy"])
        .arg("-y")
        .arg(&output_file_path)
        .spawn() {
            Ok(_) => println!("FFmpeg executed successfully."),
            Err(e) => eprintln!(
                "Failed to execute FFmpeg: {:?}", e
            ),
        }

    __promt_to_exit(format!(
        "Video has been successfully cut and saved as {}",
        output_file_path.display()
    ).as_str());

    Ok(())
}



