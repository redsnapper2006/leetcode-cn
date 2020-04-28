package main

func singleNumbers(nums []int) []int {
	sum := 0
	for i := 0; i < len(nums); i++ {
		sum ^= nums[i]
	}

	flag := (-sum) & sum
	a, b := 0, 0
	for i := 0; i < len(nums); i++ {
		if flag&nums[i] == 0 {
			a ^= nums[i]
		} else {
			b ^= nums[i]
		}
	}
	return []int{a, b}
}

func singleNumbersV2(nums []int) []int {
	buf := make([]int, 10001)
	for i := 0; i < len(nums); i++ {
		buf[nums[i]]++
	}

	var r []int
	for i := 2; i <= 10000; i++ {
		if buf[i] == 1 {
			r = append(r, i)
		}
	}
	return r
}

func main() {
	fmt.Println("a")
}
