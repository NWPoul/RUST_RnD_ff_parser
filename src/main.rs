

pub mod utils {
    pub mod error;
    pub mod u_serv;
    pub use u_serv::abs_max;
}

pub mod macros;
pub mod analise;
pub mod analise_ev_rnd;

pub mod telemetry_parser_serv;
pub mod file_sys_serv;
pub mod plot_serv;

mod cli_config;



use std::path::PathBuf;
use std::sync::Mutex;

use analise::{
    calc_velocity_arr,
    data_to_stat_vals_arr,
    v3d_list_to_magnitude_sma_list,
    v3d_list_to_magnitude_smaspr_list,
    v3d_list_to_plainsum_sma_list,
    v3d_list_to_ts_sma_v3d_list
};
use analise_ev_rnd::stft_result_analise;
// use file_sys_serv::{save_det_log_to_txt, save_sma_log_to_txt};
use lazy_static::lazy_static;

use plot_serv::{
    gnu_plot_multi_ts_data,
    gnu_plot_single_data, gnu_plot_v3d_and_multi_ts_data,
    // gnu_plot_stats_for_v3d_data,
};
use rfd::FileDialog;
use config::{Config, File as Cfg_file};

use cli_config::get_cli_merged_config;


// use file_sys_serv::get_output_filename;
use telemetry_parser_serv::{get_result_metadata_for_file, TelemetryParsedData, TsScalarArr, TsValsArr};

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





fn plot_parsed_analised_base_series(data: &[Vector3d], base_series: &[usize], title: &str) {
    gnu_plot_v3d_series_and_stats(data, base_series, title);
}
fn plot_velosity_list(data: &[Vector3d], base_series: &[usize], title: &str) {
    let velocity_list = calc_velocity_arr(data, &telemetry_parser_serv::DEF_TICK);
    gnu_plot_v3d_series_and_stats(&velocity_list.0, base_series, title);
    gnu_plot_single_data(&velocity_list.1, &telemetry_parser_serv::DEF_TICK, "mag_v");
}

// fn save_log_data(src_file_path: &PathBuf, res_data: &TelemetryParsedData) {
//     save_det_log_to_txt(&res_data.acc_data, src_file_path);
//     save_sma_log_to_txt(
//         &v3d_list_to_magnitude_sma_list(&res_data.acc_data, 1 as usize),
//         src_file_path,
//     );
// }

fn gnu_plot_stats_for_v3d_data(data: &[Vector3d], base_series: &[usize], title: &str) {
    let mut sma_magnitude_series: Vec<(Vec<f64>, Vec<f64>, String, &str)> = Vec::new();
    let mut spr_magnitude_series: Vec<(Vec<f64>, Vec<f64>, String, &str)> = Vec::new();

    let base_to_label = |base: &usize| format!("{} pt", base);
    for base in base_series {
        let cur_stats = v3d_list_to_magnitude_smaspr_list(data, *base);
        sma_magnitude_series.push((cur_stats.0.clone(), cur_stats.1, base_to_label(base), "black"));
        spr_magnitude_series.push((cur_stats.0        , cur_stats.2, base_to_label(base), "brown"));
    }

    gnu_plot_multi_ts_data(
        &[sma_magnitude_series, spr_magnitude_series].concat(),
        title
    );
}





type StatsBaseSeries = Vec<(Vec<f64>, Vec<f64>     , usize, String)>;
type V3dBaseSeries   = Vec<(Vec<f64>, Vec<Vector3d>, usize)>;

fn get_stats_for_v3d_base_series(data: &[Vector3d], series_props: &[usize]) -> (StatsBaseSeries, StatsBaseSeries) {
    let mut sma_magnitude_series: StatsBaseSeries = Vec::new();
    let mut sma_plainsum_series : StatsBaseSeries = Vec::new();
    
    for base in series_props {
        let cur_magnitude_sma = v3d_list_to_magnitude_sma_list(data, *base);
        let cur_plainsum_sma  = v3d_list_to_plainsum_sma_list(data, *base);
        sma_magnitude_series.push((cur_magnitude_sma.0, cur_magnitude_sma.1, *base, "black".into()));
        sma_plainsum_series.push((cur_plainsum_sma.0, cur_plainsum_sma.1   , *base, "brown".into()));
    }
    (sma_magnitude_series, sma_plainsum_series)
}


pub fn gnu_plot_v3d_series_and_stats(data: &[Vector3d], series_props: &[usize], title: &str) {
    let v3d_sma: V3dBaseSeries = series_props.iter().map(|base| {
        let cur_v3d_sma  = v3d_list_to_ts_sma_v3d_list(data, *base);
        (cur_v3d_sma.0, cur_v3d_sma.1, *base)
    }).collect();

    let (
        sma_magnitude_series,
        sma_plainsum_series,
    ) = get_stats_for_v3d_base_series(data, series_props);

    gnu_plot_v3d_and_multi_ts_data(
        &v3d_sma,
        &[sma_magnitude_series, sma_plainsum_series].concat(),
        title,
    );
}








fn plot_iso_series(data: &TsScalarArr, base_series: &[usize], title: &str) {
    let mut iso_series1: StatsBaseSeries = Vec::new();
    let mut iso_series2: StatsBaseSeries = Vec::new();
    
    for base in base_series {
        let base = std::cmp::min(data.v.len() / 2, *base);
        let cur_iso_stats = data_to_stat_vals_arr(&data.v, base);
        iso_series1.push((data.t.clone(), cur_iso_stats.sma, base, "black".into()));
        iso_series2.push((data.t.clone(), cur_iso_stats.spr, base, "black".into()));
    }

    gnu_plot_multi_ts_data(&iso_series1, title);
    gnu_plot_multi_ts_data(&iso_series2, title);
}



fn calculate_deployment(data: &[Vector3d], base_series: &[usize], title: &str) {
    println!("{}", title);
    for base in base_series {
        let cur_magnitude_sma = v3d_list_to_magnitude_sma_list(data, *base);
        stft_result_analise(&cur_magnitude_sma.1, *base)
    }
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
                            &res_data.acc_data.v,
                            &base_series,
                            &res_data.file_name,
                        );

                        calculate_deployment(
                            &res_data.acc_data.v,
                            &base_series,
                            &res_data.file_name,
                        );

                        // plot_parsed_analised_base_series(
                        //     &res_data.gyro_data,
                        //     &base_series,
                            
                        //     &res_data.file_name,
                        // );

                        // plot_velosity_list(
                        //     &res_data.acc_data,
                        //     &base_series,
                        // );

                        plot_iso_series(
                            &res_data.lumen_data,
                            &base_series,
                            &res_data.file_name,
                        );
                        // if SAVE_LOG {
                        //     save_log_data(&src_files_path_list[0], &res_data);
                        // };
                    },
                    Err(error)  => println!("ERR: {error}"),
                }
            }

            input_sma_base();
    }
}


