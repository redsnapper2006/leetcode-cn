package main

import (
	"fmt"
	"strconv"
)

func calPoints(ops []string) int {
	var buf []int

	for i := 0; i < len(ops); i++ {
		if ops[i] == "C" {
			buf = buf[0 : len(buf)-1]
		} else if ops[i] == "D" {
			buf = append(buf, buf[len(buf)-1]*2)
		} else if ops[i] == "+" {
			buf = append(buf, buf[len(buf)-1]+buf[len(buf)-2])
		} else {
			i, _ := strconv.ParseInt(ops[i], 10, 32)
			buf = append(buf, int(i))
		}
	}
	sum := 0
	for i := 0; i < len(buf); i++ {
		sum += buf[i]
	}
	return sum
}

func main() {
	fmt.Println("a")
}
