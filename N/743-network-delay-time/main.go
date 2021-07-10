package main

import (
	"fmt"
)

func networkDelayTime(times [][]int, N int, K int) int {
	buf := make(map[int]int)

	buf[K] = 0
	for {
		isReCalc := false
		for i := 0; i < len(times); i++ {
			u, v, w := times[i][0], times[i][1], times[i][2]
			t, ok := buf[u]
			if ok {
				c, ok2 := buf[v]
				if !ok2 || c > t+w {
					buf[v] = t + w
					isReCalc = true
				}
			}
		}
		if !isReCalc {
			break
		}
	}
	if len(buf) < N {
		return -1
	}
	max := 0
	for _, v := range buf {
		if max < v {
			max = v
		}
	}
	return max
}

func main() {
	fmt.Println("a")
}
