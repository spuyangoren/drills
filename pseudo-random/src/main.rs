extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::{self, Write};
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let n = 100;    
   

    let mut rng = rand::thread_rng();              //initialize rng
    let data: Vec<f64> = (0..n).map(|_| rng.gen::<f64>()).collect();        //generate float vector "data" with elements generated from random numbers  

    let mut file = File::create("rand.txt").expect("Failed to create file");    //create file
    
    for i in &data {
        writeln!(&mut file, "{}", i).expect("Failed to write file");     //write every element of data into file
    }

    //


    let root = BitMapBackend::new("random_numbers.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

     let mut chart = ChartBuilder::on(&root)
        .caption("Random Numbers", ("sans-serif", 40))
        .x_label_area_size(40)
        .y_label_area_size(40)
        //.build_ranged(0.0..1.0, 0.0..1.0)?;
        .build_ranged(0.0..100.0, 0.0..1.0)?;

    chart
        .configure_mesh()
        .x_desc("Index")
        .y_desc("Number")
        .axis_desc_style(("sans-serif", 20))
        .x_label_offset(5)
        .y_label_offset(5)
        .draw()?;

    chart
        .draw_series(data.iter().enumerate().map(|(j, val)| {
          //  return Circle::new((j as f64 / (data.len() - 1) as f64, *val as f64), 2, RED.filled());
          return Circle::new((j as f64, *val as f64), 2, RED.filled());
            //return Circle::new((j as i32 / (data.len() - 1) as i32, *val as i32), 2, RED.filled());
        }))?;

    Ok(())


   /* for _i in 0..100 {
        //let mut rng = rand::thread_rng();

        let random_number = rng.gen_range(1..100);
        let num_string = random_number.to_string();

        println!("{}", random_number);

        file.write_all(num_string.as_bytes()).expect("Failed to write file");

        file.write_all(b"\n").expect("Failed to write file");

        //data[i] = random_number;
    }
    
    //println!("saved");


   // println!("{}", random_number); */


}



