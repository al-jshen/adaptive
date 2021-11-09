import matplotlib.pyplot as plt
import matplotlib.animation
import numpy as np

d = np.loadtxt("t.txt", delimiter=",")

fig, ax = plt.subplots()
x, y = [], []
sc = ax.scatter(x, y)
plt.xlim(d[:, 0].min(), d[:, 0].max())
plt.ylim(d[:, 1].min(), d[:, 1].max() * 1.01)


def animate(i):
    x.append(d[i, 0])
    y.append(d[i, 1])
    sc.set_offsets(np.c_[x, y])


ani = matplotlib.animation.FuncAnimation(
    fig, animate, frames=len(d), interval=10, repeat=False
)

plt.show()
