package main

func singleNumber(nums []int) int {
	accum := 0
	for _, c := range nums {
		accum ^= c
	}
	return accum
}

func main() {
	singleNumber([]int{3, 4, 2, 3, 4, 2, 101})
}
