package main

import "fmt"

func findReplaceString(S string, indexes []int, sources []string, targets []string) string {
	buf := []byte(S)
	M := map[int]int{}
	for i := 0; i < len(indexes); i++ {
		idx := indexes[i]
		isMatch := true
		for j := 0; j < len(sources[i]); j++ {
			if buf[idx+j] != sources[i][j] {
				isMatch = false
				break
			}
		}
		if isMatch {
			for j := 0; j < len(sources[i]); j++ {
				buf[idx+j] = '0'
			}
			M[idx] = i
		}
	}

	var ret []byte
	idx := 0
	for idx < len(buf) {
		if buf[idx] <= 'z' && buf[idx] >= 'a' {
			ret = append(ret, buf[idx])
			idx++
			continue
		}

		tIdx := M[idx]
		for i := 0; i < len(targets[tIdx]); i++ {
			ret = append(ret, targets[tIdx][i])
		}

		i := 0
		for idx < len(buf) && buf[idx] == '0' && i < len(sources[tIdx]) {
			idx++
			i++
		}
	}
	return string(ret)
}

func main() {
	fmt.Println()
}
