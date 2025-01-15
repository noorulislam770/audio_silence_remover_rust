# Program Flow

The program flow is as follows:

1. The user runs the program with input and output file names as arguments
2. The program opens the input WAV file and reads its specifications
3. The program creates an output WAV file with the same specifications
4. The program processes the audio in frames
5. For each frame, the program detects if the frame contains voice or not
6. If the frame contains voice, the program writes the frame to the output WAV file
7. The program closes the input and output WAV files

## Components

The program consists of the following components:

### `main.rs`

* The main function is the entry point of the program
* The main function parses the command line arguments and calls the `process_audio` function

### `process_audio`

* The `process_audio` function takes an input and output file name as arguments
* The function opens the input WAV file and reads its specifications
* The function creates an output WAV file with the same specifications
* The function processes the audio in frames
* The function calls the `is_voice` function to detect if the frame contains voice or not
* The function writes the frame to the output WAV file if the frame contains voice
* The function closes the input and output WAV files

### `VoiceActivityDetector`

* The `VoiceActivityDetector` struct holds the state of the voice activity detection algorithm
* The `VoiceActivityDetector` struct has a method called `is_voice` which takes a frame as an argument
* The `is_voice` method detects if the frame contains voice or not and returns a boolean value

### `normalize_sample`

* The `normalize_sample` function takes an i32 sample as an argument and returns a normalized f32 value
* The function is used to normalize the audio samples to a range of -1.0 to 1.0
