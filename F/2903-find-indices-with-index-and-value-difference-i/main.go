func findIndices(nums []int, indexDifference int, valueDifference int) []int {
	for i := indexDifference; i < len(nums); i++ {
		for j := 0; j <= i-indexDifference; j++ {
			if nums[i]-nums[j] >= valueDifference ||
				nums[i]-nums[j] <= -valueDifference ||
				nums[j]-nums[i] >= valueDifference ||
				nums[j]-nums[i] <= -valueDifference {
				return []int{i, j}
			}
		}
	}
	return []int{-1, -1}
}
