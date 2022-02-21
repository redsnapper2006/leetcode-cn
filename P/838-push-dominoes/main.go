package main

import (
	"fmt"
)

func pushDominoes(dominoes string) string {
	buf := make([]byte, len(dominoes))
	copy(buf, []byte(dominoes))

	candi := [][]int{}
	for i := 0; i < len(dominoes); i++ {
		if dominoes[i] == byte('R') {
			candi = append(candi, []int{i, 2})
		} else if dominoes[i] == byte('L') {
			candi = append(candi, []int{i, 1})
		}
	}

	for len(candi) > 0 {
		t := [][]int{}
		m := map[int]int{}
		for _, v := range candi {
			idx, direction := v[0], v[1]
			if direction == 2 && idx < len(dominoes)-1 {
				if buf[idx+1] == byte('.') {
					if m[idx+1] == 0 {
						m[idx+1] = 2
					} else {
						delete(m, idx+1)
					}
				}
			} else if direction == 1 && idx > 0 {
				if buf[idx-1] == byte('.') {
					if m[idx-1] == 0 {
						m[idx-1] = 1
					} else {
						delete(m, idx-1)
					}
				}
			}
		}
		for k, v := range m {
			if v == 1 {
				buf[k] = byte('L')
			} else {
				buf[k] = byte('R')
			}
			t = append(t, []int{k, v})
		}
		candi = t
	}

	return string(buf)
}

func main() {
	fmt.Println()
}
