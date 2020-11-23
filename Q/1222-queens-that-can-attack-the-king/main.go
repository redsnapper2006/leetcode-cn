package main

import "fmt"

func queensAttacktheKing(queens [][]int, king []int) [][]int {
	M := map[string]int{}
	for _, v := range queens {
		M[fmt.Sprintf("%d_%d", v[0], v[1])]++
	}

	var ret [][]int
	offset := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}, {-1, -1}, {-1, 1}, {1, -1}, {1, 1}}
	for _, off := range offset {
		r, c := king[0], king[1]
		r += off[0]
		c += off[1]
		for r >= 0 && r < 8 && c >= 0 && c < 8 {
			_, ok := M[fmt.Sprintf("%d_%d", r, c)]
			if ok {
				ret = append(ret, []int{r, c})
				break
			}
			r += off[0]
			c += off[1]
		}
	}

	return ret
}

func main() {
	fmt.Println()
}
