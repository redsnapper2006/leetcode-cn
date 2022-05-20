package main

func findDifference(nums1 []int, nums2 []int) [][]int {
	m1, m2 := map[int]int{}, map[int]int{}
	for _, v := range nums1 {
		m1[v] = 1
	}
	for _, v := range nums2 {
		m2[v] = 1
	}

	r1 := []int{}
	o1 := map[int]int{}
	for _, v := range nums1 {
		if _, ok := m2[v]; !ok {
			o1[v] = 1
		}
	}
	for k := range o1 {
		r1 = append(r1, k)
	}
	r2 := []int{}
	o2 := map[int]int{}
	for _, v := range nums2 {
		if _, ok := m1[v]; !ok {
			o2[v] = 1
		}
	}
	for k := range o1 {
		r2 = append(r2, k)
	}

	return [][]int{r1, r2}
}
