package main

import "fmt"

func videoStitching(clips [][]int, T int) int {
	M := map[int]int{}
	for _, v := range clips {
		s, e := v[0], v[1]
		c, ok := M[s]
		if !ok {
			M[s] = e
		} else if e > c {
			M[s] = e
		}
	}
	zeroIdx, ok := M[0]
	if !ok {
		return -1
	}
	buf := make([]int, T+1)
	for i := 0; i <= zeroIdx && i <= T; i++ {
		buf[i] = 1
	}
	for i := 1; i <= T; i++ {
		if buf[i] == 0 {
			return -1
		}
		c, ok := M[i]
		if !ok {
			continue
		}
		for j := i + 1; j <= c && j <= T; j++ {
			if buf[j] > buf[i]+1 || buf[j] == 0 {
				buf[j] = buf[i] + 1
			}
		}
	}
	if buf[T] == 0 {
		return -1
	}
	return buf[T]
}

func main() {
	fmt.Println("a")
}
