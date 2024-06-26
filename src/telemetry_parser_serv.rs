

use std::sync::{ Arc, atomic::AtomicBool };

use telemetry_parser::*;
use telemetry_parser::tags_impl::*;




struct Opts {
    input: String,
    dump: bool,
    imuo: Option<String>,
}


pub struct TelemetryData {
    pub cam_info: String,
    pub acc_data: Vec<(f64, f64, f64)>,
}



fn _get_additional_metadata(samples: &[util::SampleInfo]) -> Vec<(String, String)> {
    let mut additional_metadata: Vec<(String,String)> = vec![];
    telemetry_parser::try_block!({
        let map = samples.get(0)?.tag_map.as_ref()?;
        let json = (map.get(&GroupId::Default)?.get_t(TagId::Metadata) as Option<&serde_json::Value>)?;
        for (k, v) in json.as_object()? {
            additional_metadata.push(
               ( k.to_string(), v.to_string())
            );
        }
    });
    additional_metadata
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



pub fn parse_telemetry_from_file(input_file: &str) -> Result<TelemetryData, String> {
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
        Err(e) => {return Err(e.to_string());},
    };


    let input = match Input::from_stream(
        &mut stream,
        filesize,
        &opts.input,
        |_|(),
        Arc::new(AtomicBool::new(false))
    ) {
        Ok(input) => input,
        Err(e) => {return Err(format!("FAIL TO PARSE TELEMETRY! {}", e.to_string()));},
    };

    let cam_info = format!("{} {}",
        input.camera_type(),
        input.camera_model().unwrap_or(&"".into()),
    );
    println!("Detected camera: {cam_info}");

    let samples = match input.samples.as_ref() {
        Some(samples_ref) => samples_ref,
        None => {return Err(format!("NO_SAMPLES!"))},
    };


    if opts.dump { dump_samples(samples);}

    let imu_data = match util::normalized_imu(&input, opts.imuo) {
        Ok(data) => data,
        Err(e) => {return Err(format!("FAIL TO GET IMUDATA! {}", e.to_string()));},
    };

    let mut telemetry_xyz_acc_data: Vec<(f64, f64, f64)> = Vec::new();

    for v in imu_data {
        if v.accl.is_some() {
            let accl = v.accl.unwrap_or_default();
            telemetry_xyz_acc_data.push((accl[0], accl[1],accl[2]));
        }
    }

    Ok(TelemetryData {
        cam_info: cam_info,
        acc_data: telemetry_xyz_acc_data,
    })
}
