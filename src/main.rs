#![allow(dead_code)]

extern crate hound;
mod vbap_converter;

use vbap_converter::*;


fn main()
{
    let from = "D:\\Eigene Assets\\Cargo Projects\\simple_vbap\\samples\\DryGuitar_Mono.wav";
    let to = "D:\\Eigene Assets\\Cargo Projects\\simple_vbap\\samples\\result.wav";

    let converter = VbapConverter::new(from);
    converter.pan(to, 30.0, 25.0);
}
