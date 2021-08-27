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
	return p[i] > p[j]
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
	return p[i] < p[j]
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

type MedianFinder struct {
	Less *MinHeap // desc
	More *MaxHeap // asc
}

func Constructor() MedianFinder {
	minh := &MinHeap{}
	maxh := &MaxHeap{}
	heap.Init(minh)
	heap.Init(maxh)
	return MedianFinder{Less: minh, More: maxh}
}

func (this *MedianFinder) AddNum(num int) {
	if len(*this.More) == 0 {
		heap.Push(this.More, num)
		return
	}

	if (*this.More)[0] > num {
		heap.Push(this.Less, num)
	} else {
		heap.Push(this.More, num)
	}

	if len(*this.Less) > len(*this.More) {
		m := heap.Pop(this.Less).(int)
		heap.Push(this.More, m)
	} else if len(*this.More)-len(*this.Less) > 1 {
		m := heap.Pop(this.More).(int)
		heap.Push(this.Less, m)
	}
}

func (this *MedianFinder) FindMedian() float64 {
	lSize := len(*this.Less)
	mSize := len(*this.More)
	size := lSize + mSize
	if size%2 == 0 {
		return float64((*this.Less)[0]+(*this.More)[0]) / 2
	}

	return float64((*this.More)[0])
}

func main() {
	o := Constructor()

	o.AddNum(-1)
	fmt.Println(o.FindMedian())
	o.AddNum(-2)
	fmt.Println(o.FindMedian())
	o.AddNum(-3)
	fmt.Println(o.FindMedian())
	o.AddNum(-4)
	fmt.Println(o.FindMedian())
	o.AddNum(-5)
	fmt.Println(o.FindMedian())
	// o.AddNum(1)
	// fmt.Println(o.FindMedian())
	// o.AddNum(2)
	// fmt.Println(o.FindMedian())
	// o.AddNum(3)
	// fmt.Println(o.FindMedian())

}
