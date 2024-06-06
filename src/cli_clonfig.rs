
use clap::Parser;

use crate::ConfigValues;


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[arg(short)]
    only_dir: Option<String>,

    #[arg(short)]
    srs_dir: Option<String>,
    #[arg(short)]
    dest_dir: Option<String>,
    #[arg(short)]
    ffmpeg_dir: Option<String>,
    #[arg(short)]
    postfix: Option<String>,
    #[arg(short)]
    min_accel: Option<f64>,
}



pub fn get_cli_merged_config(mut config_values: ConfigValues) -> ConfigValues {
    let cli_args = CliArgs::parse();

    if let Some(arg) = &cli_args.only_dir {
        config_values.srs_dir_path    = arg.clone();
        config_values.dest_dir_path   = arg.clone();
        config_values.ffmpeg_dir_path = arg.clone();
    } else {
        if let Some(arg) = cli_args.srs_dir {
            config_values.srs_dir_path = arg;
        }
        if let Some(arg) = cli_args.dest_dir {
            config_values.dest_dir_path = arg;
        }
        if let Some(arg) = cli_args.ffmpeg_dir {
            config_values.ffmpeg_dir_path = arg;
        }
    }

    if let Some(arg) = cli_args.postfix {
        config_values.output_file_postfix = arg;
    }
    if let Some(arg) = cli_args.min_accel {
        config_values.min_accel_trigger = arg;
    }

    println!("{:?}", config_values);
    config_values
}
