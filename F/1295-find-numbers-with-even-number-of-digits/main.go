package main

func findNumbers(nums []int) int {
	c := 0
	for i := 0; i < len(nums); i++ {
		b := nums[i]

		if (b == 100000) || (b < 10000 && b >= 1000) || (b < 100 && b >= 10) {
			c++
		}
	}
	return c
}

func main() {
	fmt.Println("a")
}
