func isPossibleToSplit(nums []int) bool {
	buf := make([]int, 101)
	for _, v := range nums {
		buf[v]++
		if buf[v] > 2 {
			return false
		}
	}
	return true
}
