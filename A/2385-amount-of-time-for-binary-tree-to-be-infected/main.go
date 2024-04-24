func amountOfTime(root *TreeNode, start int) int {
	m := map[int][]int{}
	var dfs func(root *TreeNode, m *map[int][]int)
	dfs = func(root *TreeNode, m *map[int][]int) {

		v := root.Val
		if _, ok := (*m)[v]; !ok {
			(*m)[v] = []int{}
		}

		if root.Left != nil {
			l := root.Left.Val
			(*m)[v] = append((*m)[v], l)
			if _, ok := (*m)[l]; !ok {
				(*m)[l] = []int{}
			}
			(*m)[l] = append((*m)[l], v)

			dfs(root.Left, m)
		}

		if root.Right != nil {
			r := root.Right.Val
			(*m)[v] = append((*m)[v], r)
			if _, ok := (*m)[r]; !ok {
				(*m)[r] = []int{}
			}
			(*m)[r] = append((*m)[r], v)

			dfs(root.Right, m)
		}
	}

	dfs(root, &m)

	max := 0
	q := [][]int{{start, 0}}
	visited := map[int]bool{}
	visited[start] = true
	for len(q) > 0 {
		next := q[0]
		q = q[1:]

		if max < next[1] {
			max = next[1]
		}
		v := next[0]
		for _, n := range m[v] {
			if _, ok := visited[n]; !ok {
				q = append(q, []int{n, next[1] + 1})
				visited[n] = true
			}
		}
	}

	return max
}
