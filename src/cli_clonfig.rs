
use clap::{Arg, App};

use crate::ConfigValues;

// Your ConfigValues struct definition remains unchanged

// Updated CliArgs struct with all fields
#[derive(Debug)]
pub struct CliArgs {
    srs_dir_path: Option<String>,
    dest_dir_path: Option<String>,
    ffmpeg_dir_path: Option<String>,
    output_file_postfix: Option<String>,
    dep_time_correction: Option<f64>,
    time_start_offset: Option<f64>,
    time_end_offset: Option<f64>,
    min_accel_trigger: Option<f64>,
}

pub fn get_cli_args() -> CliArgs {
    let matches = App::new("YourApp")
        .arg(
            Arg::with_name("srs-dir-path")
                .short('s')
                .long("src-dir")
                .value_name("PATH")
                .help("Sets the source directory path"),
        )
        .arg(
            Arg::with_name("dest-dir-path")
                .short('d')
                .long("dest-dir")
                .value_name("PATH")
                .help("Sets the destination directory path"),
        )
        .arg(
            Arg::with_name("ffmpeg-dir-path")
                .short('f')
                .long("ffmpeg-dir")
                .value_name("PATH")
                .help("Sets the FFmpeg directory path"),
        )
        .arg(
            Arg::with_name("output-file-postfix")
                .short('p')
                .long("postfix")
                .value_name("POSTFIX")
                .help("Sets the output file postfix"),
        )
        .arg(
            Arg::with_name("dep-time-correction")
                .short('t')
                .long("time-correction")
                .value_name("TIME")
                .help("Sets the dependency time correction value"),
        )
        .arg(
            Arg::with_name("time-start-offset")
                .short('o')
                .long("start-offset")
                .value_name("OFFSET")
                .help("Sets the start time offset value"),
        )
        .arg(
            Arg::with_name("time-end-offset")
                .short('e')
                .long("end-offset")
                .value_name("OFFSET")
                .help("Sets the end time offset value"),
        )
        .arg(
            Arg::with_name("min-accel-trigger")
                .short('a')
                .long("accel-trigger")
                .value_name("VALUE")
                .help("Sets the minimum acceleration trigger value"),
        )
        .get_matches();

    CliArgs {
        srs_dir_path: matches.value_of("srs-dir-path").map(|v| v.to_string()),
        dest_dir_path: matches.value_of("dest-dir-path").map(|v| v.to_string()),
        ffmpeg_dir_path: matches.value_of("ffmpeg-dir-path").map(|v| v.to_string()),
        output_file_postfix: matches
            .value_of("output-file-postfix")
            .map(|v| v.to_string()),
        dep_time_correction: matches
            .value_of("dep-time-correction")
            .and_then(|v| v.parse::<f64>().ok()),
        time_start_offset: matches
            .value_of("time-start-offset")
            .and_then(|v| v.parse::<f64>().ok()),
        time_end_offset: matches
            .value_of("time-end-offset")
            .and_then(|v| v.parse::<f64>().ok()),
        min_accel_trigger: matches
            .value_of("min-accel-trigger")
            .and_then(|v| v.parse::<f64>().ok()),
    }
}

pub fn get_resulting_config(mut config_values: ConfigValues) -> ConfigValues {
    let cli_args = get_cli_args();

    // Overwrite settings with CLI arguments
    if let Some(srs_dir_path) = &cli_args.srs_dir_path {
        config_values.srs_dir_path = srs_dir_path.clone();
    }
    if let Some(dest_dir_path) = &cli_args.dest_dir_path {
        config_values.dest_dir_path = dest_dir_path.clone();
    }
    if let Some(ffmpeg_dir_path) = &cli_args.ffmpeg_dir_path {
        config_values.ffmpeg_dir_path = ffmpeg_dir_path.clone();
    }
    if let Some(output_file_postfix) = &cli_args.output_file_postfix {
        config_values.output_file_postfix = output_file_postfix.clone();
    }
    if let Some(dep_time_correction) = &cli_args.dep_time_correction {
        config_values.dep_time_correction = *dep_time_correction;
    }
    if let Some(time_start_offset) = &cli_args.time_start_offset {
        config_values.time_start_offset = *time_start_offset;
    }
    if let Some(time_end_offset) = &cli_args.time_end_offset {
        config_values.time_end_offset = *time_end_offset;
    }
    if let Some(min_accel_trigger) = &cli_args.min_accel_trigger {
        config_values.min_accel_trigger = *min_accel_trigger;
    }

    println!("{:?}", config_values);
    config_values
}
