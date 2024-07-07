func maxSubarrayLength(nums []int, k int) int {
	m := map[int][]int{}
	ans := 0
	start := -1
	for i, n := range nums {
		m[n] = append(m[n], i)
		if len(m[n]) > k {
			if ans < i-1-start {
				ans = i - 1 - start
			}
			if start < m[n][len(m[n])-1-k] {
				start = m[n][len(m[n])-1-k]
			}
		}
	}
	if ans < len(nums)-start-1 {
		ans = len(nums) - start - 1
	}
	return ans
}
