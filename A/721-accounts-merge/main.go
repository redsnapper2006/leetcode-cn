package main

import (
	"fmt"
	"sort"
)

func accountsMerge(accounts [][]string) [][]string {
	M := map[string][]int{}
	for i, b := range accounts {
		for j := 1; j < len(b); j++ {
			M[b[j]] = append(M[b[j]], i)
		}
	}

	var ret [][]string
	for len(M) > 0 {
		kmail := ""
		for k := range M {
			kmail = k
			break
		}
		VISIT := map[string]int{}
		t := map[string]int{}
		candi := M[kmail]

		for j := 0; j < len(candi); j++ {
			for n := 1; n < len(accounts[candi[j]]); n++ {
				_, ok := VISIT[accounts[candi[j]][n]]
				if ok {
					continue
				}
				t[accounts[candi[j]][n]]++
			}
		}
		VISIT[kmail] = 1
		for {
			plen := len(t)
			for k := range t {
				_, ok := VISIT[k]
				if ok {
					continue
				}
				for m := 0; m < len(M[k]); m++ {
					for n := 1; n < len(accounts[M[k][m]]); n++ {
						t[accounts[M[k][m]][n]]++
					}
				}
				VISIT[k] = 1
			}
			if len(t) == plen {
				break
			}
		}
		// fmt.Println(kmail, t)
		var sarr []string
		for k := range t {
			sarr = append(sarr, k)
		}
		sort.Strings(sarr)
		ret = append(ret, append([]string{accounts[M[sarr[0]][0]][0]}, sarr...))
		for k := range t {
			delete(M, k)
		}
	}

	return ret
}

func main() {
	fmt.Println()
}
