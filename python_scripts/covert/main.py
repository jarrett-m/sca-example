import matplotlib.pyplot as plt
import numpy as np

def moving_average(data, window_size=100000):
    sumsum = np.cumsum(data)
    sumsum[window_size:] = sumsum[window_size:] - sumsum[:-window_size]
    return sumsum[window_size - 1:] / window_size

with open("covert/out.txt", "r") as f:
    lines = f.readlines()
    lines = [l.replace("\n", "") for l in lines]
    ypoints = [int(x.split()[0]) for x in lines]
    seconds = lines[-1].split()[2]
    ypoints = moving_average(ypoints)  
    xpoints = np.linspace(0, int(seconds), len(ypoints))

    plt.plot(xpoints, ypoints)
    plt.ylabel("Request Trunaround Time")
    plt.xlabel("Time in Seconds")
    plt.show()