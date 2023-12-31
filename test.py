import numpy as np
import concave_hull as ch
import matplotlib.pyplot as plt

# create point to find concave hull
pts = np.random.uniform(size=(10_000,2))

# compute the hull
res = ch.concave_hull_py(pts, 100, False)

# build a numpy array (TODO: return the automatically)
hull = []
for point in res:
    pt = [point.x, point.y]
    hull.append(pt)
hull = np.vstack(hull)

# plot the points
plt.scatter(*pts.T, s=1)

# plot the edges
for pta, ptb in zip(hull[:-1], hull[1:]):
    plt.plot([pta[0], ptb[0]], [pta[1], ptb[1]], c='k')
pta, ptb = hull[-1], hull[0]
plt.plot([pta[0], ptb[0]], [pta[1], ptb[1]], c='k')

# show the plot
plt.show()