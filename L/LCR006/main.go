package main

import "fmt"

func twoSum(numbers []int, target int) []int {
	m := map[int][]int{}
	for i, v := range numbers {
		m[v] = append(m[v], i)
		e := target - v
		if e == v && len(m[v]) == 2 {
			return m[v]
		} else if e != v && len(m[e]) == 1 {
			return []int{m[e][0], i}
		}
	}
	return nil
}

func main() {
	fmt.Println()
}
