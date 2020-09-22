package main

import (
	"fmt"
	"sort"
)

type CordArr struct {
	B   []int
	Arr [][]int
}

func (p CordArr) Len() int {
	return len(p.Arr)
}

func (p CordArr) Swap(i, j int) {
	p.Arr[i], p.Arr[j] = p.Arr[j], p.Arr[i]
}

func (p CordArr) Less(i, j int) bool {
	c1 := p.Arr[i]
	c2 := p.Arr[j]
	x1 := c1[0] - p.B[0]
	if x1 < 0 {
		x1 = -x1
	}
	y1 := c1[1] - p.B[1]
	if y1 < 0 {
		y1 = -y1
	}
	x2 := c2[0] - p.B[0]
	if x2 < 0 {
		x2 = -x2
	}
	y2 := c2[1] - p.B[1]
	if y2 < 0 {
		y2 = -y2
	}
	return x1+y1 < x2+y2
}

func allCellsDistOrder(R int, C int, r0 int, c0 int) [][]int {
	var ret [][]int
	for i := 0; i < R; i++ {
		for j := 0; j < C; j++ {
			ret = append(ret, []int{i, j})
		}
	}
	p := CordArr{B: []int{r0, c0}, Arr: ret}
	sort.Sort(p)
	return p.Arr
}

func main() {
	fmt.Println("a")
}
