package algorithms.studentAlgorithms;

import algorithms.SchedulingAlgorithm;
import java.util.ArrayList;
import java.util.Arrays;

public class FinlayRobbAlgorithm extends SchedulingAlgorithm {

  private final String name = "Min Min Algorithm";

  @Override
  public double[] runAlgorithm(double[][] etcMatrix) {
    int numberOfProcessors = etcMatrix.length;
    int numberOfTasks = etcMatrix[0].length;

    // 2D array to hold the tasks done by each CPU
    ArrayList<Integer>[] tasksUsed = new ArrayList[numberOfProcessors];
    for (int i = 0; i < tasksUsed.length; i++) {
      tasksUsed[i] = new ArrayList<>();
    }

    double[] processorTimes = new double[numberOfProcessors];

    ArrayList<Integer> unused = new ArrayList<>();
    for (int i = 0; i < numberOfTasks; i++) {
      unused.add(i);
    }

    // Min-min algorithm
    while (!unused.isEmpty()) {
      int minCPU = 0;
      int minTask = unused.getFirst();
      for (Integer task : unused) {
        for (int cpu = 0; cpu < numberOfProcessors; cpu++) {
          if ((processorTimes[cpu] + etcMatrix[cpu][task]) < (processorTimes[minCPU]
              + etcMatrix[minCPU][minTask])) {
            minCPU = cpu;
            minTask = task;
          }
        }
      }

      unused.remove((Integer) minTask);
      processorTimes[minCPU] += etcMatrix[minCPU][minTask];

      tasksUsed[minCPU].add(minTask);
    }

    // Local search
    boolean continueSearch = true;
    int c = 0;

    while (continueSearch) {
      // Find current worst cpu in processorTimes
      int worstCPU = 0;
      for (int cpu = 1; cpu < numberOfProcessors; cpu++) {
        if (processorTimes[cpu] > processorTimes[worstCPU]) {
          worstCPU = cpu;
        }
      }

      // Find the move that will reduce the makespan the most
      double bestMS = processorTimes[worstCPU];
      double oldMS = bestMS;
      int bestTaskToMove = 999999;
      int bestCPUToMoveTo = 999999;

      double ams = 0;

      for (int task: tasksUsed[worstCPU]) {
        // find the best place to move it to
        for (int newCPU = 0; newCPU < numberOfProcessors; newCPU++) {
          if (worstCPU != newCPU) {
            double newMS = Math.max(processorTimes[worstCPU] - etcMatrix[worstCPU][task],
                processorTimes[newCPU] + etcMatrix[newCPU][task]);
            if (newMS < bestMS) {
              bestMS = newMS;
              ams = newMS;
              bestTaskToMove = task;
              bestCPUToMoveTo = newCPU;
            }
          }
        }
      }

      // An improvement wasn't found or got stuck in some loop
      if (bestTaskToMove == 999999 || bestCPUToMoveTo == 999999 || c > 100000) {
        break;
      }

      // move task from worst cpu to best cpu
      tasksUsed[worstCPU].remove(tasksUsed[worstCPU].indexOf(bestTaskToMove));
      tasksUsed[bestCPUToMoveTo].add(bestTaskToMove);

      processorTimes[worstCPU] -= etcMatrix[worstCPU][bestTaskToMove];
      processorTimes[bestCPUToMoveTo] += etcMatrix[bestCPUToMoveTo][bestTaskToMove];

    }

    return processorTimes;
  }

  public String getName() {
    return name;
  }

}
