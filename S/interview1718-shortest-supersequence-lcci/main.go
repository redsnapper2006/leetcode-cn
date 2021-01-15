package main

import "fmt"

func shortestSeq(big []int, small []int) []int {
	M := map[int]int{}
	for _, v := range small {
		M[v] = 0
	}

	s := 0
	for s < len(big) {
		_, ok := M[big[s]]
		if ok {
			break
		}
		s++
	}
	if s >= len(big) {
		return nil
	}

	min := len(big)
	m := map[int]int{}
	p := s
	ls, le := -1, -1
	for s < len(big) {
		_, ok := M[big[s]]
		if ok {
			m[big[s]]++
			if len(m) == len(M) {

				for i := p; i <= s; i++ {
					_, ok2 := m[big[i]]
					if !ok2 {
						continue
					}
					m[big[i]]--
					if m[big[i]] == 0 {
						if s-i < min {
							min = s - i
							ls = i
							le = s
						}
						delete(m, big[i])
						p = i + 1
						break
					}
				}
			}
		}
		s++
	}

	if le == -1 {
		return nil
	}
	return []int{ls, le}
}

func main() {
	fmt.Println()
}
