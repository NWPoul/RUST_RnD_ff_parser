

pub mod utils {
    pub mod error;
    pub mod u_serv;
    pub use u_serv::abs_max;
}

use std::path::PathBuf;

use rfd::FileDialog;
use config::{Config, File as Cfg_file};



pub mod macros;
pub mod file_sys_serv;

pub mod gpmf_serv;
pub mod telemetry_parser_serv;

mod cli_clonfig;
use cli_clonfig::get_cli_merged_config;


// use file_sys_serv::get_output_filename;
use telemetry_parser_serv::{get_result_metadata_for_file, TelemetryResultAccData};

pub mod analise;





const DEF_DIR    : &str        = ".";
const DEF_POSTFIX: &str        = "_FFCUT";
// const DEF_PROMPT_FLIGHT: bool  = false;
const DEP_TIME_CORRECTION: f64 = 2.0;
const TIME_START_OFFSET  : f64 = -60.0;
const TIME_END_OFFSET    : f64 = 3.0;

const MIN_ACCEL_TRIGGER  : f64 = 8.0;


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


pub fn format_acc_datablock(acc_n_time: &(f64, f64)) -> String {
    format!("{:.2}m/s2  @ {}s", acc_n_time.0, acc_n_time.1)
}

// struct FileAnalysisResult {
//     pub device_name : String,
//     pub start_time  : f64,
//     pub end_time    : f64,
//     pub max_acc_data: (f64, f64),
// }
// impl FileAnalysisResult {
//     pub fn get_description(&self) -> String {
//         format!(
//             "CAM: {} Freefall: {}s-{}s ({}s) Max Acc: {}",
//             self.device_name,
//             self.start_time,
//             self.end_time,
//             self.end_time - self.start_time,
//             format_acc_datablock(&(self.max_acc_data.0, self.max_acc_data.1))
//         )
//     }
// }



pub fn parse_mp4_files(
    src_files_path_list: &Vec<PathBuf>,
    // config_values      : ConfigValues
) -> Vec<Result<TelemetryResultAccData, String>> {
    let mut result_list:Vec<Result<TelemetryResultAccData, String>> = vec![];

    for src_file_path in src_files_path_list {
        let file_res = match get_result_metadata_for_file(&src_file_path.to_string_lossy()) {
            Ok(res_acc_data) => Ok(res_acc_data.acc_data),
            Err(err_str)     => Err(err_str)
        };
        result_list.push(file_res);
    };
    result_list
}


// fn print_parsing_results(parsing_result: Vec<Result<Child, IOError>>) {
//     println!("\nPARSING RESULTS:");
//     for res in parsing_result {
//         match res {
//             Ok(content) => println!("OK: {content:?}"),
//             Err(error)  => println!("ERR: {error}")
//         }
//     }
// }

fn plot_parsed_data(data: &(Vec<(f64, f64, f64)>, Vec<f64>)) {
    crate::analise::gnu_plot_single(&data.1);
    crate::analise::gnu_plot_xyz(&(data.0));
}




fn main() {
    let mut config_values = get_config_values();
    config_values = get_cli_merged_config(config_values);

    let src_files_path_list = match FileDialog::new()
        .add_filter("mp4", &["mp4", "MP4"])
        .set_directory(&config_values.srs_dir_path)
        .pick_files() {
            Some(file_path_list) => file_path_list,
            None => {
                promptExit!("NO MP4 FILES CHOSEN!");
            }
        };

        let parsing_result = parse_mp4_files(&src_files_path_list);
        for res in parsing_result {
            match res {
                Ok(res_data) => plot_parsed_data(
                    &(
                        res_data.xyz,
                        res_data.sma,
                    )
                ),
                Err(error)  => println!("ERR: {error}"),
            }
        }
    promptExit!("\nEND");
}

