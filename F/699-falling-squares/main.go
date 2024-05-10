package main

type Node struct {
	S int
	E int
	H int
	L *Node
	R *Node
}

func fallingSquares(positions [][]int) []int {
	var query func(root *Node, start int, end int, h int) int
	query = func(root *Node, start int, end int, h int) int {
		if root == nil {
			return h
		}

		if root.S >= end {
			return query(root.L, start, end, h)
		} else if root.E >= end {
			if root.S > start {
				l := query(root.L, start, root.S, h)
				if l < root.H+h {
					l = root.H + h
				}
				return l
			} else {
				return root.H + h
			}
		} else {
			if root.E > start {
				r := query(root.R, root.E, end, h)
				if r < root.H+h {
					r = root.H + h
				}
				if start < root.S {
					l := query(root.L, start, root.S, h)
					if l > r {
						r = l
					}
				}
				return r
			} else {
				return query(root.R, start, end, h)
			}
		}
	}

	var set func(root *Node, start int, end int, h int)
	set = func(root *Node, start int, end int, h int) {
		if root.S >= end {
			if root.L == nil {
				root.L = &Node{start, end, h, nil, nil}
			} else {
				set(root.L, start, end, h)
			}
		} else if root.E >= end {
			if root.S > start {
				if root.L == nil {
					root.L = &Node{start, root.S, h, nil, nil}
				} else {
					set(root.L, start, root.S, h)
				}
			} else if root.S < start {
				newL := &Node{root.S, start, root.H, root.L, nil}
				root.L = newL
				root.S = start
			}
			if root.E > end {
				newR := &Node{end, root.E, root.H, nil, root.R}
				root.R = newR
				root.E = end
			}
			root.H = h
		} else {
			if root.S > start {
				if root.L == nil {
					root.L = &Node{start, root.S, h, nil, nil}
				} else {
					set(root.L, start, root.S, h)
				}
			}
			if root.E > start {
				if root.S < start {
					newL := &Node{root.S, start, root.H, root.L, nil}
					root.L = newL
					root.S = start
				}
				root.H = h
				if root.R == nil {
					root.R = &Node{root.E, end, h, nil, nil}
				} else {
					set(root.R, root.E, end, h)
				}
			} else {
				if root.R == nil {
					root.R = &Node{start, end, h, nil, nil}
				} else {
					set(root.R, start, end, h)
				}
			}
		}
	}

	root := &Node{positions[0][0], positions[0][0] + positions[0][1], positions[0][1], nil, nil}

	ans := make([]int, len(positions))
	ans[0] = positions[0][1]
	idx := 1
	for idx < len(positions) {
		h := query(root, positions[idx][0], positions[idx][0]+positions[idx][1], positions[idx][1])
		set(root, positions[idx][0], positions[idx][0]+positions[idx][1], h)
		if h > ans[idx-1] {
			ans[idx] = h
		} else {
			ans[idx] = ans[idx-1]
		}

		idx++
	}

	return ans
}
