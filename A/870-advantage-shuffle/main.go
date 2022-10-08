package main

import "sort"

func advantageCount(nums1 []int, nums2 []int) []int {
	sort.Ints(nums1)
	buf := make([]int, len(nums2))
	copy(buf, nums2)
	sort.Ints(nums2)

	m := map[int][]int{}
	free := []int{}

	idx1, idx2 := 0, 0
	for idx1 < len(nums1) && idx2 < len(nums2) {
		if nums1[idx1] <= nums2[idx2] {
			free = append(free, nums1[idx1])
			idx1++
			continue
		}
		_, ok := m[nums2[idx2]]
		if !ok {
			m[nums2[idx2]] = []int{}
		}
		m[nums2[idx2]] = append(m[nums2[idx2]], nums1[idx1])
		idx1++
		idx2++
	}
	// fmt.Println(m)
	freeIdx1 := 0
	ret := make([]int, len(nums2))
	for i := 0; i < len(buf); i++ {
		v, ok := m[buf[i]]
		if ok && len(v) > 0 {
			// fmt.Println(buf[i], v)
			ret[i] = v[0]
			m[buf[i]] = v[1:]
			continue
		}
		ret[i] = free[freeIdx1]
		freeIdx1++
	}
	return ret
}
