// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright © 2021 Adrian <adrian.eddy at gmail>

use std::time::Instant;
use std::sync::{ Arc, atomic::AtomicBool };

use telemetry_parser::*;
use telemetry_parser::tags_impl::*;

/** gyro2bb v0.2.8
Author: Adrian <adrian.eddy@gmail.com>

Extract gyro data from Sony, GoPro and Insta360 cameras to betaflight blackbox csv log
*/
struct Opts {
    input: String,
    dump: bool,
    imuo: Option<String>,
}

const INPUT_11_MINI: &str = "D:\\DEV\\VIDEO_TEMP\\GX019575_JAY.mp4";
const INPUT_H9:      &str = "D:\\DEV\\VIDEO_TEMP\\GX019345.mp4";



fn main() {
    // let opts: Opts = argh::from_env();
    let opts: Opts = Opts {
        input: INPUT_H9.into(),
        dump: false,
        imuo: None,
    };
    let _time = Instant::now();

    let mut stream = std::fs::File::open(&opts.input).unwrap();
    let filesize = stream.metadata().unwrap().len() as usize;

    let input = Input::from_stream(&mut stream, filesize, &opts.input, |_|(), Arc::new(AtomicBool::new(false))).unwrap();

    let mut i = 0;
    println!("Detected camera: {} {}", input.camera_type(), input.camera_model().unwrap_or(&"".into()));

    let samples = input.samples.as_ref().unwrap();

    if opts.dump {
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

    let imu_data = util::normalized_imu(&input, opts.imuo).unwrap();

    let mut csv = String::with_capacity(2*1024*1024);
    crate::try_block!({
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

    csv.push_str(r#""N","time","accSmooth[0]","accSmooth[1]","accSmooth[2]""#);
    csv.push('\n');
    for v in imu_data {
        if v.accl.is_some() {
            let accl = v.accl.unwrap_or_default();
            csv.push_str(&format!("{},{:.0},{},{},{}\n", i, (v.timestamp_ms).round(),
                -accl[2], accl[1], accl[0]
            ));
            i += 1;
        }
    }
    std::fs::write(&format!("{}.csv", std::path::Path::new(&opts.input).to_path_buf().to_string_lossy()), csv).unwrap();

    println!("Done in {:.3} ms", _time.elapsed().as_micros() as f64 / 1000.0);
}
