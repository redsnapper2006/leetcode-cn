package main

import "fmt"

type BrowserHistory struct {
	Stack  []string
	Offset int
}

func Constructor(homepage string) BrowserHistory {
	return BrowserHistory{Stack: []string{homepage}, Offset: 0}
}

func (this *BrowserHistory) Visit(url string) {
	this.Offset++
	if this.Offset == len(this.Stack) {
		this.Stack = append(this.Stack, url)
	} else {
		this.Stack[this.Offset] = url
		this.Stack = this.Stack[0 : this.Offset+1]
	}
}

func (this *BrowserHistory) Back(steps int) string {
	cnt := steps
	if steps > this.Offset {
		cnt = this.Offset
	}
	this.Offset -= cnt
	return this.Stack[this.Offset]
}

func (this *BrowserHistory) Forward(steps int) string {
	cnt := steps
	if this.Offset+cnt >= len(this.Stack) {
		cnt = len(this.Stack) - 1 - this.Offset
	}
	this.Offset += cnt
	return this.Stack[this.Offset]
}

func main() {
	fmt.Println()
}
