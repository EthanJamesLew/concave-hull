"""
test script to demonstrate the concave hull
"""
import numpy as np
import concave_hull as ch
import matplotlib.pyplot as plt

# create point to find concave hull
pts = np.random.uniform(size=(10_000,2))

# compute the hull
hull = ch.concave_hull_2d(pts, 100, False)

# plot the hull
plt.fill(hull[:,0], hull[:, 1], linewidth=3, edgecolor='k', facecolor='w')

# plot the points
# plt.scatter(*pts.T, s=1)

# show the plot
plt.show()