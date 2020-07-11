package main

import "fmt"

func groupAnagrams(strs []string) [][]string {
	M := map[string][]string{}
	for i := 0; i < len(strs); i++ {
		buf := make([]int, 26)
		for j := 0; j < len(strs[i]); j++ {
			buf[int(strs[i][j]-'a')]++
		}
		s := ""
		for j := 0; j < 26; j++ {
			if buf[j] != 0 {
				s += fmt.Sprintf("%s%d", string('a'+j), buf[j])
			}
		}
		_, ok := M[s]
		if !ok {
			M[s] = []string{}
		}
		M[s] = append(M[s], strs[i])
	}
	var ret [][]string
	for _, v := range M {
		ret = append(ret, v)
	}
	return ret
}

func main() {
	fmt.Println("a")
}
