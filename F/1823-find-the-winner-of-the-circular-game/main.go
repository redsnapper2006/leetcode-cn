package main

import "fmt"

func findTheWinner(n int, k int) int {
	buf := make([]int, n)
	start := 0
	cnt := n
	for {
		fmt.Println(buf, start, cnt)
		if cnt == 1 {
			break
		}
		steps := k
		for steps > 0 {
			if buf[start] == 1 {
			} else {
				steps--
			}
			start++
			start %= n
		}
		idx := (start - 1 + n) % n
		buf[idx] = 1
		// start++
		// start %= n
		cnt--
	}
	fmt.Println("---", buf, start, cnt)
	for i := 0; i < n; i++ {
		if buf[i] == 0 {
			return i + 1
		}
	}
	return -1
}

func main() {
	fmt.Println(findTheWinner(5, 3))
}
