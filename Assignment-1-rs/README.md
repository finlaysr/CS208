# Assignment 1

* Exploring how an algorithms speed changes with problem size
* Comparing Merge Sort and Selection Sort
* The time taken for each sorting algorithm per array length should be compared to find the crossover point
* A report on the results should be written

## Project Structure
* The project uses the rust library `gungraun` for extremely accurate benchmarking
* Code overview:
  1. The main file starts and loads the type of benchmark to be run from `benches/bench_type.txt`. This is either `random`, `linear`, or `reversed`. This determines the layout of the array to be sorted
  2. The main file clears out all the directories to be used. This involves:
    * `graphs` - legacy rust graphs generated after each benchmark
    * `output_data` - where the gungraun data will be exported to
    * `py_plotting/graphs` - modern python generated graphs
    * `test_data` - data each sorting algorithm uses, stored for verification purposes
  3. The benchmark is run repeatedly a certain amount of times
  4. The data is extracted from `.log` files and stored in csv files in `output_data`
  5. A python script runs `matplotlib` which generates a graph combining the data from all the runs
  6. This graph is saved to a file

