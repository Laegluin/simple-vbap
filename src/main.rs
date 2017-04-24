extern crate hound;


fn main()
{
    let from = "D:\\Eigene Assets\\Cargo Projects\\simple_vbap\\samples\\DryGuitar_Mono.wav";
    let to = "D:\\Eigene Assets\\Cargo Projects\\simple_vbap\\samples\\result.wav";

    mono_to_scaled_stereo(from, to, 1.0, 1.0);
}

fn mono_to_scaled_stereo(source: &str, destination: &str, left_gain: f64, right_gain: f64)
{
    let specs = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(source, specs).unwrap();
    let mut reader = hound::WavReader::open(destination).unwrap();

    for result in reader.samples::<i16>()
    {
        let sample = result.unwrap();
        let left = sample as f64 * left_gain;
        let right = sample as f64 * right_gain;

        writer.write_sample(left as i16).unwrap();
        writer.write_sample(right as i16).unwrap();
    }

    writer.finalize().unwrap();
}
