package main

import "fmt"

func combine(n int, k int) [][]int {
	buf := [][]int{}
	for i := 1; i <= n; i++ {
		size := len(buf)
		for j := 0; j < size; j++ {
			if len(buf[j]) >= k {
				continue
			}
			t := make([]int, len(buf[j]))
			copy(t, buf[j])
			t = append(t, i)
			buf = append(buf, t)
		}
	}
	var ret [][]int
	for i := 0; i < len(buf); i++ {
		if len(buf[i]) == k {
			ret = append(ret, buf[i])
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
