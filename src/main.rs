

pub mod utils {
    pub mod error;
    pub mod u_serv;
    pub use u_serv::abs_max;
}

use std::path::PathBuf;

use config::{Config, File as Cfg_file};
use gpmf_rs::Gpmf;

pub mod macros;
pub mod file_sys_serv;
pub mod gpmf_serv;
pub mod ffmpeg_serv;

mod cli_config;
use cli_config::get_cli_merged_config;

use file_sys_serv::{
    convert_to_absolute, extract_filename, get_output_filename, get_src_files_path_list
};

use ffmpeg_serv::run_ffmpeg;
use gpmf_serv::GPMFParsedData;



// type FileParsingResult  = Vec<(String, Result<GPMFParsedData, String>)>;
type FileParsingOkData  = Vec<(String, GPMFParsedData)>;
type FileParsingErrData = Vec<(String, String)>;



const GREEN: &str = "\x1B[32m";
const RED  : &str = "\x1B[31m";
const BOLD : &str = "\x1B[1m";
const RESET: &str = "\x1B[0m";


const DEF_DIR            : &str = ".";
const DEF_POSTFIX        : &str = "_FFCUT";
const DEP_TIME_CORRECTION:  f64 = 2.0;
const TIME_START_OFFSET  :  f64 = -60.0;
const TIME_END_OFFSET    :  f64 = 3.0;

const MIN_ACCEL_TRIGGER  :  f64 = 5.0;





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


pub fn parse_mp4_file(src_file_path: &PathBuf, config_values: ConfigValues) -> Result<GPMFParsedData, String> {
    let gpmf = Gpmf::new(&src_file_path, false)
        .map_err( |e| format!("GPMF FAILED: {:?}", e) )?;

    // let device_name

    let gpmf_data = match gpmf_serv::parse_sensor_data(&gpmf, &config_values, &src_file_path) {
        Ok(data)     => data,
        Err(err_msg) => {
            return Err(err_msg)
        },
    };

    let output_file_path = get_output_filename(
        &src_file_path,
        &config_values.dest_dir_path,
        &config_values.output_file_postfix
    );

    // return Err(IOError::new(ErrorKind::Other, "Command disabled"));
    // promptExit!("Command disabled" );

    let ffmpeg_status = run_ffmpeg(
        (gpmf_data.start_time, gpmf_data.end_time),
        (&src_file_path, &output_file_path ),
        &config_values.ffmpeg_dir_path,
    );
    // println!("FFMPEG STATUS: {:?}", ffmpeg_status);

    match ffmpeg_status {
        Ok(_data) => Ok(gpmf_data),
        Err(err)  => Err(err.to_string()),
    }
}


pub fn parse_mp4_files(
    src_files_path_list: Vec<PathBuf>,
    config_values      : ConfigValues
) -> (FileParsingOkData, FileParsingErrData) {
    let mut ok_list : FileParsingOkData  = vec![];
    let mut err_list: FileParsingErrData = vec![];

    for src_file_path in src_files_path_list {
        let parsing_res = parse_mp4_file(&src_file_path, config_values.clone());
        let result      = (extract_filename(src_file_path), parsing_res);
        match result {
            (filename, Ok(data))     => ok_list.push((filename, data)),
            (filename, Err(err_msg)) => err_list.push((filename, err_msg)),
        }
    };
    (ok_list, err_list)
}


fn print_parsing_results(parsing_results: (FileParsingOkData, FileParsingErrData), dest_dir: &str) {

    let dest_dir_string = convert_to_absolute(dest_dir)
        .unwrap_or("".into())
        .to_string_lossy()
        .replace("\\\\?\\", "");

    println!("\n\n{BOLD}PARSING RESULTS:{RESET}");

    println!("\n{BOLD}{GREEN}OK: => {}{RESET}", dest_dir_string);
    for res in &parsing_results.0 {
        println!("{GREEN}{:?}{RESET} {:?}", res.0, res.1.get_description());
    }

    println!("\n{RED}{BOLD}FAILED:{RESET}");
    for res in &parsing_results.1 {
        println!("{RED}{:?}{RESET} {:?}", res.0, res.1);
    }
}




fn main() {
    let mut config_values = get_config_values();
    config_values = get_cli_merged_config(config_values);

    let src_files_path_list = match get_src_files_path_list(&config_values.srs_dir_path) {
        Some(path_list) => path_list,
        None            => { promptExit!("NO MP4 FILES CHOSEN!"); }
    };

    let parsing_result = parse_mp4_files(src_files_path_list, config_values.clone());

    print_parsing_results(parsing_result, &config_values.dest_dir_path);

    promptExit!("\nEND");
}

