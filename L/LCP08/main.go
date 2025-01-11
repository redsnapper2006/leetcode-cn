package main

import "fmt"

func getTriggerTime(increase [][]int, requirements [][]int) []int {
	var C, R, H []int
	day := 1
	for i := 0; i < len(increase); i++ {
		c := increase[i][0]
		for i := 0; i < c; i++ {
			C = append(C, day)
		}
		r := increase[i][1]
		for i := 0; i < r; i++ {
			R = append(R, day)
		}
		h := increase[i][2]
		for i := 0; i < h; i++ {
			H = append(H, day)
		}
		day++
	}

	var ret []int
	for i := 0; i < len(requirements); i++ {
		c := requirements[i][0]
		cday := -1
		if c == 0 {
			cday = 0
		} else if c <= len(C) {
			cday = C[c-1]
		}
		r := requirements[i][1]
		rday := -1
		if r == 0 {
			rday = 0
		} else if r <= len(R) {
			rday = R[r-1]
		}
		h := requirements[i][2]
		hday := -1
		if h == 0 {
			hday = 0
		} else if h <= len(H) {
			hday = H[h-1]
		}
		if cday == -1 || rday == -1 || hday == -1 {
			ret = append(ret, -1)
		} else {
			v := cday
			if v < rday {
				v = rday
			}
			if v < hday {
				v = hday
			}
			ret = append(ret, v)
		}
	}
	return ret
}

func main() {
	fmt.Println(getTriggerTime([][]int{[]int{2, 8, 4}, []int{2, 5, 0}, []int{10, 9, 8}},
		[][]int{[]int{2, 11, 3}, []int{15, 10, 7}, []int{9, 17, 12}, []int{8, 1, 14}}))
}
