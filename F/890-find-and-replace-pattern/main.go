package main

import "fmt"

func findAndReplacePattern(words []string, pattern string) []string {

	var ret []string
	for _, candi := range words {
		M1 := map[byte]byte{}
		M2 := map[byte]byte{}
		isMatch := true
		for i := 0; i < len(candi); i++ {
			bc := candi[i]
			pc := pattern[i]
			v1, ok1 := M1[pc]
			v2, ok2 := M2[bc]
			if !ok1 && !ok2 {
				M1[pc] = bc
				M2[bc] = pc
			} else if (ok1 && !ok2) || (!ok1 && ok2) {
				isMatch = false
				break
			} else if v1 != bc || v2 != pc {
				isMatch = false
				break
			}
		}
		if isMatch {
			ret = append(ret, candi)
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
