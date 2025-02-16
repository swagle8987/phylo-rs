import numpy as np
import umap
import seaborn as sns
from tqdm import tqdm

sns.set(style='white', context='notebook', rc={'figure.figsize':(14,10)})

dists = np.zeros((10,10))

num_lines = ((dists.shape[0]*dists.shape[1])-dists.shape[0])/2

fname = "examples/tree-space.out"
pbar = tqdm(total=num_lines)
with open(fname, "r") as f:
    for line in f:
        parts = line[:-1].split("-")
        x = int(parts[0])
        y = int(parts[1])
        dist = int(parts[2])
        dists[x,y] = dist
        dists[y,x] = dist
        pbar.update(1)

embedding = umap.UMAP(metric='precomputed', low_memory=False).fit_transform(dists)

fig, ax = plt.subplots()
ax.scatter(
    embedding[:, 0],
    embedding[:, 1],
)

ax.grid()

fig.savefig("tree-space.png")