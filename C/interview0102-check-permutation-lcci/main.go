package main

import "fmt"

func CheckPermutation(s1 string, s2 string) bool {
	if len(s1) != len(s2) {
		return false
	}
	m1 := map[byte]int{}
	m2 := map[byte]int{}
	for i := 0; i < len(s1); i++ {
		m1[s1[i]]++
		m2[s2[i]]++
	}
	for k, v := range m1 {
		v2, ok := m2[k]
		if !ok || v2 != v {
			return false
		}
		delete(m2, k)
	}
	return len(m2) == 0
}

func main() {
	fmt.Println("a")
}
