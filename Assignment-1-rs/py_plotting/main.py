import csv
import os
from pandas import DataFrame
from seaborn import lineplot
from matplotlib import pyplot

lenghts = []
selection = []
merge = []
run_id = []


data = DataFrame({"length": [], "instructions": [], "run_id": []})


def read_data(path):
    global data
    files = os.listdir(path)
    i = 0
    for file in files:
        with open(path + file) as csv_data:
            reader = csv.reader(csv_data, delimiter=",")
            reader.__next__()
            for row in reader:
                lenghts.append(int(row[0]))
                selection.append(int(row[1]))
                merge.append(int(row[2]))
                run_id.append(i)
        i += 1

    data = DataFrame({"length": lenghts, "instructions": merge, "run_id": run_id})

    print(lenghts)
    print(selection)
    print(merge)
    print(run_id)


def show_plot():
    lineplot(data=data, x="length", y="instructions", errorbar=("pi", 100))
    pyplot.show()


def main():
    print("Hello from py-plotting!")
    read_data("../output_data/")
    show_plot()


if __name__ == "__main__":
    main()
