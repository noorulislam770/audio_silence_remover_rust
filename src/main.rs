use clap::Parser;
use hound::{WavReader, WavWriter};
use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "audio_silence_remover")]
#[command(about = "Removes silence from stereo WAV files")]
struct Args {
    /// Input WAV file path
    #[arg(help = "Input WAV file")]
    input: PathBuf,

    /// Output WAV file path
    #[arg(help = "Output WAV file")]
    output: PathBuf,
}

struct VoiceActivityDetector {
    threshold: f32,
    silence_counter: usize,
    vad_threshold: f32,
    vad_n: f32,
}

impl VoiceActivityDetector {
    fn new() -> Self {
        Self {
            threshold: 0.1,  // Using the same threshold value as in the Python code
            silence_counter: 0,
            vad_threshold: 0.0,
            vad_n: 0.0,
        }
    }

    fn is_voice(&mut self, frame: &[f32]) -> bool {
        let frame_energy: f32 = frame.iter().map(|&x| x * x).sum();
        let mean_energy = frame_energy / frame.len() as f32;

        // Calculate threshold based on the same logic as the Python code
        let min_energy: f32 = frame.iter().fold(f32::INFINITY, |a, &b| a.min(b.powi(2)));
        let max_energy: f32 = frame.iter().fold(0.0_f32, |a, &b| a.max(b.powi(2)));
        let threshold = min_energy + (max_energy - min_energy) * self.threshold;

        // Update vad_threshold adaptively
        self.vad_threshold = (self.vad_n * self.vad_threshold + threshold) / (self.vad_n + 1.0);
        self.vad_n += 1.0;

        // Compare mean_energy with vad_threshold to detect silence or voice
        if mean_energy <= self.vad_threshold {
            self.silence_counter += 1;
        } else {
            self.silence_counter = 0;
        }

        self.silence_counter <= 20  // Same condition as in the Python code
    }
}


fn normalize_sample(sample: i32) -> f32 {
    sample as f32 / i32::MAX as f32

}


fn process_audio(input_path: &PathBuf, output_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    println!("Processing audio file: {:?}", input_path);

    // Open the input WAV file
    let mut reader = WavReader::open(input_path)?;
    let spec = reader.spec();

    println!("Input audio specs: {:?}", spec);

    // Create output WAV file with the same specifications
    let mut writer = WavWriter::create(output_path, spec)?;

    // Process audio in frames
    let mut vad = VoiceActivityDetector::new();
    let frame_size = 160; // About 10ms at 16kHz
    let mut frame_buffer: Vec<f32> = Vec::with_capacity(frame_size * spec.channels as usize);
    let mut output_samples = Vec::new();

    // Handle mono and stereo cases
    if spec.channels == 1 {
        // Mono audio processing
        let samples: Vec<f32> = reader
            .samples::<i32>()
            .map(|s| match s {
                Ok(sample) => Ok(normalize_sample(sample)),
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<f32>, _>>()?;

        // Process frames
        for chunk in samples.chunks(frame_size) {
            frame_buffer.clear();
            frame_buffer.extend_from_slice(chunk);

            // Check if frame contains voice
            if vad.is_voice(&frame_buffer) {
                output_samples.extend_from_slice(chunk);
            }
        }

    } else if spec.channels == 2 {
        // Stereo audio processing
        let samples: Vec<f32> = reader
            .samples::<i32>()
            .map(|s| match s {
                Ok(sample) => Ok(normalize_sample(sample)),
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<f32>, _>>()?;

        // Process frames
        for chunk in samples.chunks(frame_size * 2) {
            frame_buffer.clear();
            frame_buffer.extend_from_slice(chunk);

            // Check if frame contains voice
            if vad.is_voice(&frame_buffer) {
                output_samples.extend_from_slice(chunk);
            }
        }
    } else {
        return Err("Only mono and stereo WAV files are supported".into());
    }

    // Convert back to the original sample format and write
    for sample in output_samples {
        let int_sample = (sample * i32::MAX as f32) as i32;
        writer.write_sample(int_sample)?;
    }

    writer.finalize()?;
    println!("Processing completed in {:?}", start.elapsed());

    Ok(())
}



fn main() {
    let args = Args::parse();

    if let Err(e) = process_audio(&args.input, &args.output) {
        eprintln!("Error processing audio: {}", e);
        std::process::exit(1);
    }
}