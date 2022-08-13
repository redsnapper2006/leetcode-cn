package main

func getAverages(nums []int, k int) []int {

	sum := 0
	sums := make([]int, len(nums))
	for i := 0; i < len(nums); i++ {
		sum += nums[i]
		sums[i] = sum
	}
	ret := []int{}
	for i := 0; i < len(nums); i++ {
		if i < k || i >= len(nums)-k {
			ret = append(ret, -1)
			continue
		}
		v := sums[i+k]
		if i > k {
			v -= sums[i-k-1]
		}
		ret = append(ret, v/(2*k+1))
	}
	return ret
}
