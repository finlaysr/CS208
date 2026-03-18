package algorithms.studentAlgorithms;

import algorithms.SchedulingAlgorithm;
import java.util.ArrayList;
import java.util.Arrays;

public class FinlayRobbAlgorithm extends SchedulingAlgorithm {

  private final String name = "Finlay Robb Algorithm";
  double[][] etcMatrix;
  int numberOfProcessors;
  int numberOfTasks;

  @Override
  public double[] runAlgorithm(double[][] etcMatrix) {
    this.etcMatrix = etcMatrix;
    numberOfProcessors = etcMatrix.length;
    numberOfTasks = etcMatrix[0].length;

    double[] processorTimes = new double[numberOfProcessors];

    // 2D array to hold the tasks done by each CPU
    ArrayList<Integer>[] tasksUsed = new ArrayList[numberOfProcessors];
    for (int i = 0; i < tasksUsed.length; i++) {
      tasksUsed[i] = new ArrayList<>();
    }

    // Run Min-min algorithm
    minMin(processorTimes, tasksUsed);

    // Run Local search
    localSearch(processorTimes, tasksUsed);

    // check every task has been used
    ArrayList<Integer> allTasks = new ArrayList<>();
    for (int i = 0; i < numberOfTasks; i++) {
      allTasks.add(i);
    }

    Arrays.stream(tasksUsed).forEach(inner -> inner.forEach(allTasks::remove));
    if (!allTasks.isEmpty()) {
      System.out.println("All tasks not used!");
      System.exit(1);
    }

    // check tasks add up properly
    final double epsilon = 1e-6; // need this due to floating point errors
    for (int i = 0; i < numberOfProcessors; i++) {
      int finalI = i;
      double summedTime =
          tasksUsed[i].stream().map(t -> etcMatrix[finalI][t]).reduce(0.0, Double::sum);
      if (Math.abs(summedTime - processorTimes[i]) > epsilon) {
        System.out.println("Task times don't match!");
        System.exit(1);
      }
    }

    return processorTimes;
  }

  public String getName() {
    return name;
  }

  private void minMin(double[] processorTimes, ArrayList<Integer>[] tasksUsed ){
    ArrayList<Integer> unused = new ArrayList<>();
    for (int i = 0; i < numberOfTasks; i++) {
      unused.add(i);
    }

    while (!unused.isEmpty()) {
      int minCPU = 0;
      int minTask = unused.getFirst();
      for (Integer task : unused) {
        for (int cpu = 0; cpu < numberOfProcessors; cpu++) {
          if ((processorTimes[cpu] + etcMatrix[cpu][task])
            < (processorTimes[minCPU] + etcMatrix[minCPU][minTask])) {
            minCPU = cpu;
            minTask = task;
          }
        }
      }

      unused.remove((Integer) minTask);
      processorTimes[minCPU] += etcMatrix[minCPU][minTask];

      tasksUsed[minCPU].add(minTask);
    }
  }

  private void localSearch(double[] processorTimes, ArrayList<Integer>[] tasksUsed ){
    int c = 0; // ensure we don't get stuck in a loop
    while (c++ < 100000) {
      // Find current worst cpu in processorTimes
      int worstCPU = 0;
      for (int cpu = 1; cpu < numberOfProcessors; cpu++) {
        if (processorTimes[cpu] > processorTimes[worstCPU]) {
          worstCPU = cpu;
        }
      }

      // Find the move that will reduce the makespan the most
      double bestMS = processorTimes[worstCPU];
      int bestTaskToMove = 999999;
      int bestCPUToMoveTo = 999999;

      // try moving all the different tasks off the worst cpu
      for (int task : tasksUsed[worstCPU]) {
        // find the best place to move it to
        for (int newCPU = 0; newCPU < numberOfProcessors; newCPU++) {
          if (worstCPU != newCPU) {
            double newMS =
              Math.max(
                processorTimes[worstCPU] - etcMatrix[worstCPU][task],
                processorTimes[newCPU] + etcMatrix[newCPU][task]);
            if (newMS < bestMS) {
              bestMS = newMS;
              bestTaskToMove = task;
              bestCPUToMoveTo = newCPU;
            }
          }
        }
      }

      // An improvement wasn't found so stop
      if (bestTaskToMove == 999999 || bestCPUToMoveTo == 999999) {
        break;
      }

      // move task from worst cpu to best cpu
      tasksUsed[worstCPU].remove((Integer) bestTaskToMove);
      tasksUsed[bestCPUToMoveTo].add(bestTaskToMove);

      processorTimes[worstCPU] -= etcMatrix[worstCPU][bestTaskToMove];
      processorTimes[bestCPUToMoveTo] += etcMatrix[bestCPUToMoveTo][bestTaskToMove];
    }
  }
}
