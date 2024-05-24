
func mostCompetitive(nums []int, k int) []int {
	stack := []int{}
	for i, v := range nums {

		for len(stack) > (k-len(nums)+i) && len(stack) > 0 && stack[len(stack)-1] > v {
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, v)
	}
	return stack[:k]
}
