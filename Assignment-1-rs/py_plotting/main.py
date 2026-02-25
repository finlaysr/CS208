from matplotlib.ticker import MultipleLocator
import csv
import os
from matplotlib import pyplot as plt
from numpy import average

BENCH_TYPE = "reversed"

lenghts: list[list[int]] = []
selection: list[list[int]] = []
merge: list[list[int]] = []

selection_avg = []
selection_min = []
selection_max = []

merge_avg = []
merge_min = []
merge_max = []

crossover = 0


def read_data(path):
    files = os.listdir(path)
    i = 0
    for file in files:
        lenghts.append([])
        selection.append([])
        merge.append([])

        with open(path + file) as csv_data:
            reader = csv.reader(csv_data, delimiter=",")
            reader.__next__()
            for row in reader:
                lenghts[i].append(int(row[0]))
                selection[i].append(int(row[1]))
                merge[i].append(int(row[2]))
        i += 1


def proccess_data():
    global crossover
    for group in zip(*selection):
        selection_avg.append(average(group))
        selection_min.append(min(group))
        selection_max.append(max(group))
    for group in zip(*merge):
        merge_avg.append(average(group))
        merge_min.append(min(group))
        merge_max.append(max(group))
    for m, s in zip(merge_avg, selection_avg):
        if m < s:
            crossover = lenghts[0][merge_avg.index(m)]
            print("crossover: ", crossover)
            break
    print(
        "max merge delta: ",
        max([(hi - low) for (hi, low) in zip(merge_max, merge_min)]),
    )
    print(
        "max selection delta: ",
        max([(hi - low) for (hi, low) in zip(selection_max, selection_min)]),
    )


def make_plot():
    fig, ax = plt.subplots()
    ax.plot(lenghts[0], merge_avg, color="blue", label="Merge Sort")
    ax.fill_between(lenghts[0], merge_max, merge_min, color="blue", alpha=0.2)

    ax.plot(lenghts[0], selection_avg, color="red", label="Selection Sort")
    ax.fill_between(lenghts[0], selection_max, selection_min, color="red", alpha=0.2)

    ax.set_xlabel("Array length")
    ax.set_ylabel("Instruction count")
    ax.legend()
    ax.grid(True, which="both")
    ax.set_xlim(left=0, right=100)
    ax.set_ylim(bottom=0, top=70_000)
    ax.xaxis.set_major_locator(MultipleLocator(10))
    ax.tick_params(axis="both", grid_color="black")
    ax.xaxis.set_minor_locator(MultipleLocator(1))
    ax.yaxis.set_minor_locator(MultipleLocator(5_000))

    ax.annotate(
        "Point at which merge \nsort becomes more efficient\n("
        + str(crossover)
        + ", "
        + str(merge_avg[crossover])
        + ")",
        xycoords="data",
        xytext=(0.01, 0.99),
        textcoords="axes fraction",
        va="top",
        ha="left",
        size=10,
        bbox=dict(facecolor="white", alpha=1.0),
        arrowprops=dict(facecolor="black", shrink=0.01),
        xy=(crossover, merge_avg[crossover]),
    )
    ax.plot([crossover], merge_avg[crossover], "o", color="black")

    plt.title(f"Sorting Comparison - {BENCH_TYPE}")
    plt.savefig(f"{BENCH_TYPE}.png", dpi=600, bbox_inches="tight")
    plt.show()


def main():
    read_data(f"../output_data/{BENCH_TYPE}/")
    proccess_data()
    make_plot()


if __name__ == "__main__":
    main()
