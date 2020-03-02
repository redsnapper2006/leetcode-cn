import java.util.ArrayList;

class Node {
  public int val;
  public Node left;
  public Node right;
  public Node next;

  public Node() {
  }

  public Node(int _val) {
    val = _val;
  }

  public Node(int _val, Node _left, Node _right, Node _next) {
    val = _val;
    left = _left;
    right = _right;
    next = _next;
  }
};

public class Solution {

  private void recur(Node root, ArrayList<Node> s, int hier) {
    if (root == null) {
      return;
    }
    if (s.size() < hier) {
      s.add(hier - 1, root);
    } else {
      Node prev = s.get(hier - 1);
      prev.next = root;
      s.set(hier - 1, root);
    }
    recur(root.left, s, hier + 1);
    recur(root.right, s, hier + 1);
  }

  public Node connect(Node root) {
    ArrayList<Node> s = new ArrayList();

    recur(root, s, 1);
    return root;
  }
}

