package main

import "fmt"

func volunteerDeployment(finalCnt []int, totalNum int64, edges [][]int, plans [][]int) []int {
	buf := make([]int, len(finalCnt)+1)
	for i, v := range finalCnt {
		buf[i+1] = v
	}
	x := make([]int, len(finalCnt)+1)
	x[0] = 1

	em := map[int][]int{}
	for _, edge := range edges {
		x, y := edge[0], edge[1]
		_, ok := em[x]
		if !ok {
			em[x] = []int{}
		}
		em[x] = append(em[x], y)
		_, ok2 := em[y]
		if !ok2 {
			em[y] = []int{}
		}
		em[y] = append(em[y], x)
	}
	for i := len(plans) - 1; i >= 0; i-- {
		plan := plans[i]
		pn := plan[0]
		idx := plan[1]
		if pn == 3 {
			for _, candi := range em[idx] {
				buf[candi] += buf[idx]
				x[candi] += x[idx]
			}
		} else if pn == 2 {
			for _, candi := range em[idx] {
				buf[candi] -= buf[idx]
				x[candi] -= x[idx]
			}
		} else {
			buf[idx] *= 2
			x[idx] *= 2
		}
	}

	sum := 0
	sumx := 0
	for i, v := range buf {
		sum += v
		sumx += x[i]
	}
	xv := int(totalNum-int64(sum)) / sumx

	ret := []int{}
	for i, v := range buf {
		ret = append(ret, v+xv*x[i])
	}

	return ret
}

func main() {
	fmt.Println()
}
