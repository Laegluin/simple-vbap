use std::f64::consts::PI;
use std::result::Result;
use hound;


type PanningCallback = fn(i32) -> PanningDirection;

#[derive(Debug)]
pub struct VbapConverter
{
    source: String,
    specs: hound::WavSpec,
}

#[derive(Debug)]
pub struct PanningDirection
{
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

    pub fn pan(&self, destination: &str, direction: PanningDirection)
    {
        let gain = VbapConverter::calculate_gain(direction.base_angle, direction.pan_angle);

        match self.specs.channels
        {
            1 => self.apply_gain_for_mono(gain, destination),
            2 => self.apply_gain_for_stereo(gain, destination),
            _ => (),
        }
    }

    fn apply_gain_for_mono(&self, gain: Gain, destination: &str)
    {
        let specs = hound::WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut reader = hound::WavReader::open(&self.source).unwrap();
        let mut writer = hound::WavWriter::create(destination, specs).unwrap();

        for result in reader.samples::<i16>()
        {
            let sample = result.unwrap();

            let left = sample as f64 * gain.left;
            let right = sample as f64 * gain.right;

            writer.write_sample(left as i16).unwrap();
            writer.write_sample(right as i16).unwrap();
        }

        writer.finalize().unwrap();
    }

    fn apply_gain_for_stereo(&self, gain: Gain, destination: &str)
    {
        let specs = hound::WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut reader = hound::WavReader::open(&self.source).unwrap();
        let mut writer = hound::WavWriter::create(destination, specs).unwrap();

        for (index, result) in reader.samples::<i16>().enumerate()
        {
            let sample = result.unwrap();

            let channel_gain: f64;

            if index % 2 == 0
            {
                channel_gain = gain.left;
            }
            else
            {
                channel_gain = gain.right;
            }

            let new_sample = sample as f64 * channel_gain;

            writer.write_sample(new_sample as i16).unwrap();
        }

        writer.finalize().unwrap();
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
