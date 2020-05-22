package main

import (
	"fmt"
	"sort"
)

type IntArrSlice [][]int

func (p IntArrSlice) Len() int {
	return len(p)
}

func (p IntArrSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p IntArrSlice) Less(i, j int) bool {
	if p[i][1] != p[j][1] {
		return p[i][1] > p[j][1]
	}

	return p[i][0] > p[j][0]
}

func filterRestaurants(restaurants [][]int, veganFriendly int, maxPrice int, maxDistance int) []int {

	idMap := map[int]int{}
	veganMap := map[int]int{}
	priceMap := map[int][]int{}
	distMap := map[int][]int{}
	rateMap := map[int]int{}

	for i := 0; i < len(restaurants); i++ {
		r := restaurants[i]
		idMap[r[0]]++
		if r[2] == 1 {
			veganMap[r[0]]++
		}
		priceMap[r[3]] = append(priceMap[r[3]], r[0])
		distMap[r[4]] = append(distMap[r[4]], r[0])
		rateMap[r[0]] = r[1]
	}
	idbuf := &idMap
	if veganFriendly == 1 {
		idbuf = &veganMap
	}

	p := map[int]int{}
	for k, v := range priceMap {
		if k <= maxPrice {
			for i := 0; i < len(v); i++ {
				_, ok := (*idbuf)[v[i]]
				if ok {
					p[v[i]]++
				}
			}
		}
	}
	d := map[int]int{}
	for k, v := range distMap {
		if k <= maxDistance {
			for i := 0; i < len(v); i++ {
				_, ok := p[v[i]]
				if ok {
					d[v[i]]++
				}
			}
		}
	}
	var buf [][]int
	for k := range d {
		buf = append(buf, []int{k, rateMap[k]})
	}
	sort.Sort(IntArrSlice(buf))
	var ret []int
	for i := 0; i < len(buf); i++ {
		ret = append(ret, buf[i][0])
	}
	return ret
}

func main() {
	fmt.Println("a")
}
