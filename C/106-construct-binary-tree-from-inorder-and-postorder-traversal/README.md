# No106

https://leetcode-cn.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/

# Logic
1. 首先根据后序遍历序列的最后一个数字创建根结点（后序遍历序列的最后一个数字就是根结点）
2. 然后在中序遍历序列中找到根结点所在的位置，这样就能确定左、右子树结点的数量，这样也就分别找到了左、右子树的中序遍历序列和后序遍历序列。 
3. 然后用递归的方法去构建左、右子树，直到叶子结点。

