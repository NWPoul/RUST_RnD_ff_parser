// struct ConfigValues {
//     srs_dir_path: String,
//     dest_dir_path: String,
//     ffmpeg_dir_path: String,
//     output_file_postfix: String,
//     dep_time_correction: f64,
//     time_start_offset: f64,
//     time_end_offset: f64,
//     min_accel_trigger: f64,
// }

// fn old_get_config_values() -> ConfigValues {
//     let mut settings = Config::default();
//     if let Err(e) = settings.merge(CfgFile::with_name("config.toml")) {
//         println!("Failed to load configuration file: {}", e);
//         println!("default configuration used");
//     }
//     println!("Config loaded from file");
//     let srs_dir_path = settings
//         .get::<String>("srs_dir_path")
//         .unwrap_or(DEF_DIR.into());
//     let dest_dir_path = settings
//         .get::<String>("dest_dir_path")
//         .unwrap_or(DEF_DIR.into());
//     let ffmpeg_dir_path = settings
//         .get::<String>("ffmpeg_dir_path")
//         .unwrap_or(DEF_DIR.into());
//     let output_file_postfix = settings
//         .get::<String>("output_file_postfix")
//         .unwrap_or(DEF_POSTFIX.into());
//     let dep_time_correction = settings
//         .get::<f64>("dep_time_correction")
//         .unwrap_or(DEP_TIME_CORRECTION);
//     let time_start_offset = settings
//         .get::<f64>("time_start_offset")
//         .unwrap_or(TIME_START_OFFSET);
//     let time_end_offset = settings
//         .get::<f64>("time_end_offset")
//         .unwrap_or(TIME_END_OFFSET);
//     let min_accel_trigger = settings
//         .get::<f64>("min_accel_trigger")
//         .unwrap_or(MIN_ACCEL_TRIGGER);
//     println!("Dir paths\n src: {}\n dest: {}\n ffmpeg: {}",
//         srs_dir_path, dest_dir_path, ffmpeg_dir_path);
//     println!("Dep Time Correction: {}", dep_time_correction);
//     println!("Time Start Offset: {}", time_start_offset);
//     println!("Time End Offset: {}", time_end_offset);
//     println!("Min Accel Trigger: {}", min_accel_trigger);
//     println!("Output postfix: {}", output_file_postfix);
//     println!("");
//     ConfigValues {
//         srs_dir_path,
//         dest_dir_path,
//         ffmpeg_dir_path,
//         dep_time_correction,
//         time_start_offset,
//         time_end_offset,
//         min_accel_trigger,
//         output_file_postfix,
//     }
// }

#[macro_export]
macro_rules! promptExit {
    ($msg: expr) => {
        crate::utils::u_serv::prompt_to_exit($msg);
        return;
    };
}

#[macro_export]
macro_rules! promptContinue {
    ($msg: expr) => {
        let confirm = crate::utils::u_serv::prompt_to_continue($msg);
        if confirm {continue} else {return};
    };
}
// macro_rules! promptExit_Ok {
//     ($msg: expr) => {
//         crate::utils::u_serv::prompt_to_exit($msg);
//         return Ok(());
//     };
// }


#[macro_export]
macro_rules! configValues {
    ($(($var:ident, $type:ty, $default:expr)),*) => {
        #[derive(Debug, Clone)]
        pub struct ConfigValues {
            $(pub $var:$type),*
        }

        pub fn get_config_values() -> ConfigValues {
            let mut settings = Config::default();

            if let Err(e) = settings.merge(CfgFile::with_name("config.toml")) {
                println!("Failed to load configuration file: {}", e);
                println!("default configuration used");
            }
            println!("Config loaded from file");

            $(
                let $var = settings
                    .get::<$type>(stringify!($var))
                    .unwrap_or($default);
                println!(concat!(stringify!($var), ": {}"), $var);
            )*
            println!();

            ConfigValues {
                $($var),*
            }
        }
    };
}
