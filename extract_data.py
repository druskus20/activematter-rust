import os
import pandas as pd

def extract_times(filename):
    """Extracts time values from a given file and returns the average time."""
    times = []
    with open(filename, 'r') as file:
        for line in file:
            if line.startswith('Time:'):
                time_value = float(line.split(':')[1].strip().split()[0])
                times.append(time_value)
    if times:
        average_time = sum(times) / len(times)
        return average_time
    else:
        return None

# Directory containing the benchmark files
directory = './'

# Initialize a list to store the data
data = []

# Loop through each file in the directory
for filename in os.listdir(directory):
    if filename.endswith('.txt'):
        file_path = os.path.join(directory, filename)
        avg_time = extract_times(file_path)
        if avg_time is not None:
            data.append({'Filename': filename, 'AverageTime': avg_time})

# Create a DataFrame
df = pd.DataFrame(data)

# Save the DataFrame to a CSV file for easier manipulation and sharing
df.to_csv('benchmark_results.csv', index=False)

# Display the DataFrame
print(df)

