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
		return false
	} else {
		return true
	}
}

func minNumber(nums []int) string {
	var buf []string
	for _, v := range nums {
		buf = append(buf, strconv.Itoa(v))
	}
	sort.Sort(StringSlice(buf))
	ret := ""
	for _, v := range buf {
		ret += v
	}
	return ret
}

func main() {
	fmt.Println("a")
}
