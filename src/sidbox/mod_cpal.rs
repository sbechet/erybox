use cpal::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::{TAU};

pub struct Audio {
    device: Device,
    supported_config: SupportedStreamConfig,
    config: StreamConfig,
    stream: Stream,
    pub is_playing: bool,
}

impl Audio {
    pub fn new() -> Audio {
        let host= cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        let supported_config = device.default_output_config().unwrap();
        let config = supported_config.config();
        println!("host: {}", host.id().name());
        println!("Device: {}, Using config: {:?}", device.name().expect("no device name?"), config);

        let stream = match supported_config.sample_format() {
            cpal::SampleFormat::F32 => {
                Self::write_sample::<f32>(&device, &config).unwrap()
            },
            cpal::SampleFormat::I16 => {
                Self::write_sample::<i16>(&device, &config).unwrap()
            },
            cpal::SampleFormat::U16 => {
                Self::write_sample::<u16>(&device, &config).unwrap()
            },
        };

        Self {
            device: device,
            supported_config: supported_config,
            config: config,
            stream: stream,
            is_playing: true,
        }
    }

    fn write_sample<T: Sample>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<Stream, String> {
        let channels = config.channels as usize;
        let sample_rate = config.sample_rate.0 as f32;
        // println!("channel:{}, sample_rate:{}", channels, sample_rate);

        let mut phi = 0.0f32;
        let frequency = 440.0;
        let amplitude = 0.05;

        let stream = device.build_output_stream(&config, 
                                                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                                                    // let mut counter = 0;
                                                    // for sample in data.iter_mut() {
                                                    //     let s = if (counter / 20) % 16 == 0 { &0.2 } else { &-0.2 };
                                                    //     counter = counter + 1;
                                                    //     *sample = Sample::from(s);
                                                    // }

                                                    for frame in data.chunks_mut(channels) {
                                                        phi += frequency / sample_rate;
                                                        println!("f={}, sr={} => phi:{}",frequency, sample_rate, phi);
                                                        let make_noise = || -> f32 {amplitude * (TAU * phi).sin()};
                                                        let value: T = cpal::Sample::from::<f32>(&make_noise());

                                                        for sample in frame.iter_mut() {
                                                            *sample = value;
                                                        }
                                                    }


                                                }, 
                                                move |_err| {}).unwrap();
        Ok(stream)
    }

    pub fn play(&mut self) {
        self.is_playing = true;
        self.stream.play().unwrap();
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
        self.stream.pause().unwrap();
    }
}
