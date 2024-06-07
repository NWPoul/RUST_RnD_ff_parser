
// use crate::prelude::*;
// mod prelude;

pub mod utils {
    pub mod error;
    pub mod u_serv;

    pub use u_serv::abs_max;
}

use std::path::PathBuf;
use rfd::FileDialog;
use config::{Config, File as Cfg_file};
use gpmf_rs::Gpmf;


pub mod macros;
pub mod file_sys_serv;
pub mod gpmf_serv;
pub mod ffmpeg_serv;

mod cli_clonfig;
use cli_clonfig::get_cli_merged_config;


use file_sys_serv::get_output_filename;

use ffmpeg_serv::run_ffmpeg;





const DEF_DIR    : &str        = ".";
const DEF_POSTFIX: &str        = "_FFCUT";
// const DEF_PROMPT_FLIGHT: bool  = false;
const DEP_TIME_CORRECTION: f64 = 2.0;
const TIME_START_OFFSET  : f64 = -60.0;
const TIME_END_OFFSET    : f64 = 3.0;

const MIN_ACCEL_TRIGGER  : f64 = 50.0;

pub const GLITCH_MARGIN  : f64 = 3.0;


configValues!(
    ( srs_dir_path       , String , DEF_DIR.to_string() ),
    ( dest_dir_path      , String , DEF_DIR.to_string() ),
    ( ffmpeg_dir_path    , String , DEF_DIR.to_string() ),
    ( output_file_postfix, String , DEF_POSTFIX.to_string() ),
    ( dep_time_correction, f64    , DEP_TIME_CORRECTION ),
    ( time_start_offset  , f64    , TIME_START_OFFSET ),
    ( time_end_offset    , f64    , TIME_END_OFFSET ),
    ( min_accel_trigger  , f64    , MIN_ACCEL_TRIGGER )
);


pub fn parse_mp4_file(src_file_path: PathBuf, config_values: ConfigValues)-> Result<std::process::Child, std::io::Error> {
    let gpmf = Gpmf::new(&src_file_path, false)?;

    gpmf_serv::get_device_info(&gpmf);

    let target_start_end_time = match gpmf_serv::parse_sensor_data(&gpmf, &config_values) {
        Ok(value)  => value,
        Err(err_msg) => return Err(
            std::io::Error::new(std::io::ErrorKind::Other, err_msg)
        ),
    };

    let output_file_path = get_output_filename(
        &src_file_path,
        &config_values.dest_dir_path,
        &config_values.output_file_postfix
    );

    // promptExit!("Command disabled" );

    let ffmpeg_status = run_ffmpeg(
        target_start_end_time,
        (&src_file_path, &output_file_path ),
        &config_values.ffmpeg_dir_path,
    );

    ffmpeg_status
}





fn main() {
    let config_values = get_config_values();
    let config_values = get_cli_merged_config(config_values);

    let src_files_path_list = match FileDialog::new()
        .add_filter("mp4", &["mp4", "MP4"])
        .set_directory(&config_values.srs_dir_path)
        .pick_files() {
            Some(file_path_list) => file_path_list,
            None => {
                promptExit!("NO MP4 FILES CHOSEN!");
            }
        };

    for src_file_path in src_files_path_list {
        let _ = parse_mp4_file(src_file_path, config_values.clone());
    }

    promptExit!("\nEND");
}

