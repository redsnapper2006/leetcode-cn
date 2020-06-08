package main

import "fmt"

func equationsPossible(equations []string) bool {
	ME := map[byte]map[byte]int{}
	for i := 0; i < len(equations); i++ {
		s := equations[i]
		if s[1] == '!' || s[0] == s[3] {
			continue
		}
		_, ok := ME[s[0]]
		if !ok {
			ME[s[0]] = map[byte]int{}
		}
		ME[s[0]][s[3]]++

		_, ok2 := ME[s[3]]
		if !ok2 {
			ME[s[3]] = map[byte]int{}
		}
		ME[s[3]][s[0]]++
	}

	var E []map[byte]int
	for len(ME) > 0 {
		var base byte
		for k := range ME {
			base = k
			break
		}

		tm := map[byte]int{}
		tm[base]++
		bb := []byte{base}
		for {
			var t []byte
			for i := 0; i < len(bb); i++ {
				candi := ME[bb[i]]
				for k := range candi {
					_, ok := tm[k]
					if !ok {
						t = append(t, k)
					}
					tm[k]++
				}
				delete(ME, bb[i])
			}
			if len(t) == 0 {
				break
			}
			bb = t
		}
		E = append(E, tm)
	}

	for i := 0; i < len(equations); i++ {
		s := equations[i]
		if s[1] == '=' {
			continue
		}

		if s[0] == s[3] {
			return false
		}

		for j := 0; j < len(E); j++ {
			e := E[j]
			_, ok := e[s[0]]
			_, ok2 := e[s[3]]
			if ok && ok2 {
				return false
			}
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
