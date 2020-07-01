package main

import "fmt"

func masterMind(solution string, guess string) []int {
	sm, gm := map[byte]int{}, map[byte]int{}

	for i := 0; i < 4; i++ {
		sm[solution[i]]++
		gm[guess[i]]++
	}

	m, pm := 0, 0
	for i := 0; i < 4; i++ {
		if solution[i] == guess[i] {
			m++
			sm[solution[i]]--
			gm[guess[i]]--
		}
	}
	for k, v := range gm {
		v2, ok := sm[k]
		if ok {
			min := v
			if v > v2 {
				min = v2
			}
			pm += min
		}
	}
	return []int{m, pm}
}

func main() {
	fmt.Println("a")
}
