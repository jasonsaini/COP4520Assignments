import java.util.concurrent.atomic.AtomicInteger;
import java.util.Scanner;

public class Main {
    private static AtomicInteger guestsWhoEntered = new AtomicInteger(0);
    private static boolean cupcake = true;
    private static boolean gameFinished = false;
    private static final Object lock = new Object();
    private static int totalGuests;

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        System.out.print("Enter the number of guests (N): ");
        totalGuests = scanner.nextInt();
        scanner.close();

        System.out.println("Starting the Minotaur's party with " + totalGuests + " guests.");

        Thread[] guests = new Thread[totalGuests];
        for (int i = 0; i < totalGuests; i++) {
            guests[i] = new Thread(new Guest(i == 0));
            guests[i].start();
        }

        for (Thread guest : guests) {
            try {
                guest.join();
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }

        System.out.println("Total guests who entered: " + guestsWhoEntered.get());
        System.out.println("The game has finished.");
    }

    static class Guest implements Runnable {
        private boolean isLeader;

        public Guest(boolean isLeader) {
            this.isLeader = isLeader;
        }

        @Override
        public void run() {
            while (!gameFinished) {
                enterLabyrinth();
            }
        }

        private void enterLabyrinth() {
            synchronized (lock) {
                if (gameFinished) {
                    return;
                }

                if (isLeader) {
                    System.out.println("Leader guest entering the labyrinth.");
                    if (!cupcake) {
                        guestsWhoEntered.incrementAndGet();
                        cupcake = true;
                        System.out.println("Leader found no cupcake. Incrementing count.");
                    }
                    if (guestsWhoEntered.get() == totalGuests - 1) {
                        announceAllGuestsVisited();
                    }
                } else {
                    System.out.println("Guest entering the labyrinth.");
                    if (cupcake) {
                        cupcake = false;
                        System.out.println("Guest found a cupcake and left it.");
                    }
                }
            }
        }

        private void announceAllGuestsVisited() {
            synchronized (lock) {
                if (!gameFinished) {
                    System.out.println("Leader guest announces: All guests have visited the labyrinth!");
                    gameFinished = true;
                }
            }
        }
    }
}
