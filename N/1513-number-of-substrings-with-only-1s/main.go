package main

import "fmt"

func numSub(s string) int {
	cnt := 0
	var ret []int
	for _, b := range s {
		if b == '1' {
			cnt++
		} else if cnt > 0 {
			ret = append(ret, cnt)
			cnt = 0
		}
	}
	if cnt > 0 {
		ret = append(ret, cnt)
		cnt = 0
	}
	sum := 0
	for _, c := range ret {
		sum += c * (c + 1) / 2
		sum %= 1000000007
	}
	return sum
}

func main() {
	fmt.Println()
}
