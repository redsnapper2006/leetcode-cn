package main

import "fmt"

func getMaxLen(nums []int) int {
	plusIdx, minusIdx := -1, -1
	flag := 1
	max := 0
	for i, v := range nums {
		if v > 0 {
			if plusIdx == -1 {
				plusIdx = i
			}
			if flag > 0 {
				t := plusIdx
				if t > minusIdx && minusIdx > -1 {
					t = minusIdx
				}
				if i-t+1 > max {
					max = i - t + 1
				}
			} else {
				if i-minusIdx > max {
					max = i - minusIdx
				}
			}
		} else if v < 0 {
			flag *= -1
			if minusIdx == -1 {
				minusIdx = i
			}
			if flag > 0 {
				t := minusIdx
				if t > plusIdx && plusIdx > -1 {
					t = plusIdx
				}
				if i-t+1 > max {
					max = i - t + 1
				}
			} else if flag < 0 {
				if i-minusIdx > max {
					max = i - minusIdx
				}
			}
		} else {
			minusIdx = -1
			plusIdx = -1
			flag = 1
		}
	}

	return max
}

func main() {
	// fmt.Println(getMaxLen([]int{-16, 0, -5, 2, 2, -13, 11, 8}))
	// fmt.Println(getMaxLen([]int{1, 2, 3, 5, -6, 4, 0, 10}))
	fmt.Println(getMaxLen([]int{-1, -2, -3, 0, 1}))

}
