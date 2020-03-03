class TreeNode {
  int val;
  TreeNode left;
  TreeNode right;

  TreeNode(int x) {
    val = x;
  }
}

public class Solution {
  public TreeNode buildTree(int[] inorder, int[] postorder) {
    if (inorder.length == 0) {
      return null;
    }
    if (inorder.length == 1) {
      return new TreeNode(inorder[0]);
    }

    int rootIndex = postorder.length - 1;
    int rootVal = postorder[rootIndex];
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
    int[] postorderLeft = new int[rootInOrderIndex];
    System.arraycopy(postorder, 0, postorderLeft, 0, rootInOrderIndex);
    int[] postorderRight = new int[inorder.length - 1 - rootInOrderIndex];
    System.arraycopy(postorder, rootInOrderIndex, postorderRight, 0,
                     inorder.length - 1 - rootInOrderIndex);

    TreeNode root = new TreeNode(rootVal);
    root.left = buildTree(inorderLeft, postorderLeft);
    root.right = buildTree(inorderRight, postorderRight);
    return root;
  }
}
