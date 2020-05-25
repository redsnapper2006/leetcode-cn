package main

import "fmt"

type FreqStack struct {
	Stack   []int
	FreqKey map[int]int
	FreqCnt map[int]int
}

func Constructor() FreqStack {
	return FreqStack{Stack: []int{}, FreqKey: make(map[int]int), FreqCnt: make(map[int]int)}
}

func (this *FreqStack) Push(x int) {
	this.Stack = append(this.Stack, x)
	prev, ok := this.FreqKey[x]
	if ok {
		this.FreqCnt[prev]--
		if this.FreqCnt[prev] == 0 {
			delete(this.FreqCnt, prev)
		}
	}
	this.FreqKey[x]++
	this.FreqCnt[this.FreqKey[x]]++
}

func (this *FreqStack) Pop() int {
	maxFreq := -1
	for k := range this.FreqCnt {
		if k > maxFreq {
			maxFreq = k
		}
	}

	idx := -1
	for i := len(this.Stack) - 1; i >= 0; i-- {
		if this.FreqKey[this.Stack[i]] == maxFreq {
			idx = i
			break
		}
	}
	item := this.Stack[idx]
	this.Stack = append(this.Stack[0:idx], this.Stack[idx+1:]...)
	this.FreqKey[item]--
	if this.FreqKey[item] == 0 {
		delete(this.FreqKey, item)
	} else {
		this.FreqCnt[this.FreqKey[item]]++
	}
	this.FreqCnt[maxFreq]--
	if this.FreqCnt[maxFreq] == 0 {
		delete(this.FreqCnt, maxFreq)
	}

	return item
}

func main() {
	o := Constructor()

	o.Push(5)
	o.Push(7)
	o.Push(5)
	o.Push(7)
	o.Push(4)
	o.Push(5)
	fmt.Println(o.Pop())
	fmt.Println(o.Pop())
	fmt.Println(o.Pop())
	fmt.Println(o.Pop())

	fmt.Println("a")
}
