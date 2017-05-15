#![allow(dead_code)]

extern crate hound;
extern crate libc;
mod vbap_converter;
mod player;

use vbap_converter::*;
use player::*;
use std::path::Path;
use std::io::{stdin, stdout, Write, BufRead};


fn main()
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 2
    {
        print_usage();
        return;
    }

    let mut from = String::new();
    let mut to = String::new();
    let mut play_after_finish = false;
    let mut custom_panning = false;
    let mut pan_angle = 0.0;

    for (index, arg) in args.into_iter().enumerate()
    {
        match (index, arg.to_lowercase().trim())
        {
            (1, path) => from = path.to_owned(),
            (2, path) => to = path.to_owned(),
            (3, angle) => pan_angle = angle.parse::<f64>().unwrap_or_default(),
            (_, "-p") | (_, "--play") => play_after_finish = true,
            (_, "-m") | (_, "--move") => custom_panning = true,
            (_, "-h") | (_, "--help") | _ => print_usage(),
        }
    }

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

    let converter = VbapConverter::new(&from).unwrap();

    if custom_panning
    {
        converter.pan_interactive(&to, pan_moving);
    }
    else 
    {
        converter.pan(&to, 30.0, pan_angle);   
    }

    if play_after_finish
    {
        play(&to);
    }      
}

fn play(path: &str)
{
    // init player and start playing
    let vlc = LibVlc::new().unwrap();
    let mut player = MediaPlayer::new(&vlc).unwrap();
    let media = Media::new(&vlc, path).unwrap();

    player.set_media(media);
    player.play();

    println!("Playing converted media! Type '!pause' or '!play' to control the player.");

    loop 
    {
        // write line prefix
        let mut stdout = stdout();
        stdout.lock();
        stdout.write("<Player> ".as_bytes()).unwrap();
        stdout.flush().unwrap();
        
        // listen to input
        let mut buffer = String::new();
        let stdin = stdin();
        stdin.read_line(&mut buffer).unwrap();

        match buffer.to_lowercase().trim()
        {
            "!p" | "!play" => player.play(),
            "!pause" => player.pause(),
            "!q" | "!quit" => break,
            _ => print_usage_player(),
        }
    }
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

fn print_usage_player()
{
    let usage1 = "<Player> [!p | !play]     Starts playback. No effect if already playing.\n";
    let usage2 = "<Player> [!pause]         Pauses playback. No effect if already paused.\n";
    let usage3 = "<Player> [!q | !quit]     Stops playback and exits the program.\n";

    println!("{0}{1}{2}", usage1, usage2, usage3);
}

fn print_usage()
{
    println!("Usage: simple_vbap <source> <destination> <angle> [-p | -m]\n");
    println!("<source>          The path of the file that will be converted. Must be in WAV format.");
    println!("<destination>     The path that the converted file will be written to. Output is in WAV.");
    println!("<angle>           The angle the audio will be panned to. Must be between 30 and -30.");
    println!("[-p | --play]     Plays the converted file.");
    println!("[-m | --move]     Uses a default panning function that alters the direction over time.");
}
