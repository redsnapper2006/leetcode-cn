func numberOfPairs(nums1 []int, nums2 []int, k int) int {
	for i := range nums2 {
		nums2[i] = nums2[i] * k
	}

	ans := 0
	for _, s1v := range nums1 {
		for ,s2v := range nums2 {
			if s1v %s2v == 0 {
				ans++
			}
		}
	}
	return ans
}
