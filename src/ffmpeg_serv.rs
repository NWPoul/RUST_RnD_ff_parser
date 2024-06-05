
use std::process::{Command, Stdio};



pub const GLITCH_MARGIN: f64 = 3.0;



pub fn run_ffmpeg(
    target_start_end_time: (f64, f64),
    files_path: ( &std::path::PathBuf, &std::path::PathBuf ),
    ffmpeg_dir_path: &str,
) -> Result<std::process::Child, std::io::Error> {
    let (mut start_time, end_time)        = target_start_end_time;
    let (src_file_path, output_file_path) = files_path;

    let glitch_margin: f64 = if start_time >= GLITCH_MARGIN {
        GLITCH_MARGIN
    } else {
        start_time
    };

    start_time -= glitch_margin;

    let ffmpeg_status = Command::new(format!("{}{}", ffmpeg_dir_path, "/ffmpeg"))
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())

        .arg("-ss")
        .arg(start_time.to_string())

        .arg("-to")
        .arg(end_time.to_string())

        .arg("-i")
        .arg(src_file_path)

        .arg("-ss")
        .arg(glitch_margin.to_string())

        .arg("-c")
        .arg("copy")

        .arg(output_file_path)

        .arg("-y")

        .spawn();

    ffmpeg_status
}
