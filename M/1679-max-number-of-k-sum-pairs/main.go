package main

import "fmt"

func maxOperations(nums []int, k int) int {
	M := map[int]int{}
	for _, v := range nums {
		M[v]++
	}

	var candi []int
	for k := range M {
		candi = append(candi, k)
	}
	ret := 0
	for i := 0; i < len(candi); i++ {
		if candi[i] > k/2 {
			continue
		}
		v1 := M[candi[i]]
		v2, ok := M[k-candi[i]]
		if ok {
			if k%2 == 0 && candi[i] == k/2 {
				ret += v1 / 2
			} else {
				t := v1
				if t > v2 {
					t = v2
				}
				ret += t
			}
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
