package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func levelOrder(root *TreeNode) [][]int {
	if root == nil {
		return nil
	}

	var b [][]int
	b = append(b, []int{root.Val})

	bb := []*TreeNode{root}

	level := 0
	for len(bb) > 0 {
		level++
		level %= 2
		var tt []*TreeNode
		var t []int
		for i := 0; i < len(bb); i++ {
			for j := 0; j < 2; j++ {
				var p *TreeNode
				if level != j {
					p = bb[i].Right
				} else {
					p = bb[i].Left
				}
				if p != nil {
					tt = append(tt, p)
					t = append(t, p.Val)
				}
			}
		}
		s, e := 0, len(tt)-1
		for s < e {
			tt[s], tt[e] = tt[e], tt[s]
			s++
			e--
		}
		bb = tt
		if len(t) > 0 {
			b = append(b, t)
		}
	}
	return b
}

func main() {
	fmt.Println("a")
}
