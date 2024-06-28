use std::{
    collections::HashSet,
    fs::{ self, File },
    io::{ Read, Write },
    path::{ Path, PathBuf },
    time::Duration
};

use rfd::FileDialog;
use crossbeam_channel::{Sender, Receiver};


use crate::{GREEN, BOLD, RESET};





fn check_path<T: AsRef<Path>>(path: T) -> bool {
    let path_buf = PathBuf::from(path.as_ref());
    path_buf.exists()
}


pub fn extract_filename<T: AsRef<Path>>(path: T) -> String {
    let path_buf = PathBuf::from(path.as_ref());
    let filename = path_buf
        .file_name()
        .unwrap_or(std::ffi::OsStr::new("default"))
        .to_string_lossy();
    filename.into()
}


fn convert_to_absolute_path_or_default<T: AsRef<Path>>(path: T) -> PathBuf {
    let def_path = PathBuf::from(".");
    let path     = PathBuf::from(path.as_ref());
    fs::canonicalize(path).unwrap_or(
        fs::canonicalize(def_path).unwrap()
    )
}


pub fn get_src_file_path(srs_dir_path: &PathBuf) -> Option<PathBuf> {
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


pub fn get_src_files_path_list<T: AsRef<Path>>(srs_dir_path: T) -> Option<Vec<PathBuf>> {
    let src_files_path_list = FileDialog::new()
        .add_filter("mp4_files", &["mp4", "MP4"])
        .set_directory(srs_dir_path)
        .set_can_create_directories(true)
        .pick_files();
    src_files_path_list
}


pub fn get_output_abs_dir(dest_dir_path: &PathBuf) -> PathBuf {
    convert_to_absolute_path_or_default(dest_dir_path)
}

pub fn get_output_file_path(
    src_file_path      : &PathBuf,
    dest_dir_path      : &PathBuf,
    output_file_postfix: &str,
    device_info        : &str,
) -> PathBuf {
    let def_path = PathBuf::from(".");
    let dest_dir_path = get_output_abs_dir(dest_dir_path);
    let output_file_name = format!(
        "{} {}{}.mp4",
        device_info,
        src_file_path.file_stem().unwrap_or(&def_path.into_os_string()).to_str().unwrap(),
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



pub fn get_current_drives() -> HashSet<String> {
    let mut drives = HashSet::new();
    for letter in 'A'..='Z' {
        let drive_path = format!("{}:\\", letter);
        if Path::new(&drive_path).exists() {
            drives.insert(drive_path);
        }
    }
    drives
}


pub fn get_src_path_for_ext_drive(drivepath_str: &PathBuf) -> PathBuf {
    let dcim_path  = format!("{:?}\\DCIM", drivepath_str);
    let gopro_path = format!("{}\\100GOPRO", dcim_path);
        // println!("gopro_path: {gopro_path}");
        if check_path(&gopro_path) {
            return gopro_path.into();
        }
        if check_path(&dcim_path) {
            return dcim_path.into();
        }
        drivepath_str.into()
}




pub fn copy_with_progress(
    src_file_path : &PathBuf,
    dest_file_path: &PathBuf,
) -> std::io::Result<()> {
    let mut src_file  = File::open(src_file_path)?;
    let mut dest_file = File::create(dest_file_path)?;

    let mut buffer = vec![0; 8_388_608];
    let total_bytes_to_copy = std::fs::metadata(src_file_path)?.len();
    let mut bytes_copied = 0;

    loop {
        let n = src_file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        bytes_copied += n;
        let progress = (bytes_copied as f64 / total_bytes_to_copy as f64) * 100.0;
        std::io::stdout().flush().unwrap();
        print!("Copying progress: {}%\r", progress.trunc());

        dest_file.write_all(&buffer[..n])?;
    }

    Ok(())
}



pub fn open_output_folder<T: AsRef<Path>>(config_dest_dir: T) {
    let path = PathBuf::from(config_dest_dir.as_ref());
    let output_dir_path = get_output_abs_dir(&path);
    let _ = open_folder(&output_dir_path);
}

#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
pub fn open_folder(folder_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let explorer_command  = "explorer.exe";
    let full_path = folder_path.canonicalize()?;

    let base_url = format!( "file://{:?}",
        full_path.clone()
            .into_os_string()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>().as_slice()
    );
    println!("base_url: {base_url}");

    let last_modified_file = fs::read_dir(full_path)?
       .filter_map(Result::ok)
       .filter(|e| e.metadata().unwrap().is_file())
       .max_by_key(|e| e.metadata().unwrap().modified().unwrap());


       println!("last_modified_file: {:?}", &last_modified_file);

    match last_modified_file {
        Some(file_entry) => {
            let file_path = file_entry.path();
            let os_string = file_path.into_os_string();
            let url = format!("{}/{:?}", base_url, os_string);

            println!("file to select: {:?}", &url);
            std::process::Command::new(explorer_command)
              .arg(url)
              .spawn()?;
        },
        None => {
            // Handle the case where the directory is empty or contains only directories
            println!("No files found in the directory.");
        }
    };

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn open_folder(folder_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let command = config.get_string("open_folder_command")?;
    let full_path = Path::new(folder_path).canonicalize()?;
    std::process::Command::new(command)
        .arg(full_path)
        .spawn()?;
    Ok(())
}


fn watch_drives_loop(rx: Receiver<()>) -> Option<PathBuf> {
    let mut known_drives = get_current_drives();
    println!("\nInitial Drives: {:?}", known_drives);
    println!("{BOLD}{GREEN}WHATCHING FOR NEW DRIVE / CARD...\n(press 'ENTER' if want to open file dialog){RESET}");

    let cur_dir = None;
    loop { //'drivers_loop:
        let current_drives = get_current_drives();

        for drive in &current_drives {
            if!known_drives.contains(drive) {
                println!("{BOLD}{GREEN}New drive detected: {}{RESET}", drive);
                match fs::read_dir(drive) {
                    Ok(_entries) => {
                        return Some(get_src_path_for_ext_drive(&drive.as_str().into()));
                    }
                    Err(e) => {
                        println!("Error reading drive {}: {}", drive, e);
                        return  None;
                    },
                }
            }
        }

        for drive in &known_drives {
            if!current_drives.contains(drive) {
                println!("Drive removed: {}", drive);
            }
        }

        known_drives = current_drives;

        std::thread::sleep(Duration::from_secs(1));

        if rx.try_recv().is_ok() {
            break;
        }
    }
    cur_dir
}




pub fn watch_drivers(tx: Sender<()>, rx: Receiver<()>) -> Option<PathBuf> {
    let handle_whatch_drivers_loop = std::thread::spawn(move || watch_drives_loop(rx));

    let _handle_whatch_input_loop = std::thread::spawn(move || {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        match tx.send(()) {
            Ok(_)  => {}
            Err(e) => {
                println!("Failed to send message to watch_drivers_loop: {}\n ", e);
                println!("{BOLD}{GREEN}Press 'Enter' to continue...{RESET}");
                ()
            }
        };
    });

    let dir_path = handle_whatch_drivers_loop.join().unwrap();
    println!("END whatch_drivers_loop {:?}", dir_path);
    dir_path
}








