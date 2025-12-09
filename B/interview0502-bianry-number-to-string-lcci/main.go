package main

import (
	"fmt"
	"strconv"
)

func printBin(num float64) string {
	if num < 0 || num > 1 {
		return "ERROR"
	}
	if num == 0 {
		return "0"
	}
	if num == 1 {
		return "ERROR"
	}

	times := 0
	val := 0
	for i := 0; i <= 32; i++ {
		num *= 2
		// fmt.Println(num, int(num), i)
		if num-float64(int(num)) == 0 {
			times = i + 1
			val = int(num)
			break
		}
	}
	if val == 0 {
		return "ERROR"
	}
	s := strconv.FormatInt(int64(val), 2)
	for len(s) < times {
		s = "0" + s
	}
	return "0." + s
}

func main() {
	fmt.Println(printBin(0.2868311060592532))
}
