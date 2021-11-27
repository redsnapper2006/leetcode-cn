package main

import "fmt"

func groupAnagrams(strs []string) [][]string {
	m := map[string][]string{}
	for _, s := range strs {
		buf := make([]int, 26)
		for _, b := range s {
			buf[int(byte(b)-byte('a'))]++
		}
		k := ""
		for i := 0; i < 26; i++ {
			k += fmt.Sprintf("%03d", buf[i])
		}
		_, ok := m[k]
		if !ok {
			m[k] = []string{}
		}
		m[k] = append(m[k], s)
	}
	ret := [][]string{}
	for _, v := range m {
		ret = append(ret, v)
	}
	return ret
}

func main() {
	fmt.Println()
}
