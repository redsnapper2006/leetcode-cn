package main

import "fmt"

func findClosedNumbers(num int) []int {
	var buf []int
	for num > 0 {
		buf = append(buf, num%2)
		num /= 2
	}
	max, min := make([]int, len(buf)), make([]int, len(buf))
	copy(max, buf)
	copy(min, buf)

	idx := -1
	for i := 0; i < len(max); i++ {
		if max[i] == 0 {
			idx = i
			break
		}
	}
	if idx == -1 {
		max[len(max)-1] = 0
		max = append(max, 1)
	} else if idx == 0 {
		one := idx
		for i := idx; i < len(max); i++ {
			if buf[i] == 1 {
				one = i
				break
			}
		}
		zero := one
		for i := zero; i < len(max); i++ {
			if buf[i] == 0 {
				zero = i
				break
			}
		}
		if zero == one {
			for i := 0; i < one; i++ {
				max[len(max)-1-i] = 0
			}
			for i := 0; i < len(max)-one-1; i++ {
				max[len(max)-1-one-i] = 1
			}
			max = append(max, 1)
		} else {
			for i := 0; i <= one; i++ {
				max[zero-1-i] = 0
			}
			for i := 0; i < zero-one-1; i++ {
				max[i] = 1
			}
			max[zero] = 1
		}
	} else {
		max[idx] = 1
		max[idx-1] = 0
	}

	retMax := 0
	for i := len(max) - 1; i >= 0; i-- {
		retMax = retMax*2 + max[i]
	}
	idx = -1
	for i := 0; i < len(min); i++ {
		if min[i] == 0 {
			idx = i
			break
		}
	}
	retMin := 0
	if idx == -1 {
		retMin = -1
	} else {
		one := -1
		for i := idx; i < len(min); i++ {
			if min[i] == 1 {
				one = i
				break
			}
		}
		min[one] = 0
		for i := 0; i <= idx; i++ {
			min[one-1-i] = 1
		}
		for i := 0; i < one-idx-1; i++ {
			min[i] = 0
		}
		for i := len(min) - 1; i >= 0; i-- {
			retMin = retMin*2 + min[i]
		}
	}
	return []int{retMax, retMin}
}

func main() {
	fmt.Println()
}
