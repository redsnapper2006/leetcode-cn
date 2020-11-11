package main

import "fmt"

func processQueries(queries []int, m int) []int {
	buf := make([]int, m)
	for i := 0; i < m; i++ {
		buf[i] = i + 1
	}

	var ret []int
	for _, v := range queries {
		idx := -1
		for i, c := range buf {
			if c == v {
				idx = i
				break
			}
		}
		copy(buf[1:], buf[0:idx])
		buf[0] = v
		ret = append(ret, idx)
	}
	return ret
}

func main() {
	fmt.Println()
}
