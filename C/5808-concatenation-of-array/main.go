package main

func getConcatenation(nums []int) []int {
	size := len(nums)
	for i := 0; i < size; i++ {
		nums = append(nums, nums[i])
	}
	return nums
}

func main() {
	fmt.Println()
}
