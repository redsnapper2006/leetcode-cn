package main

import (
	"fmt"
	"sort"
)

type CordArrSlice [][]int

func (p CordArrSlice) Len() int {
	return len(p)
}

func (p CordArrSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p CordArrSlice) Less(i, j int) bool {
	if p[i][1] == p[j][1] {
		return p[i][0] < p[j][0]
	}

	return p[i][1] < p[j][1]
}

func maximumUnits(boxTypes [][]int, truckSize int) int {
	cnt := 0
	sum := 0
	sort.Sort(CordArrSlice(boxTypes))
	for i := len(boxTypes) - 1; i >= 0; i-- {
		c := boxTypes[i][0]
		if c > truckSize-cnt {
			c = truckSize - cnt
		}
		cnt += c
		sum += c * boxTypes[i][1]
	}
	return sum
}

func main() {
	fmt.Println()
}
