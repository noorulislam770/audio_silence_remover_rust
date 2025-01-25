use clap::Parser;
use hound::{WavReader, WavSpec, WavWriter};
use std::error::Error;
use std::path::PathBuf;
// use std::time::Instant;
use rayon;

#[derive(Debug, Clone)]
pub struct VadConfig {
    pub threshold: f32,
    pub silence_frame_limit: usize,
    pub frame_size: usize,
    pub step_size: usize,
}

impl Default for VadConfig {
    fn default() -> Self {
        Self {
            threshold: 0.1,
            silence_frame_limit: 20,
            frame_size: 160,
            step_size: 160,
        }
    }
}

#[derive(Debug)]
pub struct VoiceActivityDetector {
    config: VadConfig,
    silence_counter: usize,
    vad_threshold: f32,
    vad_n: f32,
}

impl VoiceActivityDetector {
    pub fn new(config: VadConfig) -> Self {
        Self {
            config,
            silence_counter: 0,
            vad_threshold: 0.0,
            vad_n: 0.0,
        }
    }

    fn is_voice(&mut self, frame: &[f32]) -> bool {
        // Pre-allocate squared frame buffer
        let mut frame_squared = Vec::with_capacity(frame.len());
        let mut min_val = f32::INFINITY;
        let mut max_val = f32::NEG_INFINITY;
        
        // Single pass through frame for squaring and min/max
        for &sample in frame {
            let squared = sample * sample;
            frame_squared.push(squared);
            min_val = min_val.min(squared);
            max_val = max_val.max(squared);
        }
        
        let thd = min_val + (max_val - min_val) * self.config.threshold;
        
        // Update VAD threshold using running average
        self.vad_threshold = (self.vad_n * self.vad_threshold + thd) / (self.vad_n + 1.0);
        self.vad_n += 1.0;
        
        // Calculate mean energy
        let mean_energy = frame_squared.iter().sum::<f32>() / frame_squared.len() as f32;
        
        if mean_energy <= self.vad_threshold {
            self.silence_counter += 1;
        } else {
            self.silence_counter = 0;
        }
        
        self.silence_counter <= self.config.silence_frame_limit
    }

    // Process samples and return indices of voice frames
    pub fn process_channel(&mut self, samples: &[f32]) -> Vec<usize> {
        let estimated_capacity = samples.len() / self.config.step_size;
        let mut voice_frames = Vec::with_capacity(estimated_capacity);
        let mut i = 0;
        
        while i + self.config.frame_size <= samples.len() {
            if self.is_voice(&samples[i..i + self.config.frame_size]) {
                voice_frames.push(i);
            }
            i += self.config.step_size;
        }
        
        voice_frames
    }
}

pub struct AudioProcessor {
    config: VadConfig,
}

impl AudioProcessor {
    pub fn new(config: VadConfig) -> Self {
        Self { config }
    }

    pub fn process_file(&mut self, input_path: &PathBuf, output_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        
        // let start = Instant::now();
        // println!("Processing audio file: {:?}", input_path);
        let mut reader = WavReader::open(input_path)?;
        let spec = reader.spec();

        let output_spec = WavSpec {
            channels: 1,
            sample_rate: spec.sample_rate,  // Keep original sample rate
            bits_per_sample: 16,                  // Set to 16 bits for 256K bitrate
            sample_format: hound::SampleFormat::Int,
        };

        if spec.channels != 2 {
            return Err("Only stereo WAV files are supported".into());
        }
        // Read all samples at once
        let samples: Vec<f32> = reader
            .samples::<i32>()
            .map(|s| s.map(normalize_sample))
            .collect::<Result<Vec<f32>, _>>()?;

        // Split into channels without creating new vectors
        let (left_channel, right_channel): (Vec<_>, Vec<_>) = samples
            .chunks(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .unzip();

        // Process each channel
        let mut vad_left = VoiceActivityDetector::new(self.config.clone());
        let mut vad_right = VoiceActivityDetector::new(self.config.clone());

        // Get frame indices instead of copying data
        let (left_frames, right_frames) = rayon::join(
        || vad_left.process_channel(&left_channel),
        || vad_right.process_channel(&right_channel)
    );
        // Choose the channel with more voice frames
        let selected_frames = if left_frames.len() >= right_frames.len() {
            (&left_channel, left_frames)
        } else {
            (&right_channel, right_frames)
        };

        // Create output file with pre-calculated size
        let mut writer = WavWriter::create(output_path.with_extension(input_path.extension().unwrap()), output_spec)?;
        
        // Write frames directly without intermediate buffer
        for &frame_start in &selected_frames.1 {
            let frame_end = frame_start + self.config.frame_size;
            for &sample in &selected_frames.0[frame_start..frame_end] {
                // Write sample to both channels
                writer.write_sample(denormalize_sample(sample))?;
            }
        }

        writer.finalize()?;
        // println!("Processing completed in {:?}", start.elapsed());
        Ok(())
    }
}

#[inline]
fn normalize_sample(sample: i32) -> f32 {
    sample as f32 / i32::MAX as f32
}

#[inline]
fn denormalize_sample(sample: f32) -> i32 {
    (sample * i32::MAX as f32) as i32
}

fn main() {
    #[derive(Parser)]
    #[command(name = "audio_silence_remover")]
    #[command(about = "Removes silence from stereo WAV files using VAD algorithm", long_about = None)]
    struct Args {
        #[arg(help = "Input WAV file")]
        input: PathBuf,

        #[arg(help = "Output WAV file")]
        output: PathBuf,
    }

    let args = Args::parse();
    let config = VadConfig::default();
    let mut processor = AudioProcessor::new(config);

    if let Err(e) = processor.process_file(&args.input, &args.output) {
        eprintln!("Error processing audio: {}", e);
        std::process::exit(1);
    }
}