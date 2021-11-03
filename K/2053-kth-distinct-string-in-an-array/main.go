package main

import "fmt"

func kthDistinct(arr []string, k int) string {
	m := map[string]int{}
	for _, v := range arr {
		m[v]++
	}
	cnt := 0
	for _, v := range arr {
		if m[v] == 1 {
			cnt++
		}
		if cnt == k {
			return v
		}
	}
	return ""
}

func main() {
	fmt.Println()
}
