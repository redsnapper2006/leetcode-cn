package main

import (
	"fmt"
)

func numRescueBoats(people []int, limit int) int {
	buf := make([]int, 30001)
	for _, v := range people {
		buf[v]++
	}
	ret := 0
	for i := len(buf) - 1; i >= 0; i-- {
		if buf[i] == 0 {
			continue
		}

		diff := limit - i
		if diff > i {
			diff = i
		}
		for j := diff; j >= 0 && buf[i] > 0; j-- {
			if buf[j] == 0 {
				continue
			}
			if i == j {
				d := buf[i] / 2
				ret += d
				buf[i] -= d * 2
				continue
			}
			min := buf[i]
			if min > buf[j] {
				min = buf[j]
			}
			buf[i] -= min
			buf[j] -= min
			ret += min
		}
		if buf[i] > 0 {
			ret += buf[i]
			buf[i] -= buf[i]
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
