package main

import (
	"fmt"
	"sort"
	"strconv"
)

type StringSlice []string

func (ss StringSlice) Len() int {
	return len(ss)
}

func (ss StringSlice) Swap(i, j int) {
	ss[i], ss[j] = ss[j], ss[i]
}

func (ss StringSlice) Less(i, j int) bool {
	c1, c2 := ss[i]+ss[j], ss[j]+ss[i]
	if c1 > c2 {
		return true
	} else {
		return false
	}
}

func largestNumber(nums []int) string {
	var buf []string
	for _, v := range nums {
		buf = append(buf, strconv.Itoa(v))
	}
	sort.Sort(StringSlice(buf))
	ret := ""
	for _, v := range buf {
		if v == "0" && ret == "0" {
			continue
		} else {
			ret += v
		}
	}
	return ret
}

func main() {
	fmt.Println(largestNumber([]int{3, 30, 34, 5, 9}))
	fmt.Println(largestNumber([]int{0, 0}))
}
