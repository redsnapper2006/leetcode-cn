package main

import "fmt"

type OrderedStream struct {
	BUF []string
	CAP int
	OFF int
}

func Constructor(n int) OrderedStream {
	return OrderedStream{BUF: make([]string, n), CAP: n, OFF: 0}
}

func (this *OrderedStream) Insert(id int, value string) []string {
	this.BUF[id-1] = value
	if this.BUF[this.OFF] == "" {
		return nil
	}
	p := this.OFF
	for this.OFF < this.CAP && this.BUF[this.OFF] != "" {
		this.OFF++
	}
	return this.BUF[p:this.OFF]
}

func main() {
	fmt.Println()
}
