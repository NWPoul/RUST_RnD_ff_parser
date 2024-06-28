

pub mod utils {
    pub mod error;
    pub mod u_serv;
    pub use u_serv::abs_max;
}
pub mod macros;

mod cli_config;

pub mod file_sys_serv;
pub mod ffmpeg_serv;
pub mod telemetry_parser_serv;

pub mod telemetry_analysis;



use std::path::PathBuf;
use crossbeam_channel::{unbounded, Sender, Receiver};
// use std::sync::mpsc::{channel, Sender};

use config::{Config, File as CfgFile};

use cli_config::get_cli_merged_config;

use file_sys_serv::{
    // convert_to_absolute,
    convert_to_absolute, copy_with_progress, extract_filename, get_output_filename, get_src_files_path_list, watch_drivers
};

use ffmpeg_serv::run_ffmpeg;

use telemetry_analysis::{
    get_result_metadata_for_file,
    FileTelemetryResult,
};





pub const GREEN : &str = "\x1B[32m";
pub const RED   : &str = "\x1B[31m";
pub const YELLOW: &str = "\x1B[33m";
pub const BOLD  : &str = "\x1B[1m";
pub const RESET : &str = "\x1B[0m";


const DEF_DIR            : &str = ".";
const DEF_POSTFIX        : &str = "_FFCUT";
const DEP_TIME_CORRECTION:  f64 = 2.0;
const TIME_START_OFFSET  :  f64 = -60.0;
const TIME_END_OFFSET    :  f64 = 3.0;

const MIN_ACCEL_TRIGGER  :  f64 = 20.0;



type FileParsingOkData  = Vec<(PathBuf, FileTelemetryResult)>;
type FileParsingErrData = Vec<(PathBuf, String)>;


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





pub fn get_telemetry_for_files(
    src_files_path_list: &[PathBuf],
    config_values      : &ConfigValues,
) -> (FileParsingOkData, FileParsingErrData) {
    let mut ok_list : FileParsingOkData  = vec![];
    let mut err_list: FileParsingErrData = vec![];

    for src_file_path in src_files_path_list {
        let input_file = src_file_path.to_string_lossy();

        match get_result_metadata_for_file(&input_file, &config_values) {
            Ok(data)     => ok_list.push((src_file_path.clone(), data)),
            Err(err_str) => err_list.push((src_file_path.clone(), err_str)),
        };
    };
    (ok_list, err_list)
}


pub fn get_ffmpeg_status_for_file(
    src_file_path   : &PathBuf,
    file_result_data: &FileTelemetryResult,
    config_values   : &ConfigValues,
) -> Result<String, String> {
    let output_file_path = get_output_filename(
        src_file_path,
        &(&config_values.dest_dir_path).into(),
        &config_values.output_file_postfix,
        &file_result_data.device_name,
    );


    let ffmpeg_output = run_ffmpeg(
        (file_result_data.start_time, file_result_data.end_time),
        (&src_file_path, &output_file_path ),
        &config_values.ffmpeg_dir_path,
    );


    match ffmpeg_output {
        Ok(_output) => {
            println!("\nFFMPEG OK:");// {:?}", _output.stderr);
            Ok("FFMPEG STATUS - OK".into())
        },
        Err(err)  => {
            println!("\nFFMPEG ERR: {:?}", err.to_string());
            Err(err.to_string())
        }
    }
}

pub fn ffmpeg_ok_files(
    parsing_results: &(FileParsingOkData, FileParsingErrData),
    config_values  : &ConfigValues,
) {
    for res in &parsing_results.0 {
        _ = get_ffmpeg_status_for_file(
            &res.0,
            &res.1,
            config_values,
        );
    }
}




fn print_parsing_results(
    parsing_results: &(FileParsingOkData, FileParsingErrData),
    dest_dir       : &PathBuf,
) {
    let output_file_path = get_output_filename(&"".into(), dest_dir, "", "");
    let output_dir_path  = output_file_path.parent().unwrap();
    let output_dir_abs_path = convert_to_absolute(&output_dir_path);
    let output_dir_string   = output_dir_abs_path
        .to_string_lossy()
        .replace("\\\\?\\", "");

    println!("\n\n{BOLD}PARSING RESULTS:{RESET}");

    println!("\n{BOLD}{GREEN}OK: => {}{RESET}", &output_dir_string);
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
        let dest_file_path = get_output_filename(
            src_file_path,
            &(&config_values.dest_dir_path).into(),
            "_NA", ""
        );
        let copy_res = copy_with_progress(
            src_file_path,
            &dest_file_path,
        );
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

        let src_files_path_list = match get_src_files_path_list(&src_dir) {
            Some(path_list) => path_list,
            None => {
                println!("NO MP4 FILES CHOSEN!");
                // should_continue = utils::u_serv::prompt_to_continue("NO MP4 FILES CHOSEN!");
                continue;
            }
        };

        let parsing_results = get_telemetry_for_files(&src_files_path_list, &config_values);

        ffmpeg_ok_files(&parsing_results, &config_values);

        print_parsing_results(&parsing_results, &(&config_values.dest_dir_path).into());

        copy_invalid_files(&parsing_results.1, &config_values);

        should_continue = true //utils::u_serv::prompt_to_continue("");
    }
    promptExit!("\nEND");
}



