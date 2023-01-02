package main

import (
	"container/heap"
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

func getNumberOfBacklogOrders(orders [][]int) int {
	sell := &MinHeap{}
	buy := &MaxHeap{}
	heap.Init(sell)
	heap.Init(buy)
	sm := map[int]int{}
	bm := map[int]int{}

	for _, order := range orders {
		amount := order[1]
		if order[2] == 0 {
			for amount > 0 && sell.Len() > 0 {
				p := heap.Pop(sell).(int)
				if p > order[0] {
					heap.Push(sell, p)
					break
				} else {
					diff := sm[p]
					if diff > amount {
						diff = amount
					}
					sm[p] -= diff
					amount -= diff
					if sm[p] == 0 {
						delete(sm, p)
					} else {
						heap.Push(sell, p)
					}
				}
			}
			if amount > 0 {
				_, ok := bm[order[0]]
				if !ok {
					heap.Push(buy, order[0])
				}
				bm[order[0]] += amount
			}
		} else if order[2] == 1 {
			for amount > 0 && buy.Len() > 0 {
				p := heap.Pop(buy).(int)
				// fmt.Println("sell", p, order[0])
				if p < order[0] {
					heap.Push(buy, p)
					break
				} else {
					diff := bm[p]
					if diff > amount {
						diff = amount
					}
					bm[p] -= diff
					amount -= diff
					if bm[p] == 0 {
						delete(bm, p)
					} else {
						heap.Push(buy, p)
					}
				}
			}
			if amount > 0 {
				_, ok := sm[order[0]]
				if !ok {
					heap.Push(sell, order[0])
				}
				sm[order[0]] += amount
			}
		}
		// fmt.Println(sell, buy, sm, bm)
	}

	ret := 0
	for _, v := range sm {
		v %= 1000000007
		ret += v
		ret %= 1000000007
	}
	for _, v := range bm {
		v %= 1000000007
		ret += v
		ret %= 1000000007
	}
	return ret
}
