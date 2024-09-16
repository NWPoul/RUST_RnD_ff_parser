


// use std::time::Instant;
use std::sync::{ Arc, atomic::AtomicBool };
// use std::ops::{Add, Div};

use telemetry_parser::Input as TpInput;
use telemetry_parser::util as tp_util;
use telemetry_parser::tags_impl::{
    GroupId,
    TagId,
    TagValue,
    GetWithType,
};

use crate::utils::u_serv::Vector3d;


pub struct IsoAndExpData {
    pub ts  : Vec<f64>,
    pub vals: Vec<f64>,
}
impl IsoAndExpData {
    pub fn new_with_capacity(capacity: usize) -> Self {
        IsoAndExpData{
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

pub struct IsoAndExpDataObj {
    pub iso: IsoAndExpData,
    pub exp: IsoAndExpData,
}


pub struct CameraInfo {
    pub model : String,
    pub serial: Option<String>,
}

pub struct TelemetryParsedData {
    pub file_name   : String,
    pub cam_info    : CameraInfo,
    pub acc_data    : Vec<Vector3d>,
    pub iso_exp_data: IsoAndExpDataObj,
}



pub const DEF_TICK: f64 = 0.005;




fn convert_array_to_f64<T: Into<f64> + Copy>(arr: &[T]) -> Vec<f64> {
    arr.iter()
        .map(|x| (*x).into())
        .collect()
}


#[allow(unused)]
fn dump_samples(samples: &[tp_util::SampleInfo]) {
    for info in samples {
        if info.tag_map.is_none() { continue; }
        let grouped_tag_map = info.tag_map.as_ref().unwrap();

        for (group, map) in grouped_tag_map {
            for (tagid, taginfo) in map {
                println!("{: <25} {: <20} {: <10}: {}{}", format!("{}", group), format!("{}", tagid), taginfo.description, &taginfo.value.to_string().len(), &taginfo.value.to_string().chars().take(50).collect::<String>());
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

fn get_iso_data(input: &TpInput) -> std::io::Result<IsoAndExpDataObj> {
    let mut iso_data = IsoAndExpData::new_with_capacity(10000);
    let mut exp_data = IsoAndExpData::new_with_capacity(10000);

    fn add_isoexp_vals<T: Into<f64> + Copy>(arr: &[T], duration: f64, res_arr: &mut IsoAndExpData) {
        let arr = convert_array_to_f64(&arr);
        let tick = duration / arr.len() as f64;
        res_arr.add_vals(&arr, tick / 1000.0)
    }

    if let Some(ref samples) = input.samples {
        for info in samples {
            if info.tag_map.is_none() { continue }
            let duration = info.duration_ms;
            let grouped_tag_map = info.tag_map.as_ref().unwrap();


            for (group, map) in grouped_tag_map {
                if group == &GroupId::Custom("SensorISO".to_string()) {
                    if let Some(taginfo) = map.get(&TagId::Data) {
                        match &taginfo.value {
                            TagValue::Vec_u32(arr) => add_isoexp_vals(arr.get(), duration, &mut iso_data),
                            TagValue::Vec_u16(arr) => add_isoexp_vals(arr.get(), duration, &mut iso_data),
                            _ => { dbg!("SensorISO NOT Vec_u32 or Vec_u16 !!!"); }
                        }
                    }
                }
                if group == &GroupId::Exposure {
                    if let Some(taginfo) = map.get(&TagId::Data) {
                        match &taginfo.value {
                            TagValue::Vec_f32(arr) => add_isoexp_vals(arr.get(), duration, &mut exp_data),
                            _ => { dbg!("EXPOSURE NOT Vec_u32 !!!"); }
                        }
                    }
                }
            }
        }
    }
    Ok(
        IsoAndExpDataObj {
            iso: iso_data,
            exp: exp_data,
        }
    )
}





fn add_vals(ts: &mut Vec<f64>, vals: &mut Vec<Vector3d>, new_vals: &[Vector3d], step: f64) {
        let start_ts = ts.last().unwrap_or(&0.0) + step;
        let new_ts: Vec<f64> = (0..new_vals.len()).map(|i| start_ts + (i as f64) * step).collect();
        ts.extend(new_ts);
        vals.extend_from_slice(new_vals);
    }

fn get_gravity_vector_data(input: &TpInput) -> std::io::Result<(Vec<f64>, Vec<Vector3d>)> {
    let mut ts:Vec<f64>                       = Vec::with_capacity(10000);
    let mut gravity_vector_data:Vec<Vector3d> = Vec::with_capacity(10000);

    if let Some(ref samples) = input.samples {
        for info in samples[..3].iter() {
            if info.tag_map.is_none() { continue }
            let duration = info.duration_ms;
            let grouped_tag_map = info.tag_map.as_ref().unwrap();


            for (group, map) in grouped_tag_map {
                
                if group == &GroupId::GravityVector {
                    dbg!(group, map);
                    if let Some(taginfo) = map.get(&TagId::Data) {
                        dbg!(taginfo);
                        // match &taginfo.value {
                        //     TagValue::Vec_u32(arr) => add_isoexp_vals(arr.get(), duration, &mut iso_data),
                        //     TagValue::Vec_u16(arr) => add_isoexp_vals(arr.get(), duration, &mut iso_data),
                        //     _ => { dbg!("SensorISO NOT Vec_u32 or Vec_u16 !!!"); }
                        // }
                    }
                }
            }
        }
    }
    Ok( (ts, gravity_vector_data) )
}






pub fn parse_telemetry_from_mp4_file(src_file: &str) -> Result<TelemetryParsedData, String> {
    let mut stream = match std::fs::File::open(src_file) {
        Ok(stream) => stream,
        Err(e) => {return Err(e.to_string());},
    };

    let filesize = match stream.metadata() {
        Ok(metadata) => metadata.len() as usize,
        Err(e) => {return Err(format!("NO_METADATA! {}", e.to_string()));},
    };

    let input = TpInput::from_stream(&mut stream, filesize, src_file, |_|(), Arc::new(AtomicBool::new(false))).unwrap();
 
    let cam_info = get_cam_info(&input);

    let iso_data = get_iso_data(&input);
    let gravity_vector_data = get_gravity_vector_data(&input);
    let samples = input.samples.clone().unwrap();
    // dump_samples(&samples[..3]);

    let mut acc_data : Vec<Vector3d> = Vec::new();

    let imu_data = match tp_util::normalized_imu_interpolated(&input, None) {
        Ok(data) => data,
        Err(e)   => {return Err(format!("FAIL TO GET IMUDATA! {}", e.to_string()));},
    };

    for v in imu_data {
        if v.accl.is_some() {
            let accl = v.accl.unwrap_or_default();
            acc_data.push(Vector3d::new(accl[0], accl[1], accl[2]));
        }
    }

    Ok(TelemetryParsedData {
        cam_info,
        file_name   : src_file.to_string(),
        acc_data,
        iso_exp_data: iso_data.expect("no iso/exposure data found!"),
    })
}

pub fn get_result_metadata_for_file(input_file: &str) -> Result<TelemetryParsedData, String> {
    let telemetry_data = parse_telemetry_from_mp4_file(input_file)?;
    Ok(TelemetryParsedData{
        file_name: input_file.to_string(),
        cam_info : telemetry_data.cam_info,
        acc_data : telemetry_data.acc_data,
        
        iso_exp_data : telemetry_data.iso_exp_data,
    })
}





