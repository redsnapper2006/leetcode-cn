func occurrencesOfElement(nums []int, queries []int, x int) []int {
	pos := []int{}
	for i, v := range nums {
		if v == x {
			pos = append(pos, i)
		}
	}

	ans := []int{}
	for _, q := range queries {
		if q > len(pos) {
			ans = append(ans, -1)
		} else {
			ans = append(ans, pos[q-1])
		}
	}
	return ans
}
