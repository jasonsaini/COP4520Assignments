class Servant implements Runnable {
    private ConcurrentLinkedList list;
    private int[] presents;
    private int actionType; // 1 for add, 2 for remove, 3 for check

    Servant(ConcurrentLinkedList list, int[] presents, int actionType) {
        this.list = list;
        this.presents = presents;
        this.actionType = actionType;
    }

    @Override
    public void run() {
        for (int present : presents) {
            switch (actionType) {
                case 1:
                    list.add(present);
                    break;
                case 2:
                    list.remove(present);
                    break;
                case 3:
                    list.contains(present);
                    break;
            }
        }
    }
}

public class MinotaursParty {
    public static void main(String[] args) throws InterruptedException {
        final int NUM_PRESENTS = 500000;
        ConcurrentLinkedList list = new ConcurrentLinkedList();
        Thread[] servants = new Thread[4];
        int[] presents = new int[NUM_PRESENTS];

        // Initialize presents
        for (int i = 0; i < NUM_PRESENTS; i++) {
            presents[i] = i;
        }

        // Shuffle the presents to simulate randomness
        java.util.Collections.shuffle(java.util.Arrays.asList(presents));

        // Create and start servant threads
        for (int i = 0; i < 4; i++) {
            servants[i] = new Thread(new Servant(list, presents, i % 3 + 1));
            servants[i].start();
        }

        // Wait for all threads to finish
        for (Thread servant : servants) {
            servant.join();
        }

// Logic to check the final state of the list
        int presentsInList = 0;
        Node current = list.getHead();
        while (current != null) {
            presentsInList++;
            current = current.getNext();
        }

        System.out.println("Expected presents: " + NUM_PRESENTS);
        System.out.println("Presents in list: " + presentsInList);

        // Assuming presentsInList should be 0 if all presents are processed correctly
        if (presentsInList == 0) {
            System.out.println("All presents have been processed correctly.");
        } else {
            System.out.println("Mismatch in presents processing. Some presents are missing or extra.");
        }
}
