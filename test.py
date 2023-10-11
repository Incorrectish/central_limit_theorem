import random
import matplotlib.pyplot as plt

# Define your map of numbers and their associated probabilities
probability_map = {1: 0.35, 3: 0.15, 4: 0.15, 6: 0.35}

# Normalize probabilities
total_prob = sum(probability_map.values())
normalized_probabilities = {
    key: value / total_prob for key, value in probability_map.items()
}

# Create cumulative distribution
cumulative_probabilities = []
cumulative_prob = 0
for key, value in normalized_probabilities.items():
    cumulative_prob += value
    cumulative_probabilities.append((key, cumulative_prob))


def gen_data_point(n):
    random_numbers = []
    for _ in range(n):
        rand = random.random()  # Generate a random number between 0 and 1
        for num, cumulative_prob in cumulative_probabilities:
            if rand <= cumulative_prob:
                random_numbers.append(num)
                break
    return sum(random_numbers) / n


n = int(input("Give a number of samples to to be used for the sample mean: "))
total_data_points = int(input("Give a number of times to sample the population: "))
data = [gen_data_point(n) for _ in range(total_data_points)]

plt.hist(data, bins=50, edgecolor="k")  # Adjust the number of bins as needed

# Add labels and a title
plt.xlabel("Value")
plt.ylabel("Frequency")
plt.title("Histogram Example")

# Show the histogram
plt.show()
