package main

import "fmt"

func findingUsersActiveMinutes(logs [][]int, k int) []int {
	m := map[int]map[int]int{}
	for i := 0; i < len(logs); i++ {
		_, ok := m[logs[i][0]]
		if !ok {
			m[logs[i][0]] = map[int]int{}
		}
		m[logs[i][0]][logs[i][1]]++
	}
	ret := make([]int, k)
	for _, v := range m {
		ret[len(v)-1]++
	}
	return ret
}

func main() {
	fmt.Println(findingUsersActiveMinutes([][]int{{1, 2}}, 3))
}
