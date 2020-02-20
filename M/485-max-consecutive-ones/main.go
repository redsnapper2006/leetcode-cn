package main

func findMaxConsecutiveOnes(nums []int) int {
	c := 0
	max := 0
	for i := 0; i < len(nums); i++ {
		if nums[i] == 1 {
			c++
		} else {
			if c > max {
				max = c
			}
			c = 0
		}
	}
	if c > max {
		max = c
	}
	return max
}

func main() {

}
