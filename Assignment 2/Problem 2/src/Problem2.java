import java.util.Scanner;
import java.util.concurrent.Semaphore;

public class Problem2 {
    private static final Semaphore showroomSemaphore = new Semaphore(1);
    private static int totalGuests;

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        System.out.print("Enter the number of guests (N): ");
        totalGuests = scanner.nextInt();
        scanner.close();

        System.out.println("Starting the Vase Viewing Party with " + totalGuests + " guests.");

        for (int i = 0; i < totalGuests; i++) {
            new Thread(new Guest()).start();
        }
    }

    static class Guest implements Runnable {
        @Override
        public void run() {
            try {
                showroomSemaphore.acquire();
                System.out.println(Thread.currentThread().getName() + " is viewing the vase.");
                Thread.sleep(1000); // Simulate time spent viewing the vase
                System.out.println(Thread.currentThread().getName() + " has finished viewing.");
                showroomSemaphore.release();
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }
    }
}
