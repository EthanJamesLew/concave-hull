import numpy as np
import matplotlib.pyplot as plt
import concave_hull as ch


class SPZ:
    """Simple Sparse Polynomial Zonotope"""
    def __init__(self, G: np.ndarray, GI: np.ndarray, E: np.ndarray, iden: np.ndarray):
        self.G = G
        self.GI = GI
        self.E = E
        self.iden = iden

    def generate(self, alpha, beta):
        result = np.zeros((self.n,))
        for i in range(0, self.h):
            fac = 1.0
            for k in range(0, self.p):
                fac *= alpha[k]**self.E[k, i]
            result = result + fac * self.G[:, i]
        for j in range(0, self.q):
            result = result + beta[j] * self.GI[:, j]
        return result

    @property
    def h(self):
        """number of dependent generator"""
        return self.G.shape[1]

    @property
    def p(self):
        """number of dependent factors"""
        return self.E.shape[0]

    @property
    def n(self):
        return self.G.shape[0]

    @property
    def q(self):
        if len(self.GI) > 0:
            return self.GI.shape[1]
        else:
            return 0
    

if __name__ == "__main__":
    import time

    # see Kochdumper et al.
    spz = SPZ(
        np.array(
            [
                [4, 2, 1, 2],
                [4, 0, 2, 2]
            ]
        ),
        np.array(
            [
                [1],
                [0]
            ]
        ),
        np.array(
            [
                [0, 1, 0, 3],
                [0, 0, 1, 1]
            ]
        ),
        np.array([1, 2])
    )

    N = 1_000_000
    alphas = np.random.uniform(size=(N, spz.p)) * 2 - 1
    betas = np.random.uniform(size=(N, spz.q)) * 2 - 1
    pts = np.zeros((N, 2))
    for idx, (alpha, beta) in enumerate(zip(alphas, betas)):
        pts[idx] = (spz.generate(alpha, beta))
    
    # compute the hull
    start = time.time()
    hull = ch.concave_hull_2d(pts, 40, False)
    end = time.time()
    print(f"Compute Concave Hull Time [{N} Points]: {end - start: 2.4f} seconds")
    
    # plot the points
    plt.scatter(*pts.T, s=1, alpha=0.3, c='C0')

    # plot the hull
    plt.plot(hull[:,0], hull[:, 1], linewidth=3, c='k')
    
    plt.title(f'Concave Hull Approximation [{N} samples]')
    plt.xlabel('$x$')
    plt.ylabel('$y$')

    # save to doc
    plt.savefig('./doc/img/spz.png')

    # show the plot
    plt.show()