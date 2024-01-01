"""Run on the Hessigheim benchmark"""
import argparse
import pathlib
import time
import numpy as np
import matplotlib.pyplot as plt
import concave_hull as ch

def read_file_to_numpy(filename):
    """
    Reads a file and extracts the first two numbers (X, Y) from each line
    (skipping the first line) into a 2D Nx2 numpy array.
    """
    with open(filename, 'r') as file:
        lines = file.readlines()[1:]  # Skip the first line
        data = [list(map(float, line.split()[0:2])) for line in lines]
        return np.array(data)

def main():
    # Parsing the command line argument for the filename
    parser = argparse.ArgumentParser(description='Process a txt file to extract X, Y coordinates.')
    parser.add_argument('filename', type=str, help='The filename of the txt file')
    args = parser.parse_args()
    basename = pathlib.Path(args.filename).stem

    # Process the file and get the numpy array
    pts = read_file_to_numpy(args.filename)

    # compute the hull
    start = time.time()
    hull = ch.concave_hull_2d(pts, 400, False)
    end = time.time()
    print(f"Compute Concave Hull Time [{len(pts)} Points]: {end - start: 2.4f} seconds")

    # plot the hull
    plt.fill(hull[:,0], hull[:, 1], linewidth=3, edgecolor='k', facecolor='w')

    # plot the points
    plt.scatter(*pts.T, alpha=0.02, s=1)
    
    plt.title(f'{basename} Concave Hull Approximation [{len(pts)} points]')
    plt.xlabel('$x$')
    plt.ylabel('$y$')

    xdiff = hull[:, 0].max() - hull[:, 0].min() 
    ydiff = hull[:, 1].max() - hull[:, 1].min() 
    plt.xlim(hull[:, 0].min() - xdiff * 0.05, hull[:, 0].max() + xdiff * 0.05)
    plt.ylim(hull[:, 1].min() - ydiff * 0.05, hull[:, 1].max() + ydiff * 0.05)

    # save to doc
    plt.savefig(f'./doc/img/{basename}.png')

    # show the plot
    plt.show()

if __name__ == "__main__":
    main()

