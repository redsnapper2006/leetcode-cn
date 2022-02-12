package main

import "sort"

func minimumSum(num int) int {
	buf := []int{}

	for i := 0; i < 4; i++ {
		buf = append(buf, num%10)
		num /= 10
	}

	sort.Ints(buf)
	ten := buf[0] + buf[1]
	one := buf[2] + buf[3]
	if one > 9 {
		ten++
		one -= 10
	}
	hunderd := 0
	if ten > 9 {
		hunderd = 1
		ten -= 10
	}
	return hunderd*100 + ten*10 + one
}
