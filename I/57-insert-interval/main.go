package main

import "fmt"

type Interval struct {
	S int
	E int
}

func insert(intervals [][]int, newInterval []int) [][]int {
	if len(intervals) == 0 {
		return [][]int{newInterval}
	}
	var buf []Interval
	NEW := Interval{S: newInterval[0], E: newInterval[1]}
	idx := -1
	for i, iv := range intervals {
		if iv[1] < NEW.S {
			buf = append(buf, Interval{S: iv[0], E: iv[1]})
		} else if NEW.E < iv[0] {
			buf = append(buf, NEW)
			idx = i
			break
		} else {
			if iv[0] < NEW.S {
				NEW.S = iv[0]
			}
			if iv[1] > NEW.E {
				NEW.E = iv[1]
			}
		}
	}
	if idx == -1 {
		buf = append(buf, NEW)
	} else {
		for i := idx; i < len(intervals); i++ {
			buf = append(buf, Interval{S: intervals[i][0], E: intervals[i][1]})
		}
	}

	var ret [][]int
	for _, b := range buf {
		ret = append(ret, []int{b.S, b.E})
	}
	return ret
}

func main() {
	fmt.Println("a")
}
