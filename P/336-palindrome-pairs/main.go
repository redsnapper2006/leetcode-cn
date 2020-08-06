package main

import (
	"fmt"
)

func palindromePairs(words []string) [][]int {
	M := map[string]int{}
	for i := 0; i < len(words); i++ {
		M[words[i]] = i
	}

	isPali := func(w string) bool {
		for m := 0; m < (len(w)+1)/2; m++ {
			if w[m] != w[len(w)-1-m] {
				return false
			}
		}
		return true
	}
	reverse := func(w string) string {
		buf := []byte(w)
		s, e := 0, len(buf)-1
		for s < e {
			buf[s], buf[e] = buf[e], buf[s]
			s++
			e--
		}
		return string(buf)
	}

	var ret [][]int
	for i := 0; i < len(words); i++ {
		for j := 0; j < len(words[i]); j++ {
			prefix, suffix := words[i][0:j], words[i][j:]
			if isPali(prefix) {
				idx, ok := M[reverse(suffix)]
				if ok && i != idx {
					ret = append(ret, []int{idx, i})
				}
			}
			if isPali(suffix) {
				idx, ok := M[reverse(prefix)]
				if ok && i != idx {
					ret = append(ret, []int{i, idx})
					if j == 0 {
						ret = append(ret, []int{idx, i})
					}
				}
			}
		}
	}
	return ret
}

func palindromePairsV2(words []string) [][]int {
	letters := make([][]int, len(words))
	for i := 0; i < len(words); i++ {
		letters[i] = make([]int, 26)
		for j := 0; j < len(words[i]); j++ {
			letters[i][words[i][j]-'a']++
		}
	}

	var candi [][]int
	for i := 0; i < len(words); i++ {
		for j := i + 1; j < len(words); j++ {
			isTarget := true
			isFirstOdd := true
			for l := 0; l < 26; l++ {
				if (letters[i][l]+letters[j][l])%2 == 0 {
					continue
				} else {
					if isFirstOdd {
						isFirstOdd = false
					} else {
						isTarget = false
						break
					}
				}
			}
			if isTarget {
				candi = append(candi, []int{i, j})
			}
		}
	}

	var ret [][]int
	for m := 0; m < len(candi); m++ {
		i, j := candi[m][0], candi[m][1]
		buf := make([]byte, len(words[i])+len(words[j]))
		copy(buf, words[i])
		copy(buf[len(words[i]):], words[j])
		isOK := true
		for m := 0; m < (len(buf)+1)/2; m++ {
			if buf[m] != buf[len(buf)-1-m] {
				isOK = false
				break
			}
		}
		if isOK {
			ret = append(ret, []int{i, j})
		}
		copy(buf, words[j])
		copy(buf[len(words[j]):], words[i])
		isOK = true
		for m := 0; m < (len(buf)+1)/2; m++ {
			if buf[m] != buf[len(buf)-1-m] {
				isOK = false
				break
			}
		}
		if isOK {
			ret = append(ret, []int{j, i})
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
