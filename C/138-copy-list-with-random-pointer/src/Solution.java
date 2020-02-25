import java.util.HashMap;

class Node {
  int val;
  Node next;
  Node random;

  public Node(int val) {
    this.val = val;
    this.next = null;
    this.random = null;
  }
}


class Solution {
  public Node copyRandomList(Node head) {
    HashMap<Node, Node> map = new HashMap<>();
    Node p = head;
    Node copyHead = new Node(0);
    Node q = copyHead;

    for (; p != null; ) {
      Node t = new Node(p.val);
      q.next = t;
      map.put(p, t);
      q = q.next;
      p = p.next;
    }

    p = head;
    q = copyHead.next;
    for (; p != null; ) {
      if (p.random != null)
        q.random = map.get(p.random);
      q = q.next;
      p = p.next;
    }

    return copyHead.next;
  }
}
