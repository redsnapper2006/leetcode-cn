package main

import "fmt"

func canFinishDFS(numCourses int, prerequisites [][]int) bool {

	edgeMap := make(map[int][]int)
	for i := 0; i < len(prerequisites); i++ {
		s, e := prerequisites[i][0], prerequisites[i][1]
		v, ok := edgeMap[s]
		if !ok {
			edgeMap[s] = []int{e}
		} else {
			edgeMap[s] = append(v, e)
		}
	}

	visitHistory := make(map[int]bool)
	var DFS func(stack []int) bool
	DFS = func(stack []int) bool {
		_, e := stack[len(stack)-2], stack[len(stack)-1]

		if result, ok := visitHistory[e]; ok {
			return result
		}
		c := edgeMap[e]
		isOK := true
		for i := 0; i < len(c); i++ {
			for j := 0; j < len(stack); j++ {
				if stack[j] == c[i] {
					visitHistory[c[i]] = false
					isOK = false
				}
			}
		}

		if !isOK {
			visitHistory[e] = false
			return false
		}

		for i := 0; i < len(c); i++ {
			r := DFS(append(stack, c[i]))
			if !r {
				visitHistory[e] = false
				return false
			}
		}
		return true
	}

	for i := 0; i < len(prerequisites); i++ {
		r := DFS(prerequisites[i])
		if !r {
			visitHistory[prerequisites[i][1]] = false
			return false
		}
	}
	return true
}

func canFinish(numCourses int, prerequisites [][]int) bool {
	if len(prerequisites) == 0 || numCourses <= 1 {
		return true
	}
	nodeMap := make(map[int]int)
	ingressMap := make(map[int]int)
	edgeMap := make(map[int][]int)
	for i := 0; i < len(prerequisites); i++ {
		s, e := prerequisites[i][0], prerequisites[i][1]
		nodeMap[s]++
		nodeMap[e]++
		ingressMap[e]++
		v, ok := edgeMap[s]
		if !ok {
			edgeMap[s] = []int{e}
		} else {
			edgeMap[s] = append(v, e)
		}
	}
	numCourses = len(nodeMap)

	var q []int
	for k, _ := range nodeMap {
		if _, ok := ingressMap[k]; !ok {
			q = append(q, k)
		}
	}

	for len(q) > 0 {
		for i := 0; i < len(q); i++ {
			c := q[i]
			delete(nodeMap, c)
			edges := edgeMap[c]
			for j := 0; j < len(edges); j++ {
				ingressMap[edges[j]]--
			}
			delete(edgeMap, c)
			numCourses--
		}
		var t []int
		for k, _ := range nodeMap {
			l, ok := ingressMap[k]
			if !ok || l == 0 {
				t = append(t, k)
			}
		}
		q = t
	}

	return numCourses == 0
}

func main() {
	fmt.Println(canFinish(3, [][]int{[]int{1, 0}}))

}
