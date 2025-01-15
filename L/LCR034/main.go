package main

import "fmt"

func isAlienSorted(words []string, order string) bool {
	buf := make([]int, 26)
	for i, v := range order {
		buf[int(v-'a')] = i + 1
	}

	for i := 0; i < len(words)-1; i++ {
		p, n := words[i], words[i+1]
		pIdx, nIdx := 0, 0
		isValid := true
		for pIdx < len(p) && nIdx < len(n) {
			if buf[int(p[pIdx]-'a')] < buf[int(n[nIdx]-'a')] {
				break
			} else if buf[int(p[pIdx]-'a')] == buf[int(n[nIdx]-'a')] {
				pIdx++
				nIdx++
			} else {
				isValid = false
				break
			}
		}
		// fmt.Println(pIdx, nIdx, isValid)
		if pIdx < len(p) && nIdx >= len(n) {
			isValid = false
		}
		if !isValid {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println()
}
