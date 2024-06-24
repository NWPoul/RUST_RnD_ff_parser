

pub mod utils {
    pub mod error;
    pub mod u_serv;
    pub use u_serv::abs_max;
}
// use std::io::{self, Read};
use std::path::PathBuf;
use crossbeam_channel::{unbounded, Sender, Receiver};
// use std::sync::mpsc::{channel, Sender};

use config::{Config, File as Cfg_file};
use gpmf_rs::Gpmf;

pub mod macros;
pub mod file_sys_serv;
pub mod gpmf_serv;
pub mod ffmpeg_serv;

mod cli_config;
use cli_config::get_cli_merged_config;

use file_sys_serv::{
    // convert_to_absolute,
    copy_with_progress, extract_filename, get_output_filename, get_src_files_path_list, watch_drivers
};

use ffmpeg_serv::run_ffmpeg;
use gpmf_serv::GPMFParsedData;



// type FileParsingResult  = Vec<(String, Result<GPMFParsedData, String>)>;
type FileParsingOkData  = Vec<(PathBuf, GPMFParsedData)>;
type FileParsingErrData = Vec<(PathBuf, String)>;



pub const GREEN: &str = "\x1B[32m";
pub const RED: &str = "\x1B[31m";
pub const YELLOW: &str = "\x1B[33m";
pub const BOLD: &str = "\x1B[1m";
pub const RESET: &str = "\x1B[0m";


const DEF_DIR            : &str = ".";
const DEF_POSTFIX        : &str = "_FFCUT";
const DEP_TIME_CORRECTION:  f64 = 2.0;
const TIME_START_OFFSET  :  f64 = -60.0;
const TIME_END_OFFSET    :  f64 = 3.0;

const MIN_ACCEL_TRIGGER  :  f64 = 20.0;





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


    let ffmpeg_output = run_ffmpeg(
        (gpmf_data.start_time, gpmf_data.end_time),
        (&src_file_path, &output_file_path ),
        &config_values.ffmpeg_dir_path,
    );


    match ffmpeg_output {
        Ok(_output) => {
            println!("\nFFMPEG OK:");// {:?}", _output.stderr);
            Ok(gpmf_data)
        },
        Err(err)  => {
            println!("\nFFMPEG ERR: {:?}", err.to_string());
            Err(err.to_string())
        }
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
        let result      = (src_file_path, parsing_res);
        match result {
            (src_file_path, Ok(data))     => ok_list.push((src_file_path, data)),
            (src_file_path, Err(err_msg)) => err_list.push((src_file_path, err_msg)),
        }
    };
    (ok_list, err_list)
}


fn print_parsing_results(
    parsing_results: &(FileParsingOkData, FileParsingErrData),
    dest_dir: &str,
) {
    let def_path = PathBuf::from(".");
    let output_file_path = get_output_filename(&PathBuf::from(""), dest_dir, "");
    let dest_dir_string = output_file_path
        .parent()
        .unwrap_or(&def_path);

    println!("\n\n{BOLD}PARSING RESULTS:{RESET}");

    println!("\n{BOLD}{GREEN}OK: => {}{RESET}", &dest_dir_string.to_string_lossy());
    for res in &parsing_results.0 {
        println!(
            "{GREEN}{:?}{RESET} {:?}",
            extract_filename(&res.0),
            res.1.get_description()
        );
    }

    if parsing_results.1.len() > 0 {
        println!("\n{RED}{BOLD}FAILED:{RESET}");
        for res in &parsing_results.1 {
            println!("{RED}{:?}{RESET} {:?}", extract_filename(&res.0), res.1);
        }
    }
}


fn copy_invalid_files(err_results: &FileParsingErrData, config_values: &ConfigValues) {
    let should_copy = if err_results.len() > 0 {
        println!(
            "Can't parse {} files. Copy them as is? (Y/n)",
            err_results.len()
        );
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().is_empty() || input.trim().to_lowercase() == "y"
    } else {
        false
    };

    if !should_copy {
        return;
    }

    for (src_file_path, _) in err_results {
        let dest_file_path =
            get_output_filename(src_file_path, &config_values.dest_dir_path, "_NA");
        let copy_res = copy_with_progress(src_file_path.to_str().unwrap(), &dest_file_path.to_str().unwrap());
        match copy_res {
            Ok(number) => {
                println!(
                    "copy succsess: {:?} to {:?} ({:?})",
                    src_file_path, dest_file_path, number
                );
            }
            Err(err) => {
                println!("Failed to copy {:?} to {:?}", src_file_path, dest_file_path);
                println!("Error: {:?}", err.to_string());
            }
        }
    }
}



fn main() {
    let mut config_values = get_config_values();
    config_values = get_cli_merged_config(config_values);

    let (tx, rx): (Sender<()>, Receiver<()>) = unbounded();

    let mut should_continue = true;
    while should_continue {
        let whatched_dir = watch_drivers(tx.clone(), rx.clone());
        println!("main::whatched_dir: {:?}", whatched_dir);
        let src_dir = whatched_dir.unwrap_or((&config_values.srs_dir_path).into());

        let src_files_path_list = match get_src_files_path_list(&src_dir.to_string_lossy()) {
            Some(path_list) => path_list,
            None => {
                println!("NO MP4 FILES CHOSEN!");
                // should_continue = utils::u_serv::prompt_to_continue("NO MP4 FILES CHOSEN!");
                continue;
            }
        };

        let parsing_results = parse_mp4_files(src_files_path_list, config_values.clone());

        print_parsing_results(&parsing_results, &src_dir.to_string_lossy());

        copy_invalid_files(&parsing_results.1, &config_values);

        should_continue = true //utils::u_serv::prompt_to_continue("");
    }
    promptExit!("\nEND");
}



