import scipy.io.wavfile as wavfile
import matplotlib.pyplot as plt
import numpy as np
import sys
import os

def plot_wav_files(file_paths):
    """
    Plot waveforms for multiple WAV files, each with its own time axis.
    
    Args:
        file_paths (list): List of paths to WAV files
    """
    if len(file_paths) == 0:
        print("Error: No input files provided")
        print("Usage: python silence_removal.py file1.wav file2.wav ...")
        sys.exit(1)

    # Create subplots based on number of input files
    plt.figure(figsize=(12, 4 * len(file_paths)))
    
    for i, file_path in enumerate(file_paths):
        if not os.path.exists(file_path):
            print(f"Error: File '{file_path}' does not exist")
            sys.exit(1)
        
        if not file_path.lower().endswith('.wav'):
            print(f"Error: File '{file_path}' is not a WAV file")
            sys.exit(1)
            
        try:
            # Read and process each file
            sr, y = wavfile.read(file_path)
            
            # Normalize the waveform
            y = y / np.max(np.abs(y))
            
            # Calculate duration and create time axis for this specific file
            duration = len(y) / sr
            time_axis = np.linspace(0, duration, num=len(y))
            
            # Create subplot for this file
            plt.subplot(len(file_paths), 1, i + 1)
            plt.plot(time_axis, y)
            plt.title(f'Waveform of {os.path.basename(file_path)} (Duration: {duration:.2f}s)')
            plt.xlabel('Time (seconds)')
            plt.ylabel('Amplitude')
            plt.ylim([-1, 1])
            
        except Exception as e:
            print(f"Error reading file '{file_path}': {str(e)}")
            sys.exit(1)
    
    plt.tight_layout()
    plt.show()

if __name__ == "__main__":
    # Remove the script name from sys.argv
    input_files = sys.argv[1:]
    plot_wav_files(input_files)