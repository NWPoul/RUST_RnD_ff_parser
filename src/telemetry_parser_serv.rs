


// use std::time::Instant;
use std::sync::{ Arc, atomic::AtomicBool };
use std::ops::{Add, Div};

use telemetry_parser::*;
use telemetry_parser::tags_impl::*;

use crate::utils::u_serv::Vector3d;


pub trait NumericData: Into<f64> + Add<Output=Self> + Div<Output=Self> + Copy {}
impl<T> NumericData for T where T: Into<f64> + Add<Output=T> + Div<Output=T> + Copy {}

struct Opts {
    input: String,
    dump: bool,
    imuo: Option<String>,
}

pub struct iso_data {
    pub tick: f64,
    pub vals: Vec<f64>,
}
pub struct TelemetryParsedData {
    pub cam_info : String,
    pub acc_data : Vec<Vector3d>,
    pub gyro_data: Vec<Vector3d>,
    pub iso_data : iso_data,
}



pub const DEF_TICK: f64 = 0.005;



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

    let cam_info = format!("{} {}",
        input.camera_type(),
        input.camera_model().unwrap_or(&"".into()),
    );
    println!("Detected camera: {cam_info}");

    let samples = match input.samples.as_ref() {
        Some(samples_ref) => samples_ref,
        None => {return Err(format!("NO_SAMPLES!"))},
    };
    let iso_data = get_iso_data(&input);

    if opts.dump { dump_samples(samples)}

    let imu_data = match util::normalized_imu(&input, opts.imuo) {
        Ok(data) => data,
        Err(e)   => {return Err(format!("FAIL TO GET IMUDATA! {}", e.to_string()));},
    };

    let mut telemetry_xyz_acc_data : Vec<Vector3d> = Vec::new();
    let mut telemetry_xyz_gyro_data: Vec<Vector3d> = Vec::new();

    for v in imu_data {
        if v.accl.is_some() {
            let accl = v.accl.unwrap_or_default();
            telemetry_xyz_acc_data.push(Vector3d::new(accl[0], accl[1], accl[2]));
        }
        if v.gyro.is_some() {
            let gyro = v.gyro.unwrap_or_default();
            telemetry_xyz_gyro_data.push(Vector3d::new(gyro[0], gyro[1],gyro[2]));
        }
    }

    Ok(TelemetryParsedData {
        cam_info : cam_info,
        acc_data : telemetry_xyz_acc_data,
        gyro_data: telemetry_xyz_gyro_data,
        iso_data : iso_data.expect("msg"),
    })
}

pub fn get_result_metadata_for_file(input_file: &str) -> Result<TelemetryParsedData, String> {
    let telemetry_data = parse_telemetry_from_mp4_file(input_file)?;
    Ok(TelemetryParsedData{
        cam_info : telemetry_data.cam_info,
        acc_data : telemetry_data.acc_data,
        
        gyro_data: telemetry_data.gyro_data,
        iso_data : telemetry_data.iso_data,
    })
}







pub fn get_iso_data(input: &Input) -> std::io::Result<iso_data> {
    // let mut timestamp = 0f64;
    // let accurate_ts = input.has_accurate_timestamps();

    let mut final_data_vals = Vec::<f64>::with_capacity(10000);
    let mut final_data = iso_data{
        vals: final_data_vals,
        tick: DEF_TICK,
    };

    if let Some(ref samples) = input.samples {
        for info in samples {
            if info.tag_map.is_none() { continue; }
            let duration = info.duration_ms;

            let grouped_tag_map = info.tag_map.as_ref().unwrap();







            fn convert_array_to_f64<T>(arr: &[T]) -> Vec<f64>
            where T: Into<f64>{
                arr.iter().map(|x| *x as f64).collect()
            }
            
            fn perform_operations(arr: Vec<f64>) {
                // Perform identical operations on arr here
                let tick = 30. / arr.len() as f64;
                
                // You can add more operations here if needed
            }







            for (group, map) in grouped_tag_map {
                if group == &GroupId::Custom("SensorISO".to_string()) {
                    if let Some(taginfo) = map.get(&TagId::Data) {
                        // match &taginfo.value {
                        // // dbg!(&taginfo.value);
                        //     // Sony and GoPro
                        //     TagValue::Vec_u32(arr) => {
                        //         let arr:Vec<f64> = arr.get().into_iter().map(|x| *x as f64).collect();
                        //         let tick = duration / arr.len() as f64;
                        //         // dbg!(tick);
                        //         // final_data.vals.extend(arr);
                        //         // final_data.tick = tick;
                        //     },
                        //     TagValue::Vec_u16(arr) => {
                        //         let arr:Vec<f64> = arr.get().into_iter().map(|x| *x as f64).collect();
                        //         let tick = duration / arr.len() as f64;
                        //         // dbg!(tick);
                        //         // final_data.vals.extend(arr);
                        //         // final_data.tick = tick;
                                

                        //     },
                            
                        //     _ => {
                        //         dbg!("NOT Vec_u32!!! {}", &taginfo.value);
                        //         ()
                        //     }
                        // }
                        match &taginfo.value {
                            TagValue::Vec_u32(arr) | TagValue::Vec_u16(arr) => {
                                let converted_arr = convert_array_to_f64(arr);
                                perform_operations(converted_arr);
                            }
                            _ => {
                                dbg!("NOT Vec_u32 or u16!!! {}", &taginfo.value);
                                ()
                            }
                        }
                    }
                }
                if group == &GroupId::Exposure {
                    if let Some(taginfo) = map.get(&TagId::Data) {
                        dbg!("EXPOSURE {}");//, &taginfo.value);
                        match &taginfo.value {
                            // Sony and GoPro
                            TagValue::Vec_f32(arr) => {
                                let arr:Vec<f64> = arr.get().into_iter().map(|x| *x as f64).collect();
                                let tick = duration / arr.len() as f64;
                                dbg!(tick, &arr);
                                final_data.vals.extend(arr);
                                final_data.tick = tick;
                            },
                           
                            _ => {
                                dbg!("EXPOSURE NOT Vec_u32!!! {}", &taginfo.value);
                                ()
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(final_data)
}


//      let mut timestamp = 0f64;
//     let mut first_timestamp = None;
//     let accurate_ts = input.has_accurate_timestamps();

//     let mut final_data = Vec::<IMUData>::with_capacity(10000);
//     let mut data_index = 0;

//     let mut fix_timestamps = false;

//     for info in samples {
//         if info.tag_map.is_none() { continue; }

//         let grouped_tag_map = info.tag_map.as_ref().unwrap();

//         for (group, map) in grouped_tag_map {
//             if group == &GroupId::Exposure {
//                 // dbg!(map.get(&TagId::Data));
//             };
//             if group == &GroupId::Custom("SensorISO".to_string()) {
//                 if let Some(taginfo) = map.get(&TagId::Data) {
//                     match &taginfo.value {
//                         TagValue::Vec_Vec_i16(arr) => {
//                             let arr = arr.get();


macro_rules! try_block {
    ($type:ty, $body:block) => {
        (|| -> Option<$type> {
            Some($body)
        }())
    };
    ($body:block) => {
        (|| -> Option<()> {
            $body
            Some(())
        }())
    };
}