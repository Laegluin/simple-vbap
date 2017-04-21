extern crate hound;


fn main()
{
    let specs = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("D:\\Eigene Assets\\Cargo Projects\\simple_vbap\\samples\\result.wav", specs).unwrap();
    let mut reader = hound::WavReader::open("D:\\Eigene Assets\\Cargo Projects\\simple_vbap\\samples\\DryGuitar_Mono.wav").unwrap();

    for result in reader.samples::<i16>()
    {
        let sample = result.unwrap();
        let computed_sample = sample as f64 * 0.5;

        writer.write_sample(computed_sample as i16).unwrap();
        writer.write_sample(computed_sample as i16).unwrap();
    }

    writer.finalize().unwrap();
}
