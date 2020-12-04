package main

import "fmt"

func avoidFlood(rains []int) []int {
	M := map[int]int{}
	pending := []int{}
	var ret []int
	for idx, v := range rains {
		if v > 0 {
			prev, ok := M[v]
			if !ok {
				ret = append(ret, -1)
				M[v] = idx
			} else {
				// replace := -1
				s, e := 0, len(pending)-1
				for s <= e {
					m := s + (e-s)/2
					if pending[m] < prev {
						s = m + 1
					} else {
						e = m - 1
					}
				}
				if s >= len(pending) || pending[s] < prev {
					return nil
				}

				ret[pending[s]] = v
				pending = append(pending[0:s], pending[s+1:]...)
				ret = append(ret, -1)
				M[v] = idx
			}
		} else if v == 0 {
			ret = append(ret, 1)
			pending = append(pending, len(ret)-1)
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
