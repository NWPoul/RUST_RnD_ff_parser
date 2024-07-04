use std::fs;
use std::path::PathBuf;




pub fn save_log_to_txt(max_accel_data_list: &Vec<(f64, f64, f64, f64)>, file_path: &PathBuf) {
    use std::fs::File;
    use std::io::Write;

    let srs_file_name = file_path.file_name().unwrap().to_str().unwrap();
    let log_file_name = format!("max_accel_{}.txt", srs_file_name);

    let mut file = File::create(log_file_name).expect("Failed to create file");

    for data in max_accel_data_list.iter() {
        let (sec, acc_data_max, acc_data_skv, acc_data_v_avg) = data;
        writeln!(
            file,
            "{:?}\t{:?}\t{:?}\t{:?}",
            sec.trunc() as u64,
            acc_data_max.round() as u64,
            acc_data_skv.round() as u64,
            acc_data_v_avg.round() as u64
        )
        .expect("Failed to write to file");
    }
}

pub fn save_det_log_to_txt(data_list: &Vec<(f64,f64,f64)>, file_path: &PathBuf) {
    use std::fs::File;
    use std::io::Write;

    let srs_file_name = file_path.file_name().unwrap().to_str().unwrap();
    let log_file_name = format!("LOG_accel_{}.txt", srs_file_name);

    let mut file = File::create(log_file_name).expect("Failed to create file");

    for data in data_list.iter() {
        writeln!(
            file,
            "{:?}\t{:?}\t{:?}\t", data.0, data.1, data.2)
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