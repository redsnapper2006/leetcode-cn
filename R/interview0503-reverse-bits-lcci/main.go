package main

import "fmt"

func reverseBits(num int) int {
	buf := make([]int, 32)
	for i := 31; i >= 0; i-- {
		if num&(1<<i) > 0 {
			buf[31-i] = 1
		} else {
			buf[31-i] = 0
		}
	}
	var arr []int
	s := 0
	for s < 32 {
		if buf[s] == 0 {
			arr = append(arr, 0)
			s++
		} else {
			c := 0
			for s < 32 && buf[s] == 1 {
				c++
				s++
			}
			arr = append(arr, c)

		}
	}
	max := -1
	for i := 0; i < len(arr); i++ {
		if arr[i] == 0 {
			left, right := 0, 0
			if i > 0 {
				left = arr[i-1]
			}
			if i < len(arr)-1 {
				right = arr[i+1]
			}
			if max < left+right+1 {
				max = left + right + 1
			}
		}
	}
	return max
}

func main() {
	fmt.Println("a")
}
