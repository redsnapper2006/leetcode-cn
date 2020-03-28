package main

import "fmt"

func trap(height []int) int {
	sIdx := -1
	for i := 0; i < len(height)-1; i++ {
		if height[i] > height[i+1] {
			sIdx = i
			break
		}
	}
	if sIdx == -1 || sIdx == len(height)-2 {
		return 0
	}

	sum := 0
	var stack []int
	var offset []int
	for i := sIdx; i < len(height)-1; i++ {
		stack = append(stack, height[i])
		offset = append(offset, i)
		if height[i] < height[i+1] {
			level := 0
			off := -1
			popIdx := -1
			for m := len(stack) - 1; m >= 0; m-- {
				if stack[m] > height[i+1] {
					popIdx = m
					level = height[i+1]
					off = offset[m]
					break
				}
			}
			if popIdx == -1 {
				level = stack[0]
				off = 0
			}
			for m := popIdx + 1; m < len(stack); m++ {
				b := stack[m]
				o := offset[m]
				sum += (level - b) * (i - off)
				level = b
				off = o
			}
			stack = stack[0 : popIdx+1]
			offset = offset[0 : popIdx+1]
		}
	}
	return sum
}

func main() {
	fmt.Println(trap([]int{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1}))
	fmt.Println(trap([]int{8, 2, 8, 9, 0, 1, 7, 7, 9}))
}
