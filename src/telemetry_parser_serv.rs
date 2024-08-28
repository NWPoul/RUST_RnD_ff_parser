


// use std::time::Instant;
use std::sync::{ Arc, atomic::AtomicBool };
use std::ops::{Add, Div};

use telemetry_parser::Input as TpInput;
use telemetry_parser::util as tp_util;
use telemetry_parser::tags_impl::{
    GroupId,
    TagId,
    TagValue,
    GetWithType,
};

use crate::utils::u_serv::Vector3d;



struct Opts {
    input: String,
    dump: bool,
    imuo: Option<String>,
}


pub struct IsoData {
    pub ts  : Vec<f64>,
    pub vals: Vec<f64>,
}
impl IsoData {
    pub fn new_with_capacity(capacity: usize) -> Self {
        IsoData{
            ts  : Vec::<f64>::with_capacity(capacity),
            vals: Vec::<f64>::with_capacity(capacity),
        }
    }

    pub fn add_vals(&mut self, new_vals: &[f64], step: f64) {
        let start_ts = *self.ts.last().unwrap_or(&0.0) + step;
        let new_ts: Vec<f64> = (0..new_vals.len()).map(|i| start_ts + (i as f64) * step).collect();
        self.ts.extend(new_ts);
        self.vals.extend_from_slice(new_vals);
    }
}


struct CameraInfo {
    model : String,
    serial: Option<String>,
}

pub struct TelemetryParsedData {
    pub file_name: String,
    pub cam_info : CameraInfo,
    pub acc_data : Vec<Vector3d>,
    pub iso_data : (IsoData, IsoData),
}



pub const DEF_TICK: f64 = 0.005;



fn _get_additional_metadata(samples: &[tp_util::SampleInfo]) {
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


fn dump_samples(samples: &[tp_util::SampleInfo]) {
    for info in samples {
        if info.tag_map.is_none() { continue; }
        let grouped_tag_map = info.tag_map.as_ref().unwrap();

        for (group, map) in grouped_tag_map {
            for (tagid, taginfo) in map {
                println!("{: <25} {: <20} {: <10}: {}", format!("{}", group), format!("{}", tagid), taginfo.description, &taginfo.value.to_string().chars().take(50).collect::<String>());
            }
        }
    }
}

fn get_cam_serial(sample_0: &tp_util::SampleInfo) -> Option<String> {
    if let Some(grouped_tag_map) = sample_0.tag_map.as_ref() {
        for map in grouped_tag_map.values() {
            if let Some(taginfo) = map.values().find(|taginfo| taginfo.description == "CASN") {
                return Some(taginfo.value.to_string());
            }
        }
    }
    println!("NO CASN");
    return None
}
fn get_cam_info(input: &TpInput) -> CameraInfo {
    let mut cam_model  = "".to_string();
    let mut cam_serial = None;

    if let Some(model) = input.camera_model() {
        cam_model = model.to_string();
    };
    if let Some(samples) = &input.samples {
        cam_serial = get_cam_serial(&samples[0]);
    };

    println!("Detected camera: {cam_model} {:?}", &cam_serial);

    CameraInfo{
        model : cam_model.into(),
        serial: cam_serial,
    }
    // if opts.dump { dump_samples(&samples[0..2])}
    // dump_samples(&samples.clone()[0..1]);
    // dump_samples(&samples[0..2]);
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

    let input = TpInput::from_stream(&mut stream, filesize, &opts.input, |_|(), Arc::new(AtomicBool::new(false))).unwrap();
 
    let cam_info = get_cam_info(&input);

    let iso_data = get_iso_data(&input);

    let mut telemetry_xyz_acc_data : Vec<Vector3d> = Vec::new();

    let imu_data = match tp_util::normalized_imu_interpolated(&input, opts.imuo) {
        Ok(data) => data,
        Err(e)   => {return Err(format!("FAIL TO GET IMUDATA! {}", e.to_string()));},
    };

    for v in imu_data {
        if v.accl.is_some() {
            let accl = v.accl.unwrap_or_default();
            telemetry_xyz_acc_data.push(Vector3d::new(accl[0], accl[1], accl[2]));
        }
    }

    Ok(TelemetryParsedData {
        file_name: input_file.to_string(),
        cam_info : cam_info,
        acc_data : telemetry_xyz_acc_data,
        iso_data : iso_data.expect("msg"),
    })
}

pub fn get_result_metadata_for_file(input_file: &str) -> Result<TelemetryParsedData, String> {
    let telemetry_data = parse_telemetry_from_mp4_file(input_file)?;
    Ok(TelemetryParsedData{
        file_name: input_file.to_string(),
        cam_info : telemetry_data.cam_info,
        acc_data : telemetry_data.acc_data,
        
        iso_data : telemetry_data.iso_data,
    })
}







pub fn convert_array_to_f64<T: Into<f64> + Copy>(arr: &[T]) -> Vec<f64> {
    arr.iter()
        .map(|x| (*x).into())
        .collect()
}
fn perform_operations(arr: Vec<f64>, duration: f64, res_arr: &mut IsoData) {
    let tick = duration / arr.len() as f64;
    res_arr.add_vals(&arr, tick)
}

pub fn get_iso_data(input: &TpInput) -> std::io::Result<(IsoData, IsoData)> {
    // let accurate_ts = input.has_accurate_timestamps();
    let mut final_iso_data = IsoData::new_with_capacity(10000);
    let mut final_expusure_data = IsoData::new_with_capacity(10000);

    if let Some(ref samples) = input.samples {
        for info in samples {
            if info.tag_map.is_none() { continue; }
            let duration = info.duration_ms;

            let grouped_tag_map = info.tag_map.as_ref().unwrap();



            for (group, map) in grouped_tag_map {
                if group == &GroupId::Custom("SensorISO".to_string()) {
                    if let Some(taginfo) = map.get(&TagId::Data) {
                        match &taginfo.value {
                            TagValue::Vec_u32(arr) => {
                                let arr = convert_array_to_f64(arr.get());
                                perform_operations(arr, duration, &mut final_iso_data);
                            },
                            TagValue::Vec_u16(arr) => {
                                let arr = convert_array_to_f64(arr.get());
                                perform_operations(arr, duration, &mut final_iso_data);
                            },
                            _ => { dbg!("SensorISO NOT Vec_u32!!!"); }
                        }
                    }
                }

                if group == &GroupId::Exposure {
                    if let Some(taginfo) = map.get(&TagId::Data) {
                        match &taginfo.value {
                            // Sony and GoPro
                            TagValue::Vec_f32(arr) => {
                                let arr = convert_array_to_f64(arr.get());
                                perform_operations(arr, duration, &mut final_expusure_data);
                            },
                           
                            _ => { dbg!("EXPOSURE NOT Vec_u32!!!"); }
                        }
                    }
                }
            }
        }
    }
    Ok((final_iso_data, final_expusure_data))
}


