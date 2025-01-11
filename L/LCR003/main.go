package main

import "fmt"

func countBits(num int) []int {
	buf := []int{0, 1}
	for len(buf) <= num {
		size := len(buf)
		for i := 0; i < size; i++ {
			buf = append(buf, buf[i]+1)
		}
	}

	return buf[0 : num+1]
}

func main() {
	fmt.Println()
}
