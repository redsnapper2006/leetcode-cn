class TreeNode {
  int val;
  TreeNode left;
  TreeNode right;

  TreeNode(int x) {
    val = x;
  }
}

public class Solution {
  public TreeNode buildTree(int[] preorder, int[] inorder) {
    if (inorder.length == 0) {
      return null;
    }
    if (inorder.length == 1) {
      return new TreeNode(inorder[0]);
    }

    int rootVal = preorder[0];
    int rootInOrderIndex = -1;
    for (int i = 0; i < inorder.length; i++) {
      if (inorder[i] == rootVal) {
        rootInOrderIndex = i;
        break;
      }
    }

    int[] inorderLeft = new int[rootInOrderIndex];
    System.arraycopy(inorder, 0, inorderLeft, 0, rootInOrderIndex);
    int[] inorderRight = new int[inorder.length - 1 - rootInOrderIndex];
    System.arraycopy(inorder, rootInOrderIndex + 1, inorderRight, 0,
                     inorder.length - 1 - rootInOrderIndex);
    int[] preorderLeft = new int[rootInOrderIndex];
    System.arraycopy(preorder, 1, preorderLeft, 0, rootInOrderIndex);
    int[] preorderRight = new int[inorder.length - 1 - rootInOrderIndex];
    System.arraycopy(preorder, rootInOrderIndex + 1, preorderRight, 0,
                     inorder.length - 1 - rootInOrderIndex);

    TreeNode root = new TreeNode(rootVal);
    root.left = buildTree(preorderLeft, inorderLeft);
    root.right = buildTree(preorderRight, inorderRight);
    return root;
  }
}
