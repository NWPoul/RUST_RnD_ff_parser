


// use std::time::Instant;
use std::sync::{ Arc, atomic::AtomicBool };

use telemetry_parser::*;
use telemetry_parser::tags_impl::*;

use crate::analise::get_sma_list;


struct Opts {
    input: String,
    dump: bool,
    imuo: Option<String>,
}

    // let mut csv = String::with_capacity(2*1024*1024);
    // telemetry_parser::try_block!({
    //     let map = samples.get(0)?.tag_map.as_ref()?;
    //     let json = (map.get(&GroupId::Default)?.get_t(TagId::Metadata) as Option<&serde_json::Value>)?;
    //     for (k, v) in json.as_object()? {
    //         csv.push('"');
    //         csv.push_str(&k.to_string());
    //         csv.push_str("\",");
    //         csv.push_str(&v.to_string());
    //         csv.push('\n');
    //     }
    // });
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


pub fn parse_telemetry_from_mp4_file(input_file: &str) -> (Vec<(f64, f64, f64)>, Vec<f64>) {
    let opts: Opts = Opts {
        input: input_file.into(),
        dump: false,
        imuo: None,
    };

    let mut stream = std::fs::File::open(&opts.input).unwrap();
    let filesize = stream.metadata().unwrap().len() as usize;

    let input = Input::from_stream(&mut stream, filesize, &opts.input, |_|(), Arc::new(AtomicBool::new(false))).unwrap();

    println!("Detected camera: {} {}", input.camera_type(), input.camera_model().unwrap_or(&"".into()));

    let samples = input.samples.as_ref().unwrap();
    if opts.dump { dump_samples(samples);}

    let imu_data = util::normalized_imu(&input, opts.imuo).unwrap();

    let mut telemetry_xyz_acc_data: Vec<(f64, f64, f64)> = Vec::new();

    for v in imu_data {
        if v.accl.is_some() {
            let accl = v.accl.unwrap_or_default();
            telemetry_xyz_acc_data.push((accl[0], accl[1],accl[2]));
        }
    }

    let telemetry_sma_acc_data = get_sma_list(&telemetry_xyz_acc_data, 100);
    (
        telemetry_xyz_acc_data,
        telemetry_sma_acc_data,
    )
}
