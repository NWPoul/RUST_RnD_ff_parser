

pub mod utils {
    pub mod error;
    pub mod u_serv;
    pub use u_serv::abs_max;
}

pub mod macros;
pub mod analise;

pub mod telemetry_parser_serv;
pub mod file_sys_serv;
pub mod plot_serv;

mod cli_config;



use std::path::PathBuf;
use std::sync::Mutex;

use analise::v3d_list_to_magnitude_sma_list;
use file_sys_serv::{save_det_log_to_txt, save_sma_log_to_txt};
use lazy_static::lazy_static;

use plot_serv::{gnu_plot_series, gnu_plot_single, gnu_plot_single_spr, gnu_plot_stats_for_data};
use rfd::FileDialog;
use config::{Config, File as Cfg_file};

use cli_config::get_cli_merged_config;


// use file_sys_serv::get_output_filename;
use telemetry_parser_serv::{get_result_metadata_for_file, TelemetryParsedData};
use utils::u_serv::Vector3d;


pub mod analise_log_v;



lazy_static! {
    pub static ref SMA_BASE: Mutex<Vec<usize>> = Mutex::new(vec![50]);
}





const DEF_DIR    : &str        = ".";
const DEP_TIME_CORRECTION: f64 = 2.0;
const TIME_START_OFFSET  : f64 = -3.0;
const TIME_END_OFFSET    : f64 = 3.0;

const MIN_ACCEL_TRIGGER  : f64 = 20.0;

pub const PLOT_RAW: bool = true;
pub const SAVE_LOG: bool = false;


configValues!(
    ( srs_dir_path       , String , DEF_DIR.to_string() ),
    ( dest_dir_path      , String , DEF_DIR.to_string() ),
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
) -> Vec<Result<TelemetryParsedData, String>> {
    let mut result_list:Vec<Result<TelemetryParsedData, String>> = vec![];

    for src_file_path in src_files_path_list {
        let file_res = get_result_metadata_for_file(&src_file_path.to_string_lossy());
        result_list.push(file_res);
    };
    result_list
}



fn plot_parsed_analised_base_series(data: &[Vector3d], base_series: &[usize]) {
    gnu_plot_series(data, base_series);
}

fn plot_parsed_iso_series(data: &[u32], title: &str) {
    gnu_plot_single(data, &telemetry_parser_serv::DEF_TICK, title);
}

fn save_log_data(src_file_path: &PathBuf, res_data: &TelemetryParsedData) {
    save_det_log_to_txt(&res_data.acc_data, src_file_path);
    save_sma_log_to_txt(
        &v3d_list_to_magnitude_sma_list(&res_data.acc_data, 1 as usize),
        src_file_path,
    );
}


fn input_sma_base() {
    let mut base_series = SMA_BASE.lock().unwrap();
    println!("\ninput base (current {:?})...\n", &base_series);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input_vec = input.trim().split_whitespace();
    if input_vec.clone().count() < 1 {
        return
    }

    *base_series = input_vec
        .map(
            |s| s.parse::<usize>().unwrap_or(50 as usize)
        )
        .collect::<Vec<usize>>()
}





fn main() {
    let mut config_values = get_config_values();
    config_values = get_cli_merged_config(config_values);

    loop {
        let base_series = SMA_BASE.lock().unwrap().to_owned();
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
                    Ok(res_data) => {
                        plot_parsed_analised_base_series(
                            &res_data.acc_data,
                            &base_series,
                        );
                        gnu_plot_stats_for_data(
                            &res_data.acc_data,
                            &base_series,
                        );
                        // plot_serv::gnu_plot_ts_data(
                        //     &res_data.iso_data.0.ts,
                        //     &res_data.iso_data.0.vals,
                        //     &res_data.file_name,
                        // );
                        // plot_serv::gnu_plot_ts_data(
                        //     &res_data.iso_data.1.ts,
                        //     &res_data.iso_data.1.vals,
                        //     &res_data.file_name,
                        // );
                        if SAVE_LOG {
                            save_log_data(&src_files_path_list[0], &res_data);
                        };
                    },
                    Err(error)  => println!("ERR: {error}"),
                }
            }

            input_sma_base();
    }
}

