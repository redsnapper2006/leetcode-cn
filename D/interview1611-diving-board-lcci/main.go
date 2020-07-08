package main

import (
	"fmt"
)

func divingBoard(shorter int, longer int, k int) []int {
	if k == 0 {
		return nil
	}
	var buf []int
	for i := 0; i <= k; i++ {
		buf = append(buf, shorter*(k-i)+longer*i)
	}
	ret := []int{buf[0]}
	for i := 1; i < len(buf); i++ {
		if buf[i] == buf[i-1] {
			continue
		}
		ret = append(ret, buf[i])
	}
	return ret
}

func main() {
	fmt.Println("a")
}
