class TreeNode {
  int val;
  TreeNode left;
  TreeNode right;

  TreeNode(int x) {
    val = x;
  }
}

class Solution {
  class Result2 {
    boolean pHit;
    boolean qHit;
    TreeNode node;

    Result2(boolean ph, boolean qh, TreeNode tn) {
      pHit = ph;
      qHit = qh;
      node = tn;
    }

    public String toString() {
      return pHit + "," + qHit + "," + node;
    }
  }

  private TreeNode search1(TreeNode root, TreeNode t) {
    if (root == null) {
      return null;
    }
    if (root.val == t.val) {
      return root;
    }
    TreeNode l = null, r = null;
    if (root.left != null) {
      l = search1(root.left, t);
    }
    if (root.right != null) {
      r = search1(root.right, t);
    }
    if (l != null) {
      return l;
    }
    if (r != null) {
      return r;
    }
    return null;
  }

  private Result2 search2(TreeNode root, TreeNode p, TreeNode q) {
    if (root == null) {
      return new Result2(false, false, null);
    }
    boolean pFound = false;
    boolean qFound = false;
    if (root.val == p.val) {
      TreeNode t = search1(root, q);
      if (t != null) {
        qFound = true;
      }
      return new Result2(true, qFound, root);
    } else if (root.val == q.val) {
      TreeNode t = search1(root, p);
      if (t != null) {
        pFound = true;
      }
      return new Result2(pFound, true, root);
    } else {
      Result2 l = null, r = null;
      if (root.left != null) {
        l = search2(root.left, p, q);
      }
      if (root.right != null) {
        r = search2(root.right, p, q);
      }
      if (l != null && l.pHit && l.qHit) {
        return l;
      }
      if (r != null && r.pHit && r.qHit) {
        return r;
      }
      return new Result2(((l != null && l.pHit) || (r != null && r.pHit)),
                         ((l != null && l.qHit) || (r != null && r.qHit)), root);
    }
  }

  public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
    Result2 r = search2(root, p, q);
    return r.node;
  }

  public static void main(String[] args) {

  }
}
