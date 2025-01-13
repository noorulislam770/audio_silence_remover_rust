# Audio Silence Remover

A command-line tool for removing silence from stereo WAV files.

## Overview

This tool uses a voice activity detection (VAD) algorithm to identify and remove silent segments from audio files. It takes an input WAV file, processes it to remove silence, and outputs the resulting audio to a new WAV file.

## Usage

To use this tool, simply run the following command:

```
audio_silence_remover --input <input_file.wav> --output <output_file.wav>
```

Replace `<input_file.wav>` with the path to your input WAV file, and `<output_file.wav>` with the desired path for the output WAV file.

## Options

* `--input`: Specify the input WAV file path.
* `--output`: Specify the output WAV file path.

## Requirements

* Rust 1.45 or later
* Clap 2.33 or later
* Hound 0.8 or later

## Building and Installing

To build and install the tool, run the following commands:
cargo build cargo install

This will install the tool to your system's PATH.

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! If you'd like to contribute to this project, please fork the repository and submit a pull request.

## Acknowledgments

This project uses the following libraries:

* Clap: A command-line argument parser for Rust.
* Hound: A WAV file reader and writer for Rust.

