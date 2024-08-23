

use gnuplot::{
    Figure,
    Caption,
    Color,
};

use crate::analise::{parser_data_to_sma_list, parser_data_to_t_sma_xyz_list};

use crate::utils::u_serv::Vector3d;
use crate::PLOT_RAW;



pub fn format_data_for_plot(data: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let mut t: Vec<f64> = vec![0.0];
    let mut y = Vec::new();

    for (i, tdata) in data.iter().enumerate() {
        t.push(i as f64 * 0.005);
        y.push(*tdata);
    }
    (t, y)
}



// fn gnu_get_xyz_data(data: &Vec<(f64, f64, f64)>) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
//     let mut t: Vec<f64> = vec![0.0];
//     let mut xy: Vec<f64> = Vec::new();
//     let mut yy: Vec<f64> = Vec::new();
//     let mut zy: Vec<f64> = Vec::new();
//     for (i, tdata) in data.iter().enumerate() {
//         t.push(i as f64 * 0.005);
//         xy.push(tdata.0);
//         yy.push(tdata.1);
//         zy.push(tdata.2);
//     }
//    (t, xy, yy, zy)
// }
    // let (raw_t, raw_x, raw_y, raw_z) = gnu_get_xyz_data(data);
    // if PLOT_RAW { fg_2d
    //     .lines(raw_t, raw_x, &[Color("green"), Caption("raw_x"),])
    //     .lines(raw_t, raw_y, &[Color("red"), Caption("raw_y"),])
    //     .lines(raw_t, raw_z, &[Color("blue"), Caption("raw_z"),]);
    // }


pub fn gnu_plot_single(data: &[f64]) {
    let (t,y) = format_data_for_plot(data);

    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&t, &y, &[Color("black")]);

    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}




pub fn gnu_plot_series(data: &[Vector3d], base_series: &[usize]) {
    let mut sma_series   : Vec<(Vec<f64>,Vec<f64>, usize)> = Vec::new();
    let mut smaxyz_series: Vec<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, usize)> = Vec::new();

    for base in base_series {
        let cur_sma = parser_data_to_sma_list(data, *base);
        let cur_smaxyz = parser_data_to_t_sma_xyz_list(data, *base);
        sma_series.push((cur_sma.0, cur_sma.1, *base));
        smaxyz_series.push((cur_smaxyz.0, cur_smaxyz.1, cur_smaxyz.2, cur_smaxyz.3, *base));
    }

    
    let mut fg: Figure = Figure::new();
    let fg_2d = fg.axes2d();
    

    
    if PLOT_RAW {
        for smaxyz_data in smaxyz_series.iter() {
            let label = format!("{} pt", smaxyz_data.4);
            fg_2d
                .lines(&smaxyz_data.0, &smaxyz_data.1, &[Color("green"), Caption(&format!("raw_x {}",  &label))])
                .lines(&smaxyz_data.0, &smaxyz_data.2, &[Color("red")  , Caption(&format!("raw_y {}",  &label))])
                .lines(&smaxyz_data.0, &smaxyz_data.3, &[Color("blue") , Caption(&format!("raw_z {}",  &label))]);
        }
    }

    for sma_data in sma_series.iter() {
        let label = format!("{} pt", sma_data.2);
        fg_2d.lines(
            &sma_data.0,
            &sma_data.1,
            &[
                Caption(&label),
                Color("black"),
            ]
        );
    }

    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}