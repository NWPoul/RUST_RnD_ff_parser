


// use std::time::Instant;
use std::sync::{ Arc, atomic::AtomicBool };

use telemetry_parser::*;
use telemetry_parser::tags_impl::*;

struct Opts {
    input: String,
    dump: bool,
    imuo: Option<String>,
}


pub struct TelemetryParsedData {
    pub cam_info : String,
    pub acc_data : Vec<(f64, f64, f64)>,
    pub gyro_data: Vec<(f64, f64, f64)>,
}





fn _get_additional_metadata(samples: &[util::SampleInfo]) {
    let mut csv = String::with_capacity(2*1024*1024);
    telemetry_parser::try_block!({
        let map = samples.get(0)?.tag_map.as_ref()?;
        let json = (map.get(&GroupId::Default)?.get_t(TagId::Metadata) as Option<&serde_json::Value>)?;
        for (k, v) in json.as_object()? {
            csv.push('"');
            csv.push_str(&k.to_string());
            csv.push_str("\",");
            csv.push_str(&v.to_string());
            csv.push('\n');
        }
    });
}


fn dump_samples(samples: &[util::SampleInfo]) {
    for info in samples {
        if info.tag_map.is_none() { continue; }
        let grouped_tag_map = info.tag_map.as_ref().unwrap();

        for (group, map) in grouped_tag_map {
            for (tagid, taginfo) in map {
                println!("{: <25} {: <25} {: <50}: {}", format!("{}", group), format!("{}", tagid), taginfo.description, &taginfo.value.to_string());
            }
        }
    }
}



pub fn parse_telemetry_from_mp4_file(input_file: &str) -> Result<TelemetryParsedData, String> {
    let opts: Opts = Opts {
        input: input_file.into(),
        dump: false,
        imuo: None,
    };

    let mut stream = match std::fs::File::open(&opts.input) {
        Ok(stream) => stream,
        Err(e) => {return Err(e.to_string());},
    };

    let filesize = match stream.metadata() {
        Ok(metadata) => metadata.len() as usize,
        Err(e) => {return Err(format!("NO_METADATA! {}", e.to_string()));},
    };

    let input = Input::from_stream(&mut stream, filesize, &opts.input, |_|(), Arc::new(AtomicBool::new(false))).unwrap();

    let cam_info = format!("{} {}", input.camera_type(), input.camera_model().unwrap_or(&"".into()));
    println!("Detected camera: {cam_info}");

    let samples = match input.samples.as_ref() {
        Some(samples_ref) => samples_ref,
        None => {return Err(format!("NO_SAMPLES!"))},
    };

    if opts.dump { dump_samples(samples);}

    let imu_data = match util::normalized_imu(&input, opts.imuo) {
        Ok(data) => data,
        Err(e)   => {return Err(format!("FAIL TO GET IMUDATA! {}", e.to_string()));},
    };

    let mut telemetry_xyz_acc_data: Vec<(f64, f64, f64)> = Vec::new();
    let mut telemetry_xyz_gyro_data: Vec<(f64, f64, f64)> = Vec::new();

    for v in imu_data {
        if v.accl.is_some() {
            let accl = v.accl.unwrap_or_default();
            telemetry_xyz_acc_data.push((accl[0], accl[1],accl[2]));
        }
        if v.gyro.is_some() {
            let gyro = v.gyro.unwrap_or_default();
            telemetry_xyz_gyro_data.push((gyro[0], gyro[1],gyro[2]));
        }
    }

    Ok(TelemetryParsedData {
        cam_info : cam_info,
        acc_data : telemetry_xyz_acc_data,
        gyro_data: telemetry_xyz_gyro_data,
    })
}

pub fn get_result_metadata_for_file(input_file: &str) -> Result<TelemetryParsedData, String> {
    let telemetry_data = parse_telemetry_from_mp4_file(input_file)?;
    Ok(TelemetryParsedData{
        cam_info : telemetry_data.cam_info,
        acc_data : telemetry_data.acc_data,
        gyro_data: telemetry_data.gyro_data,
    })
}

// let telemetry_sma_acc_data = get_sma_list(&telemetry_xyz_acc_data, 100);