import java.util.concurrent.locks.ReentrantLock;

class ConcurrentLinkedList {
    private Node head;
    private ReentrantLock lock = new ReentrantLock();

    private class Node {
        int data;
        Node next;

        Node(int data) {
            this.data = data;
        }


    public Node getNext() {
        return next;
    }
}

    public void add(int data) {
        lock.lock();
        try {
            if (head == null || data < head.data) {
                Node newNode = new Node(data);
                newNode.next = head;
                head = newNode;
            } else {
                Node current = head;
                while (current.next != null && current.next.data < data) {
                    current = current.next;
                }
                Node newNode = new Node(data);
                newNode.next = current.next;
                current.next = newNode;
            }
        } finally {
            lock.unlock();
        }
    }

    public boolean remove(int data) {
        lock.lock();
        try {
            if (head == null) return false;
            if (head.data == data) {
                head = head.next;
                return true;
            }
            Node current = head;
            while (current.next != null && current.next.data != data) {
                current = current.next;
            }
            if (current.next == null) return false;
            current.next = current.next.next;
            return true;
        } finally {
            lock.unlock();
        }
    }

    public boolean contains(int data) {
        lock.lock();
        try {
            Node current = head;
            while (current != null) {
                if (current.data == data) return true;
                current = current.next;
            }
            return false;
        } finally {
            lock.unlock();
        }
    }

    
    public Node getHead() {
        return head;
    }
}
