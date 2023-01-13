use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, OutputStreamHandle};

pub struct Audio {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    pub is_playing: bool,
}



impl Audio {
    pub fn new() -> Audio {
        // Get a output stream handle to the default physical sound device
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        Self {
            stream: stream,
            stream_handle: stream_handle,
            is_playing: true,
        }
    }

    pub fn play(&mut self) {
        const CLAP: &[u8] = include_bytes!("clap.wav");

        self.is_playing = true;
        // Load a sound from a file, using a path relative to Cargo.toml
        // let file = BufReader::new(File::open("clap.wav").unwrap());
        
        let cursor = std::io::Cursor::new(CLAP);
        let file = BufReader::new(cursor);

        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        if let Err(e) = self.stream_handle.play_raw(source.convert_samples()) {
            println!("error: {:?}", e);
        }
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }
}
