package main

import "fmt"

func paintingPlan(n int, k int) int {
	if n*n == k {
		return 1
	}
	remain := n*n - k
	sum := 0
	for i := 1; i <= remain; i++ {
		if remain%i != 0 {
			continue
		}

		d := remain / i
		if d > n {
			continue
		}
		tt := 1
		for j := n; j > n-i; j-- {
			tt *= j
		}
		bt := 1
		for j := i; j > 1; j-- {
			bt *= j
		}

		tb := 1
		for j := n; j > n-d; j-- {
			tb *= j
		}
		bb := 1
		for j := d; j > 1; j-- {
			bb *= j
		}
		sum += (tt / bt) * (tb / bb)
	}
	return sum
}

func main() {
	fmt.Println(paintingPlan(3, 7))
}
