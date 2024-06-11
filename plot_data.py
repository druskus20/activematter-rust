import pandas as pd
import matplotlib.pyplot as plt

# Read the data from the CSV file
df = pd.read_csv('benchmark_results.csv')

print(df)
# filter df and only leave stuff that ends in 2048.txt birds
df = df[df['Filename'].str.contains('-64')]

df = df[df['Filename'].str.contains('omp') | df['Filename'].str.contains('basic') | df['Filename'].str.contains('dumb')]

#df = df[~df['Filename'].str.contains('c_omp')]
#df = df[~df['Filename'].str.contains('rust_rayon_ndarray')]
#df = df[~df['Filename'].str.contains('ndarray')]

# Assuming the dataframe 'df' is already loaded
# Extract method and thread count from 'Filename'
df['Method'] = df['Filename'].str.extract(r'([a-z_]+)-1')[0]
df['Threads'] = df['Filename'].str.extract(r'-\d+-(\d+)-')[0].astype(int)
df['Birds'] = df['Filename'].str.extract(r'(\d+)\.txt$').astype(int)

#df['Threads'] = df['Filename'].str.extract(r'c_omp-(\d+)-\d+-\d+\.txt')[0].astype(int)
# Sort the DataFrame by 'Threads'
#df['Threads'] = df['Filename'].str.extract(r'-(\d+)-\d+-\d+\.txt')[0].astype(int)
df.sort_values(by='Birds', inplace=True)

print(df['Method'])
print(df['Threads'])
# Plotting
plt.figure(figsize=(12, 8))

# Loop through each method to create a line plot
for method in df['Method'].unique():
    method_df = df[df['Method'] == method]
    plt.plot(method_df['Birds'], method_df['AverageTime'], marker='o', label=method)

# Adding labels and title
plt.xlabel('Number of Birds')
plt.ylabel('Average Time')
plt.title('Performance Comparison by Number of Birds with 64 threads')
plt.legend()
plt.grid(True)

# Show plot
plt.savefig("plot.png")
