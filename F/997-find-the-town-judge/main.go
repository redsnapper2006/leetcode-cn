package main

import "fmt"

func findJudge(N int, trust [][]int) int {
	if N == 1 && len(trust) == 0 {
		return 1
	}
	ingress := map[int]int{}
	outgress := map[int]int{}
	for i := 0; i < len(trust); i++ {
		out, in := trust[i][0], trust[i][1]
		ingress[in]++
		outgress[out]++
	}

	var candi []int
	for k, v := range ingress {
		if v == N-1 {
			candi = append(candi, k)
		}
	}
	if len(candi) != 1 {
		return -1
	}
	_, ok := outgress[candi[0]]
	if ok {
		return -1
	}
	return candi[0]
}

func main() {
	fmt.Println("a")
}
