package main

import "fmt"

func findLongestWord(s string, d []string) string {

	ret := ""
	for i := 0; i < len(d); i++ {
		candi := d[i]

		sIdx, cIdx := 0, 0
		for sIdx < len(s) && cIdx < len(candi) {
			if s[sIdx] != candi[cIdx] {
				sIdx++
			} else {
				sIdx++
				cIdx++
			}
		}
		if cIdx == len(candi) && (len(ret) == 0 || len(ret) < len(candi) || (len(ret) == len(candi) && ret > candi)) {
			ret = candi
		}
	}

	return ret
}

func main() {
	fmt.Println("a")
}
