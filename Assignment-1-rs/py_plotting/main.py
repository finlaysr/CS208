from matplotlib.ticker import MultipleLocator
import csv
import os
from matplotlib import pyplot as plt
from numpy import average
from sys import argv, exit

lenghts: list[list[int]] = []
selection: list[list[int]] = []
merge: list[list[int]] = []

selection_avg = []
selection_min = []
selection_max = []
selection_delta = 0
selection_delta_percent = 0

merge_avg = []
merge_min = []
merge_max = []
merge_delta = 0
merge_delta_percent = 0

crossover = 0
bench_type = ""


def read_args():
    global bench_type
    with open("../benches/bench_type.txt") as file:
        bench_type = file.read().splitlines()[0]
    if bench_type not in ("reversed", "linear", "random"):
        exit("Invalid bench type!")


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
    global merge_delta, merge_delta_percent, selection_delta, selection_delta_percent
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

    for hi, low in zip(merge_max, merge_min):
        if (hi - low) > merge_delta:
            merge_delta = hi - low
            merge_delta_percent = round(merge_delta / hi * 100, 2)

    for hi, low in zip(selection_max, selection_min):
        if (hi - low) > selection_delta:
            selection_delta = hi - low
            selection_delta_percent = round(selection_delta / hi * 100, 2)


def make_plot():
    fig, ax = plt.subplots(figsize=(14, 8))
    ax.plot(lenghts[0], merge_avg, color="blue", label="Merge Sort", lw=1, marker=".")
    ax.fill_between(lenghts[0], merge_max, merge_min, color="blue", alpha=0.2)

    ax.plot(
        lenghts[0], selection_avg, color="red", label="Selection Sort", lw=1, marker="."
    )
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
        f"""Point at which merge \nsort becomes more efficient
        ({str(crossover)}, {str(int(merge_avg[crossover]))})""",
        xycoords="data",
        xytext=(0.02, 0.97),
        textcoords="axes fraction",
        va="top",
        ha="left",
        size=8,
        bbox=dict(facecolor="white", alpha=1.0),
        arrowprops=dict(facecolor="black", shrink=0.01, width=2),
        xy=(crossover, merge_avg[crossover]),
    )
    ax.plot([crossover], merge_avg[crossover], "o", color="black")

    ax.text(
        0.98,
        0.025,
        f"""Iterations: {str(len(lenghts))}
        max merge delta: {merge_delta} ({merge_delta_percent:.2f}%)
        max selection delta: {selection_delta} ({selection_delta_percent:.2f}%)""",
        verticalalignment="bottom",
        horizontalalignment="right",
        transform=ax.transAxes,
        size=7,
        bbox=dict(facecolor="white", alpha=1.0),
    )

    plt.title(f"Sorting Comparison - {bench_type}")
    plt.savefig(f"graphs/{bench_type}.png", dpi=1000, bbox_inches="tight")
    plt.show()


def main():
    read_args()
    read_data(f"../output_data/{bench_type}/")
    proccess_data()
    make_plot()


if __name__ == "__main__":
    main()
