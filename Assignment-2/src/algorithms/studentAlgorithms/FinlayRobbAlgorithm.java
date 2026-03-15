package algorithms.studentAlgorithms;

import algorithms.SchedulingAlgorithm;
import java.util.ArrayList;

public class FinlayRobbAlgorithm extends SchedulingAlgorithm {
  private final String name = "Min Min Algorithm";

  @Override
  public double[] runAlgorithm(double[][] etcMatrix){
    int numberOfProcessors = etcMatrix.length;
    int numberOfTasks = etcMatrix[0].length;
    double[] processorTimes = new double[numberOfProcessors];

    ArrayList<Integer> unused = new ArrayList<>();
    for (int i = 0; i < numberOfTasks; i++) {
      unused.add(i);
    }

    while (!unused.isEmpty()){
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
    }

    return processorTimes;
  }

  public String getName(){
    return name;
  }

}
