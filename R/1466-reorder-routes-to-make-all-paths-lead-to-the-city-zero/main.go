package main

import "fmt"

func minReorder(n int, connections [][]int) int {

	ingress := map[int][]int{}
	outgress := map[int][]int{}

	for i := 0; i < len(connections); i++ {
		s := connections[i][0]
		d := connections[i][1]
		ingress[d] = append(ingress[d], s)
		outgress[s] = append(outgress[s], d)
	}

	count := 0
	candi := []int{0}
	visit := make([]int, n)
	for len(candi) > 0 {
		var t []int
		for i := 0; i < len(candi); i++ {
			visit[candi[i]] = 1
			in := ingress[candi[i]]
			for j := 0; j < len(in); j++ {
				if visit[in[j]] == 1 {
					continue
				}
				t = append(t, in[j])
			}
			out := outgress[candi[i]]
			for j := 0; j < len(out); j++ {
				if visit[out[j]] == 1 {
					continue
				}
				count++
				t = append(t, out[j])
			}
		}
		candi = t
	}
	return count
}

func main() {
	fmt.Println("a")
}
