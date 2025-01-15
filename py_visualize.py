import scipy.io.wavfile as wavfile
import matplotlib.pyplot as plt
import numpy as np

# Load the three audio files
file1 = '2.wav'
file2 = '1_sample_PYTHON_CONVERTED.wav.wav'
file3 = '3.wav'

# Read the audio files
sr1, y1 = wavfile.read(file1)
sr2, y2 = wavfile.read(file2)
sr3, y3 = wavfile.read(file3)

# Normalize the waveforms (optional, for better visualization)
y1 = y1 / np.max(np.abs(y1))
y2 = y2 / np.max(np.abs(y2))
y3 = y3 / np.max(np.abs(y3))

# Determine the same time axis range (use the shortest duration)
duration = min(len(y1) / sr1, len(y2) / sr2, len(y3) / sr3)
time_axis = np.linspace(0, duration, num=int(duration * sr1))  # Assuming all files have similar sample rates

# Resize the waveforms to match the same duration
y1 = y1[:len(time_axis)]
y2 = y2[:len(time_axis)]
y3 = y3[:len(time_axis)]

# Plot the waveforms
plt.figure(figsize=(12, 9))  # Adjust the figure size for 3 plots

plt.subplot(3, 1, 1)
plt.plot(time_axis, y1)
plt.title('Waveform of ' + file1)
plt.xlabel('Time (seconds)')
plt.ylabel('Amplitude')
plt.ylim([-1, 1])  # Set consistent y-axis limits

plt.subplot(3, 1, 2)
plt.plot(time_axis, y2)
plt.title('Waveform of ' + file2)
plt.xlabel('Time (seconds)')
plt.ylabel('Amplitude')
plt.ylim([-1, 1])  # Set consistent y-axis limits

plt.subplot(3, 1, 3)
plt.plot(time_axis, y3)
plt.title('Waveform of ' + file3)
plt.xlabel('Time (seconds)')
plt.ylabel('Amplitude')
plt.ylim([-1, 1])  # Set consistent y-axis limits

plt.tight_layout()
plt.show()
