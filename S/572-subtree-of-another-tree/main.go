package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isMatch(s *TreeNode, t *TreeNode) bool {
	if s == nil && t == nil {
		return true
	}
	if (s == nil && t != nil) || (s != nil && t == nil) {
		return false
	}
	if s.Val == t.Val {
		return isMatch(s.Left, t.Left) && isMatch(s.Right, t.Right)
	}
	return false
}

func isSubtree(s *TreeNode, t *TreeNode) bool {
	if s == nil && t == nil {
		return true
	}
	if (s == nil && t != nil) || (s != nil && t == nil) {
		return false
	}

	if isMatch(s, t) {
		return true
	} else {
		return isSubtree(s.Left, t) || isSubtree(s.Right, t)
	}
}

func main() {
	fmt.Println("a")
}
