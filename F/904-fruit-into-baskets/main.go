package main

import "fmt"

func totalFruit(tree []int) int {
	if len(tree) <= 1 {
		return len(tree)
	}

	b := tree[0]
	c := 1
	var treeStack []int
	var countStack []int
	for i := 1; i < len(tree); i++ {
		if tree[i] == b {
			c++
		} else {
			treeStack = append(treeStack, b)
			countStack = append(countStack, c)
			b = tree[i]
			c = 1
		}
	}
	treeStack = append(treeStack, b)
	countStack = append(countStack, c)

	M2 := make(map[int]int)
	max := 0
	start, end := -1, 0
	for i := 0; i < len(treeStack); i++ {
		_, ok := M2[treeStack[i]]
		if !ok && len(M2) == 2 {
			c := 0
			for i := start + 1; i <= end; i++ {
				c += countStack[i]
			}

			if max < c {
				max = c
			}
			delete(M2, treeStack[i-2])
			start = i - 2
		}
		M2[treeStack[i]] = 1
		end = i
	}
	c = 0
	for i := start + 1; i <= end; i++ {
		c += countStack[i]
	}

	if max < c {
		max = c
	}
	return max
}

func main() {
	// fmt.Println(totalFruit([]int{5, 0, 0, 7, 0, 7, 2, 7}))

	// fmt.Println(totalFruit([]int{1, 0, 1, 4, 1, 4, 1, 2, 3}))

	fmt.Println(totalFruit([]int{0, 1, 2, 2}))

}
