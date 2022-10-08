package main

import "sort"

func sortPeople(names []string, heights []int) []string {
	m := map[int]string{}

	for i := 0; i < len(names); i++ {
		m[heights[i]] = names[i]
	}
	sort.Sort(sort.Reverse(sort.IntSlice(heights)))
	ret := []string{}
	for _, h := range heights {
		ret = append(ret, m[h])
	}
	return ret
}
