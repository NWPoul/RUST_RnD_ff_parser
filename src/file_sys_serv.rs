use std::fs;
use std::path::PathBuf;
use rfd::FileDialog;


use gpmf_rs::SensorData;


pub fn extract_filename(path: &PathBuf) -> String {
    path
       .file_name()
       .and_then(|name| name.to_str())
       .map(String::from)
       .unwrap_or_else(|| String::from("<unknown>"))
}


pub fn convert_to_absolute(dest_dir: &str) -> Result<PathBuf, std::io::Error> {
    fs::canonicalize(PathBuf::from(dest_dir))
}


pub fn save_log_to_txt(max_accel_data_list: &Vec<(f64, f64, f64)>, file_path: &PathBuf) {
    use std::fs::File;
    use std::io::Write;

    let srs_file_name = file_path.file_name().unwrap().to_str().unwrap();
    let log_file_name = format!("max_accel_{}.txt", srs_file_name);

    let mut file = File::create(log_file_name).expect("Failed to create file");

    for data in max_accel_data_list.iter() {
        let (sec, acc_data_max, acc_data_skv) = data;
        writeln!(
            file,
            "{:?}\t{:?}\t{:?}", sec.trunc() as u64, acc_data_max.round() as u64, acc_data_skv.round() as u64)
            .expect("Failed to write to file");
    }
}

pub fn save_det_log_to_txt(data_list: &Vec<f64>, file_path: &PathBuf) {
    use std::fs::File;
    use std::io::Write;

    let srs_file_name = file_path.file_name().unwrap().to_str().unwrap();
    let log_file_name = format!("LOG_accel_{}.txt", srs_file_name);

    let mut file = File::create(log_file_name).expect("Failed to create file");

    for data in data_list.iter() {
        writeln!(
            file,
            "{:?}", data.round() as u64)
            .expect("Failed to write to file");
    }
}

pub fn save_gsensor_data(data_list: Vec<SensorData>, file_path: &PathBuf) {
    use std::fs::File;
    use std::io::Write;

    let srs_file_name = file_path.file_name().unwrap().to_str().unwrap();
    let log_file_name = format!("GSENSOR_DATAl_{}.txt", srs_file_name);

    let mut file = File::create(log_file_name).expect("Failed to create file");

    for data in data_list.iter() {
        writeln!(
            file,
            "{:?}", data.to_owned()
        )
        .expect("Failed to write to file");
    }
}



pub fn get_src_file_path(srs_dir_path: &str) -> Option<PathBuf> {
    let paths = fs::read_dir(srs_dir_path)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();
            path.extension()
                .and_then(|ext| ext.to_str().map(|s| s.to_lowercase() == "mp4"))
                .unwrap_or(false)
        })
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    if !paths.is_empty() {
        Some(paths[0].to_owned())
    } else {
        None
    }
}


pub fn get_src_files_path_list(srs_dir_path: &str) -> Option<Vec<PathBuf>> {
    let src_files_path_list = FileDialog::new()
        .add_filter("mp4_files", &["mp4", "MP4"])
        .set_directory(srs_dir_path)
        .pick_files();
    src_files_path_list
}


pub fn get_output_filename(
    src_file_path      : &PathBuf,
    dest_dir_path      : &str,
    output_file_postfix: &str
) -> PathBuf {
    let mut dest_dir_path = PathBuf::from(dest_dir_path);
    if !dest_dir_path.exists() {
        println!("dest_dir_path : {:?} don't exist\n", dest_dir_path);
        dest_dir_path = PathBuf::from(src_file_path.parent().unwrap())
    }
    let output_file_name = format!(
        "{}{}.mp4",
        src_file_path.file_stem().unwrap().to_str().unwrap(),
        output_file_postfix
    );

    let output_file_path = dest_dir_path.join(&output_file_name);

    // if output_file_path.exists() {
    //     println!("NEW output_file_path: {:?}", output_file_path);
        // let original_extension = output_file_path.extension().unwrap_or_default();
        // let new_extension = format!(".copy.{}", original_extension.to_str().unwrap());
        // let new_file_name = PathBuf::from(output_file_path.file_name().unwrap()).with_extension(new_extension);
        // output_file_path.set_file_name(new_file_name);
    // }

    output_file_path
}