package main

import "fmt"

func findLength(A []int, B []int) int {
	buf := map[int][]int{}
	for i := 0; i < len(A); i++ {
		_, ok := buf[A[i]]
		if !ok {
			buf[A[i]] = []int{}
		}
		buf[A[i]] = append(buf[A[i]], i)
	}

	ret := 0
	pos := make([]int, 1000)
	for i := 0; i < len(B); i++ {
		p, ok := buf[B[i]]
		if !ok {
			for m := 0; m < len(pos); m++ {
				pos[m] = 0
			}
		} else {
			t := make([]int, 1000)
			for m := 0; m < len(t); m++ {
				t[m] = 0
			}
			for k := 0; k < len(p); k++ {
				o := p[k]
				if o > 0 {
					t[o] = pos[o-1] + 1
				} else {
					t[o] = 1
				}
				if t[o] > ret {
					ret = t[o]
				}
			}
			pos = t
		}
	}
	return ret
}

func main() {
	fmt.Println(findLength([]int{1, 2, 3, 2, 1}, []int{3, 2, 1, 4, 7}))

	// fmt.Println(findLength([]int{0, 1, 1, 1, 1}, []int{1, 0, 1, 0, 1}))

}
