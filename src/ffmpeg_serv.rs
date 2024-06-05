
use std::process::{Command, Stdio};

use crate:: ConfigValues;



pub const GLITCH_MARGIN: f64 = 3.0;



pub fn run_ffmpeg(
    config_values    : ConfigValues,
    mut target_start_time: f64,
    target_end_time  : f64,
    src_file_path    : &std::path::PathBuf,
    output_file_path : &std::path::PathBuf
) -> Result<std::process::Child, std::io::Error> {

    let glitch_margin:f64 = if target_start_time >= GLITCH_MARGIN {
        GLITCH_MARGIN
    } else {
        target_start_time
    };

    target_start_time = target_start_time - GLITCH_MARGIN;



    let ffmpeg_status = Command::new(format!("{}{}", config_values.ffmpeg_dir_path, "/ffmpeg"))
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())

        .arg("-ss").arg(target_start_time.to_string())
        .arg("-to").arg(target_end_time.to_string())

        .arg("-i").arg(&src_file_path)

        .arg("-ss").arg(glitch_margin.to_string())
        .arg("-c").arg("copy")

        .arg(output_file_path)
        .arg("-y")

        .spawn();
    ffmpeg_status
}