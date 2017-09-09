import numpy as np
import seaborn as sns
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker

data = np.loadtxt("../src/main.txt",
    dtype = 'int32',
    delimiter = ':',
    converters = { 0: lambda s: int(s, 16) }
)

axes = sns.barplot(data[:,0], data[:,1])
axes.set_title("byte value histogram")
axes.set_xlabel("byte value")
axes.set_ylabel("occurences")

axes.xaxis.set_major_locator(ticker.MultipleLocator(15))
axes.xaxis.set_major_formatter(ticker.ScalarFormatter())

plt.savefig("main.svg")
plt.show()
