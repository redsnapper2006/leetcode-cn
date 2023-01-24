package main

import "fmt"

func countPoints(points [][]int, queries [][]int) []int {
	ret := make([]int, len(queries))
	for i := 0; i < len(queries); i++ {
		x, y, r := queries[i][0], queries[i][1], queries[i][2]
		cnt := 0
		for j := 0; j < len(points); j++ {
			if (points[j][0]-x)*(points[j][0]-x)+(points[j][1]-y)*(points[j][1]-y) <= r*r {
				cnt++
			}
		}
		ret[i] = cnt
	}
	return ret
}

func main() {
	fmt.Println(countPoints([][]int{{1, 2}}, [][]int{{1, 2, 3}}))
}
