package main

import "fmt"

func getLucky(s string, k int) int {
	buf := []int{}
	for _, b := range s {
		buf = append(buf, int(byte(b)-'a')+1)
	}
	sum := 0
	for i := 0; i < len(buf); i++ {
		c := buf[i]
		for c > 0 {
			sum += c % 10
			c /= 10
		}
	}

	for i := 1; i < k; i++ {
		temp := sum
		t := 0
		for temp > 0 {
			t += temp % 10
			temp /= 10
		}
		sum = t
	}
	return sum
}

func main() {
	fmt.Println()
}
