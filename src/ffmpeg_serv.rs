
use std::process::{Command, Stdio};
use std::path::PathBuf;



pub const GLITCH_MARGIN: f64 = 3.0;


fn check_get_ffmpeg(ffmpeg_dir_path: &str) -> Result<PathBuf, std::io::Error> {
    let ffmpeg_path = PathBuf::from(ffmpeg_dir_path).join("ffmpeg.exe");
    if ffmpeg_path.exists() {
        return Ok(ffmpeg_path);
    }

    eprintln!("\nffmpeg not found at {:?}... trying sys PATH", &ffmpeg_path);
    let output = Command::new("ffmpeg")
        .arg("-version")
        .output();

    match output {
        Ok(_) => {
            println!("OK ffmpeg is in the system PATH");
            return Ok(PathBuf::from("ffmpeg"));
        },
        Err(_) => {
            println!("FAIL ffmpeg not in the system PATH");
            let error = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "ffmpeg not found!"
            );
            return Err(error);
        }
    }
}



pub fn run_ffmpeg(
    target_start_end_time: (f64, f64),
    files_path           : (&PathBuf, &PathBuf),
    ffmpeg_dir_path      : &str,
) -> Result<std::process::Child, std::io::Error> {
    let (mut start_time, end_time) = target_start_end_time;
    let (src_file_path, output_file_path) = files_path;

    let glitch_margin: f64 = if start_time >= GLITCH_MARGIN {
        GLITCH_MARGIN
    } else {
        start_time
    };

    start_time -= glitch_margin;

    let ffmpeg_path = check_get_ffmpeg(ffmpeg_dir_path)?;

    let ffmpeg_status = Command::new(&ffmpeg_path)
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