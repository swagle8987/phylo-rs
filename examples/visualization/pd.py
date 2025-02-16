import matplotlib.pyplot as plt
import seaborn as sns

sns.set(style='white', rc={'figure.figsize':(14,10)})

data = dict()
with open("examples/phylogenetic-diversity/pds.out", "r") as f:
    for line in f.readlines():
        clade = line.split(": ")[0]
        pds = [float(y) for y in line.split(": ")[1].split(",")]
        data[clade] = pds

x = range(2015, 2023)
fig, ax = plt.subplots()
linestyles = ['solid', 'dashdot', 'dashed']
markers = ["X", "o","^","s"]
for n,clade in enumerate(data):
    # print(n,clade, data[clade])
    ax.plot(x,data[clade],label=clade.upper(), linestyle=linestyles[n%3], marker=markers[n%4], markersize=10)
    ax.grid()
box = ax.get_position()
ax.set_position([box.x0, box.y0, box.width * 0.8, box.height])
ax.set_xlabel("Year")
ax.set_ylabel("Phylogenetic Diversity")
ax.legend(loc="best", bbox_to_anchor=(1, 1))
fig.savefig("phylogenetic-diversity.png")