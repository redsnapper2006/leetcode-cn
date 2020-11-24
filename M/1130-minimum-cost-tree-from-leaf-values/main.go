package main

import (
	"fmt"
)

func mctFromLeafValues(arr []int) int {
	sum := 0
	stack := []int{1<<31 - 1}
	for _, v := range arr {
		for v >= stack[len(stack)-1] {
			n := stack[len(stack)-1]
			c := stack[len(stack)-2]
			if c > v {
				c = v
			}
			sum += c * n
			stack = stack[0 : len(stack)-1]
		}
		stack = append(stack, v)
	}
	for i := len(stack) - 1; i > 1; i-- {
		sum += stack[i] * stack[i-1]
	}
	return sum
}

func mctFromLeafValuesV2(arr []int) int {
	var recur func(arr []int) int
	M := map[string]int{}

	recur = func(arr []int) int {
		if len(arr) == 1 {
			return 0
		}
		k := fmt.Sprint(arr)
		v, ok := M[k]
		if ok {
			return v
		}
		if len(arr) == 2 {
			M[k] = arr[0] * arr[1]
			return arr[0] * arr[1]
		}

		min := 1<<31 - 1
		for i := 1; i < len(arr); i++ {
			left, right := 0, 0
			for j := 0; j < i; j++ {
				if arr[j] > left {
					left = arr[j]
				}
			}
			for j := i; j < len(arr); j++ {
				if arr[j] > right {
					right = arr[j]
				}
			}
			lb := recur(arr[0:i])
			rb := recur(arr[i:])

			if left*right+lb+rb < min {
				min = left*right + lb + rb
			}
			// fmt.Println(lb,rb,left*right,min )
		}
		M[k] = min
		return min
	}

	return recur(arr)
}

func main() {
	fmt.Println(mctFromLeafValues([]int{10, 14, 7, 10, 6, 14, 4, 14, 4, 4, 4, 15, 7, 4, 9}))
}
