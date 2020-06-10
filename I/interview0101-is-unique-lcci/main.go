package main

import "fmt"

func isUnique(astr string) bool {
	m := map[byte]int{}

	for i := 0; i < len(astr); i++ {
		_, ok := m[astr[i]]
		if ok {
			return false
		}
		m[astr[i]]++
	}
	return true
}

func main() {
	fmt.Println("a")
}
