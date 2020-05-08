package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findFrequentTreeSum(root *TreeNode) []int {
	BUF := make(map[int]int)

	var recur func(r *TreeNode, b *map[int]int) int
	recur = func(r *TreeNode, b *map[int]int) int {
		if r == nil {
			return 0
		}

		sum := r.Val + recur(r.Left, b) + recur(r.Right, b)
		(*b)[sum]++
		return sum
	}
	recur(root, &BUF)
	max := -1 << 31
	for _, v := range BUF {
		if v > max {
			max = v
		}
	}
	var ret []int
	for k, v := range BUF {
		if v == max {
			ret = append(ret, k)
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
