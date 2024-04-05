import java.util.Random;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

public class Problem2 {

    public static void main(String[] args) {
        SharedMemory sharedMemory = new SharedMemory();
        TemperatureReport report = new TemperatureReport(sharedMemory);
        Thread[] sensorThreads = new Thread[8];

        for (int i = 0; i < 8; i++) {
            sensorThreads[i] = new Thread(new TemperatureSensor(sharedMemory, i));
            sensorThreads[i].start();
        }

        // Assuming a simple loop to trigger the hourly report
        // In a real scenario, this should be replaced with a more sophisticated timing mechanism
        while (true) {
            try {
                Thread.sleep(3600000); // Sleep for 1 hour
                report.generateReport();
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                break;
            }
        }
    }

    static class TemperatureSensor implements Runnable {
        private final SharedMemory sharedMemory;
        private final int sensorId;
        private final Random random = new Random();

        public TemperatureSensor(SharedMemory sharedMemory, int sensorId) {
            this.sharedMemory = sharedMemory;
            this.sensorId = sensorId;
        }

        @Override
        public void run() {
            while (!Thread.currentThread().isInterrupted()) {
                int temperature = -100 + random.nextInt(171); // Random temperature between -100F and 70F
                sharedMemory.storeReading(sensorId, temperature);
                try {
                    Thread.sleep(60000); // Sleep for 1 minute
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                }
            }
        }
    }

    static class SharedMemory {
        private final Lock lock = new ReentrantLock();

        public void storeReading(int sensorId, int temperature) {
            lock.lock();
            try {
                // Store the temperature in a suitable data structure
                // Logic for maintaining the top 5 highest and lowest temperatures goes here
                // Update the interval with the largest temperature difference
            } finally {
                lock.unlock();
            }
        }

        // Methods to retrieve the hourly report data
        public int[] getHighestTemperatures() {
            // Return top 5 highest temperatures
            return new int[0];
        }

        public int[] getLowestTemperatures() {
            // Return top 5 lowest temperatures
            return new int[0];
        }

        public int[] getLargestTemperatureDifferenceInterval() {
            // Return the 10-minute interval with the largest temperature difference
            return new int[0];
        }
    }

    static class TemperatureReport {
        private final SharedMemory sharedMemory;

        public TemperatureReport(SharedMemory sharedMemory) {
            this.sharedMemory = sharedMemory;
        }

        public void generateReport() {
            int[] highestTemperatures = sharedMemory.getHighestTemperatures();
            int[] lowestTemperatures = sharedMemory.getLowestTemperatures();
     
