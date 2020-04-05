package main

import "fmt"

type LFUCache struct {
	C   int
	MAX int
	M   map[int]int
	F   map[int]int
	MF  map[int][]int
}

func Constructor(capacity int) LFUCache {
	return LFUCache{C: 0, M: make(map[int]int), MF: make(map[int][]int), F: make(map[int]int), MAX: capacity}
}

func (this *LFUCache) Get(key int) int {
	v, ok := this.M[key]
	if !ok {
		return -1
	}

	prev := this.F[key]
	idx := -1
	for i := 0; i < len(this.MF[prev]); i++ {
		if this.MF[prev][i] == key {
			idx = i
			break
		}
	}
	this.MF[prev] = append(this.MF[prev][0:idx], this.MF[prev][idx+1:]...)
	this.F[key]++
	_, ok2 := this.MF[this.F[key]]
	if !ok2 {
		this.MF[this.F[key]] = []int{key}
	} else {
		this.MF[this.F[key]] = append(this.MF[this.F[key]], key)
	}
	return v
}

func (this *LFUCache) Put(key int, value int) {
	if this.MAX == 0 {
		return
	}
	_, ok := this.M[key]
	if !ok {
		if this.C < this.MAX {
			this.C++
		} else {
			min := 1<<31 - 1
			for _, v := range this.F {
				if v < min {
					min = v
				}
			}
			remove := this.MF[min][0]
			delete(this.M, remove)
			delete(this.F, remove)
			this.MF[min] = this.MF[min][1:]
			if len(this.MF[min]) == 0 {
				delete(this.MF, min)
			}
		}
		this.M[key] = value
		this.F[key]++
		_, ok := this.MF[this.F[key]]
		if !ok {
			this.MF[this.F[key]] = []int{key}
		} else {
			this.MF[this.F[key]] = append(this.MF[this.F[key]], key)
		}
	} else {
		this.M[key] = value
		prev := this.F[key]
		idx := -1
		for i := 0; i < len(this.MF[prev]); i++ {
			if this.MF[prev][i] == key {
				idx = i
				break
			}
		}
		this.MF[prev] = append(this.MF[prev][0:idx], this.MF[prev][idx+1:]...)
		this.F[key]++
		_, ok := this.MF[this.F[key]]
		if !ok {
			this.MF[this.F[key]] = []int{key}
		} else {
			this.MF[this.F[key]] = append(this.MF[this.F[key]], key)
		}
	}
}

func main() {
	o := Constructor(2)
	// o := Constructor(0)
	o.Put(1, 1)
	o.Put(2, 2)
	fmt.Println(o.Get(1))
	o.Put(3, 3)
	fmt.Println(o.Get(2))
	fmt.Println(o.Get(3))
	o.Put(4, 4)
	fmt.Println(o.Get(1))
	fmt.Println(o.Get(3))
	fmt.Println(o.Get(4))

}
