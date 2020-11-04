public class TreeNode {
  int val;
  TreeNode left;
  TreeNode right;

  TreeNode(int x) {
    val = x;
  }
}

class Solution {
  private int[] recur(final TreeNode root, final TreeNode target, int[] hier) {
    if (root == null) {
      return null;
    }
    if (root == target) {
      return hier;
    }
    int[] left = new int[hier.length + 1];
    System.arraycopy(hier, 0, left, 0, hier.length);
    left[left.length - 1] = 1;
    int[] leftResult = recur(root.left, target, left);
    int[] right = new int[hier.length + 1];
    System.arraycopy(hier, 0, right, 0, hier.length);
    right[right.length - 1] = 2;
    int[] rightResult = recur(root.right, target, right);
    if (leftResult != null) {
      return leftResult;
    } else {
      return rightResult;
    }
  }

  public final TreeNode getTargetCopyV2(final TreeNode original, final TreeNode cloned,
                                        final TreeNode target) {
    int[] r = new int[1];
    int[] guide = recur(original, target, r);
    if (guide.length == 1) {
      return cloned;
    }
    TreeNode tn = cloned;
    for (int i = 1; i < guide.length; i++) {
      if (guide[i] == 1) {
        tn = tn.left;
      } else {
        tn = tn.right;
      }
    }
    return tn;
  }

  public final TreeNode getTargetCopy(final TreeNode original, final TreeNode cloned,
                                      final TreeNode target) {
    if (original == null) {
      return null;
    }
    if (original == target) {
      return cloned;
    }
    TreeNode left = getTargetCopy(original.left, cloned.left, target);
    if (left != null) {
      return left;
    }
    TreeNode right = getTargetCopy(original.right, cloned.right, target);
    if (right != null) {
      return right;
    }
    return null;
  }

}
