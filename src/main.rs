#![allow(dead_code)]

extern crate hound;
extern crate libc;
mod vbap_converter;
mod player;

use vbap_converter::*;
use player::*;
use std::path::Path;
use std::io::{stdin, BufRead};


fn main()
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 2
    {
        print_usage();
        return;
    }

    let from = args[1].to_owned();
    let to = args[2].to_owned();

    if !Path::new(&from).exists()
    {
        println!("\"{0}\" is not a valid path", from);
        return;
    }

    if Path::new(&to).exists()
    {
        println!("\"{0}\" already exists. Override? (y/n)", to);

        let mut buffer = String::new();
        let stdin = stdin();
        stdin.lock().read_line(&mut buffer).unwrap();

        match buffer.to_lowercase().trim() 
        {
            "y" => std::fs::remove_file(&to).unwrap(),
            _ => return,
        }
    }

    let mut play_after_finish = false;
    let mut custom_panning = false;

    // set flags
    if args.len() > 3
    {
        for i in 3 .. args.len()
        {
            match args[i].to_lowercase().as_str()
            {
                "-p" | "--play" => play_after_finish = true,
                "-m" | "--move" => custom_panning = true,
                "-h" | "--help" | _ => print_usage(),
            }
        }
    }

    let converter = VbapConverter::new(&from).unwrap();

    if custom_panning
    {
        converter.pan_interactive(&to, pan_moving);
    }
    else 
    {
        converter.pan(&to, 30.0, 25.0);   
    }

    if play_after_finish
    {
        play();
    }      
}

fn play()
{

}

#[allow(unused_variables, unused_assignments)]
fn pan_moving(sample_index: u32, user_data: Option<()>) -> PanningDirection<()>
{
    let base_angle = 30.0;
    let mut pan_angle = 0.0;

    let periodic_index = sample_index % 80000;

    match periodic_index 
    {
        0...40000 => 
        {
            pan_angle = 25.0 - 25.0 * (periodic_index as f64 / 40000.0);
        },
        _ => 
        {
            pan_angle = -25.0 * ((periodic_index as f64 - 40000.0) / 40000.0);
        },
    }

    PanningDirection 
    {
        user_data: Option::None,
        base_angle: base_angle,
        pan_angle: pan_angle,
    }
}

fn print_usage()
{
    println!("Wrong usage.");
}
