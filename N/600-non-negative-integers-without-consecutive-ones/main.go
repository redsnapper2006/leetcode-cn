package main

import "fmt"

func findIntegers(num int) int {
	buf := make([]int, 32)
	buf[0] = 1
	buf[1] = 2
	for i := 2; i < 32; i++ {
		buf[i] = buf[i-1] + buf[i-2]
	}
	cnt := 0
	i := 31
	for i >= 0 {
		if num&(1<<i) > 0 {
			cnt += buf[i]
			j := i - 1
			if j >= 0 && num&(1<<j) > 0 {
				cnt += buf[j] - 1
				break
			}
		}
		i--
	}
	return cnt + 1
}

func main() {
	for i := 1; i <= 64; i = i * 2 {
		fmt.Println(i, findIntegers(i)-1)
	}
	fmt.Println(findIntegers(48))
	fmt.Println(findIntegers(56))
	fmt.Println(findIntegers(999999933))
}
