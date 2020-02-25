import java.util.Stack;

class Node {
  public int val;
  public Node prev;
  public Node next;
  public Node child;
}

public class Solution {
  public Node flatten(Node head) {
    if (head == null) {
      return head;
    }
    Node newHead = new Node();
    Node cur = newHead;

    Stack<Node> stack = new Stack<>();
    stack.push(head);
    while (!stack.isEmpty()) {
      Node p = stack.pop();
      cur.next = p;
      p.prev = cur;
      if (p.next != null) {
        stack.push(p.next);
      }
      if (p.child != null) {
        stack.push(p.child);
        p.child = null;
      }
      cur = cur.next;
    }
    newHead.next.prev = null;
    return newHead.next;
  }
}
