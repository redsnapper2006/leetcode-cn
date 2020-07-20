class Node {
  public int val;
  public Node left;
  public Node right;

  public Node() {
  }

  public Node(int _val) {
    val = _val;
  }

  public Node(int _val, Node _left, Node _right) {
    val = _val;
    left = _left;
    right = _right;
  }
};

public class Solution {
  private Node recur(Node root) {
    if (root == null) {
      return null;
    }
    Node left = recur(root.left);
    Node right = recur(root.right);

    Node head = null;
    if (left != null) {
      head = left;
      Node p = left;
      while (p.right != null) {
        p = p.right;
      }

      p.right = root;
      root.left = p;
    } else {
      head = root;
    }

    root.right = right;
    if (right != null) {
      right.left = root;
    }
    return head;
  }

  public Node treeToDoublyList(Node root) {
    if (root == null) {
      return null;
    }
    Node head = recur(root);
    Node p = head;

    while (p.right != null) {
      p = p.right;
    }
    p.right = head;
    head.left = p;
    return head;
  }
}
