package main

import (
	"fmt"
)

func findOrder(numCourses int, prerequisites [][]int) []int {
	M := make(map[int][]int)
	E := make(map[int]int)
	for i := 0; i < numCourses; i++ {
		E[i] = 0
	}
	for i := 0; i < len(prerequisites); i++ {
		p := prerequisites[i]
		v, ok := M[p[1]]
		if !ok {
			M[p[1]] = []int{p[0]}
		} else {
			M[p[1]] = append(v, p[0])
		}
		E[p[0]]++
	}

	findZeroIngress := func(E map[int]int) []int {
		var ret []int
		for k, v := range E {
			if v == 0 {
				ret = append(ret, k)
			}
		}
		return ret
	}

	var ret [][]int
	idxArr := findZeroIngress(E)
	for len(idxArr) > 0 {
		ret = append(ret, idxArr)
		for i := 0; i < len(idxArr); i++ {
			numCourses--
			if v, ok := M[idxArr[i]]; ok {
				for j := 0; j < len(v); j++ {
					E[v[j]]--
				}
			}
			delete(M, idxArr[i])
			delete(E, idxArr[i])
		}
		idxArr = findZeroIngress(E)
	}
	if numCourses > 0 {
		return nil
	}

	var r []int
	for i := 0; i < len(ret); i++ {
		r = append(r, ret[i]...)
	}
	return r
}

func main() {
	fmt.Println(findOrder(2, [][]int{[]int{0, 1}}))
}
