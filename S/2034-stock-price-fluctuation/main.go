package main

import (
	"container/heap"
	"fmt"
)

type MinHeap []int

func (p MinHeap) Len() int {
	return len(p)
}

func (p MinHeap) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p MinHeap) Less(i, j int) bool {
	return p[i] < p[j]
}

func (p *MinHeap) Push(c interface{}) {
	*p = append(*p, c.(int))
}
func (p *MinHeap) Pop() interface{} {
	n := len(*p)
	v := (*p)[n-1]
	*p = (*p)[0 : n-1]
	return v
}

type MaxHeap []int

func (p MaxHeap) Len() int {
	return len(p)
}

func (p MaxHeap) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p MaxHeap) Less(i, j int) bool {
	return p[i] > p[j]
}

func (p *MaxHeap) Push(c interface{}) {
	*p = append(*p, c.(int))
}
func (p *MaxHeap) Pop() interface{} {
	n := len(*p)
	v := (*p)[n-1]
	*p = (*p)[0 : n-1]
	return v
}

type StockPrice struct {
	Map          map[int]int
	MaxTimestamp int
	Min          *MinHeap
	Max          *MaxHeap
	Buf          map[int]int
}

func Constructor() StockPrice {
	minh := &MinHeap{}
	heap.Init(minh)
	maxh := &MaxHeap{}
	heap.Init(maxh)
	return StockPrice{Map: map[int]int{}, MaxTimestamp: 0, Min: minh, Max: maxh, Buf: map[int]int{}}

}

func (this *StockPrice) Update(timestamp int, price int) {
	prev, ok := this.Map[timestamp]
	if ok {
		this.Buf[prev]--
	}
	this.Map[timestamp] = price
	heap.Push(this.Max, price)
	heap.Push(this.Min, price)
	this.Buf[price]++
	if this.MaxTimestamp < timestamp {
		this.MaxTimestamp = timestamp
	}
}

func (this *StockPrice) Current() int {
	return this.Map[this.MaxTimestamp]
}

func (this *StockPrice) Maximum() int {
	// fmt.Println("max", *(this.Max), *(this.Min), this.Buf)
	for len(*(this.Max)) > 0 {
		candi := heap.Pop(this.Max).(int)
		// fmt.Println(candi)
		if this.Buf[candi] > 0 {
			heap.Push(this.Max, candi)
			return candi
		}
	}
	return -1
}

func (this *StockPrice) Minimum() int {
	for len(*(this.Min)) > 0 {
		candi := heap.Pop(this.Min).(int)
		if this.Buf[candi] > 0 {
			heap.Push(this.Min, candi)
			return candi
		}
	}
	return -1
}

func main() {
	sp := Constructor()
	sp.Update(1, 10)
	sp.Update(2, 5)
	fmt.Println(sp.Current())
	fmt.Println(sp.Maximum())
	sp.Update(1, 3)
	fmt.Println(sp.Maximum())
	sp.Update(4, 2)
	fmt.Println(sp.Minimum())
}
