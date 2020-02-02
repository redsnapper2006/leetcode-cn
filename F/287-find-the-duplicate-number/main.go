package main

import (
	"fmt"
)

func findDuplicate(nums []int) int {
	tortoise, rabbit := nums[0], nums[0]
	for {
		tortoise = nums[tortoise]
		rabbit = nums[nums[rabbit]]
		if tortoise == rabbit {
			break
		}
	}
	p1, p2 := nums[0], tortoise
	for p1 != p2 {
		p1 = nums[p1]
		p2 = nums[p2]
	}

	return p1
}

func main() {
	fmt.Println(findDuplicate([]int{75, 75, 75, 75, 75, 91, 75, 75, 75, 75, 75, 75, 30, 75, 75, 78, 75, 75, 75, 75, 75, 7, 79, 93, 75, 75, 15, 75, 75, 75, 75, 75, 75, 4, 75, 75, 21, 75, 75, 19, 75, 59, 75, 76, 75, 75, 75, 75, 75, 75, 75, 33, 75, 75, 75, 58, 75, 75, 5, 75, 97, 81, 48, 42, 75, 87, 75, 75, 25, 27, 94, 20, 75, 75, 75, 29, 75, 75, 86, 67, 75, 75, 75, 65, 75, 75, 75, 75, 75, 39, 75, 56, 75, 75, 75, 75, 3, 75, 75, 75}))
}
