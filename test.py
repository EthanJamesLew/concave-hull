import numpy as np
import concave_hull as ch
import matplotlib.pyplot as plt

# create point to find concave hull
pts = np.random.uniform(size=(10_000,2))

# compute the hull
hull = ch.concave_hull_2d(pts, 100, False)

# plot the points
plt.scatter(*pts.T, s=1)

# plot the edges
for pta, ptb in zip(hull[:-1], hull[1:]):
    plt.plot([pta[0], ptb[0]], [pta[1], ptb[1]], c='k')
pta, ptb = hull[-1], hull[0]
plt.plot([pta[0], ptb[0]], [pta[1], ptb[1]], c='k')

# show the plot
plt.show()