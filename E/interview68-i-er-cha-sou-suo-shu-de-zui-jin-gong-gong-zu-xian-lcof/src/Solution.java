class TreeNode {
  int val;
  TreeNode left;
  TreeNode right;

  TreeNode(int x) {
    val = x;
  }
}

public class Solution {
  public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
    TreeNode l = null, r = null;
    if (p.val > q.val) {
      r = p;
      l = q;
    } else {
      r = q;
      l = p;
    }
    if ((root.val >= l.val) && (root.val <= r.val)) {
      return root;
    }
    if (root.val < l.val) {
      return lowestCommonAncestor(root.right, l, r);
    }
    return lowestCommonAncestor(root.left, l, r);
  }
}
