package main

import "fmt"

func maxCompatibilitySum(students [][]int, mentors [][]int) int {
	sb := make([]int, len(students))
	mb := make([]int, len(mentors))

	for i := 0; i < len(students); i++ {
		s := 0
		m := 0
		for j := 0; j < len(students[i]); j++ {
			s = s*2 + students[i][j]
			m = m*2 + mentors[i][j]
		}
		sb[i] = s
		mb[i] = m
	}
	x := 0
	for i := 0; i < len(students[0]); i++ {
		x = x*2 + 1
	}
	// fmt.Println(sb, mb)
	buf := make([][]int, len(students))
	for i := 0; i < len(students); i++ {
		buf[i] = make([]int, len(students))
		for j := 0; j < len(students); j++ {
			c := (sb[i] ^ mb[j]) ^ x
			cnt := 0
			for c > 0 {
				if c&1 > 0 {
					cnt++
				}
				c = c >> 1
			}
			buf[i][j] = cnt
		}
	}
	// fmt.Println("buf", buf)
	N := len(students)
	max := -1
	var dfs func(buf [][]int, row int, sum int, occu map[int]int)

	dfs = func(buf [][]int, row int, sum int, occu map[int]int) {
		if row == N {
			if max < sum {
				max = sum
			}
			return
		}
		for i := 0; i < N; i++ {
			_, ok := occu[i]
			if ok {
				continue
			}
			occu[i]++

			dfs(buf, row+1, sum+buf[row][i], occu)
			delete(occu, i)
		}
	}
	occu := map[int]int{}
	dfs(buf, 0, 0, occu)
	return max
}

func main() {
	fmt.Println(maxCompatibilitySum([][]int{{0, 1, 0, 1, 1, 1}, {1, 0, 0, 1, 0, 1}, {1, 0, 1, 1, 0, 0}},
		[][]int{{1, 0, 0, 0, 0, 1}, {0, 1, 0, 0, 1, 1}, {0, 1, 0, 0, 1, 1}}))
	fmt.Println(maxCompatibilitySum([][]int{{1, 1, 0}, {1, 0, 1}, {0, 0, 1}},
		[][]int{{1, 0, 0}, {0, 0, 1}, {1, 1, 0}}))
}
