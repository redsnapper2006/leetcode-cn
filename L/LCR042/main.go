package main

import "fmt"

type RecentCounter struct {
	Buf []int
}

func Constructor() RecentCounter {
	return RecentCounter{Buf: []int{}}
}

func (this *RecentCounter) Ping(t int) int {
	this.Buf = append(this.Buf, t)
	s := t - 3000
	idx := -1
	for i, v := range this.Buf {
		if v >= s {
			idx = i
			break
		}
	}
	this.Buf = this.Buf[idx:]
	return len(this.Buf)
}

func main() {
	fmt.Println()
}
