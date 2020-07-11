package main

import "fmt"

func drawLine(length int, w int, x1 int, x2 int, y int) []int {
	base := w / 32 * y
	buf := make([]int32, length)
	for i := x1; i <= x2; i++ {
		idx := i / 32
		offset := i % 32
		buf[base+idx] |= 1 << (31 - offset)
	}
	var r []int
	for i := 0; i < length; i++ {
		r = append(r, int(buf[i]))
	}
	return r
}

func main() {
	fmt.Println(drawLine(15, 96, 81, 95, 1))
}
