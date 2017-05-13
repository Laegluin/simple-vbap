use std::f64::consts::PI;
use std::result::Result;
use std::io::{Write, Seek};
use hound;


#[derive(Debug)]
enum ChannelGain
{
    Left(f64),
    Right(f64),
}

#[derive(Debug)]
pub struct VbapConverter
{
    source: String,
    specs: hound::WavSpec,
}

#[derive(Debug)]
pub struct PanningDirection<T>
{
    pub user_data: Option<T>,
    pub base_angle: f64,
    pub pan_angle: f64,
}

#[derive(Debug)]
struct Gain
{
    left: f64,
    right: f64,
}


impl VbapConverter
{
    pub fn new(source: &str) -> Result<VbapConverter, &str>
    {
        let reader = hound::WavReader::open(source).unwrap();

        if reader.spec().channels > 2 || reader.spec().channels < 1
        {
            return Result::Err("Only mono or stereo files are supported.");
        }

        Result::Ok(VbapConverter {
                       source: source.to_owned(),
                       specs: reader.spec(),
                   })
    }

    pub fn pan(&self, destination: &str, base_angle: f64, pan_angle: f64)
    {      
        #![allow(unused_variables)]  
        let const_pan = |index: u32, user_data: Option<()>| PanningDirection
        {
            user_data: Option::None,
            base_angle: base_angle,
            pan_angle: pan_angle,
        };

        self.write_samples(destination, const_pan);
    }

    pub fn pan_interactive<T, F>(&self, destination: &str, callback: F)
        where F: Fn(u32, Option<T>) -> PanningDirection<T>
    {
        self.write_samples(destination, callback);
    }

    fn write_samples<T, F>(&self, destination: &str, callback: F) 
        where F: Fn(u32, Option<T>) -> PanningDirection<T>
    {
        // write output as stereo pcm
        let specs = hound::WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut reader = hound::WavReader::open(&self.source).unwrap();
        let mut writer = hound::WavWriter::create(destination, specs).unwrap();

        // first call has no user data to pass
        let mut user_data: Option<T> = Option::None;

        // iterate over samples
        for (index, result) in reader.samples::<i16>().enumerate()
        {
            let sample = result.unwrap();
            let sample_pair_index = index / self.specs.channels as usize;
            let mut gain: Gain = Gain {left: 0.0, right: 0.0};

            if index % self.specs.channels as usize == 0
            {
                let direction = callback(sample_pair_index as u32, user_data);
                gain = VbapConverter::calculate_gain(direction.base_angle, direction.pan_angle);
                
                user_data = direction.user_data;
            }

            // write samples depending on input format
            if self.specs.channels == 1
            {
                VbapConverter::write_samples_mono(sample, gain, &mut writer);
            }
            else if self.specs.channels == 2
            {
                // calculate channel of current sample
                if index % 2 == 0
                {
                    VbapConverter::write_samples_stereo(sample, ChannelGain::Left(gain.left), &mut writer);
                }
                else
                {
                    VbapConverter::write_samples_stereo(sample, ChannelGain::Right(gain.right), &mut writer);
                }
            }
        }

        // finalize the written data
        writer.finalize().unwrap();
    }

    fn write_samples_mono<W>(sample: i16, gain: Gain, writer: &mut hound::WavWriter<W>)
        where W: Write + Seek
    {
            let left = sample as f64 * gain.left;
            let right = sample as f64 * gain.right;

            writer.write_sample(left as i16).unwrap();
            writer.write_sample(right as i16).unwrap();
    }

    fn write_samples_stereo<W>(sample: i16, channel_gain: ChannelGain, writer: &mut hound::WavWriter<W>)
        where W: Write + Seek
    {
        let new_sample: i16;

        match channel_gain 
        {
            ChannelGain::Left(gain) => new_sample = (sample as f64 * gain) as i16,
            ChannelGain::Right(gain) => new_sample = (sample as f64 * gain) as i16,
        }

        writer.write_sample(new_sample).unwrap();
    }

    fn calculate_gain(base_angle: f64, pan_angle: f64) -> Gain
    {
        if pan_angle == 0.0
        {
            return Gain { left: 1.0, right: 1.0 };
        }

        if pan_angle >= base_angle || pan_angle <= -base_angle
        {
            panic!("The pan angle must be between base_angle and -base_angle.");
        }

        let base_angle_rad = base_angle * PI / 180.0;
        let pan_angle_rad = pan_angle * PI / 180.0;

        let mut right = (base_angle_rad.tan() - pan_angle_rad.tan()).powi(2);
        right /= 2.0 * (base_angle_rad.tan().powi(2)) + 2.0 * (pan_angle_rad.tan().powi(2));
        right = right.sqrt();

        let mut left = right * (base_angle_rad.tan()) + right * (pan_angle_rad.tan());
        left /= base_angle_rad.tan() - pan_angle_rad.tan();

        Gain { left: left, right: right }
    }
}
