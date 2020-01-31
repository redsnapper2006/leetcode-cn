package main

import (
	"fmt"
	"math/rand"
)

type RandomizedSet struct {
	M      map[int]*Val
	A      []int
	R      map[int]int
	Count  int
	Offset int
}

type Val struct {
	Index    int
	IsDelete bool
}

/** Initialize your data structure here. */
func Constructor() RandomizedSet {
	return RandomizedSet{M: make(map[int]*Val), A: []int{}, R: make(map[int]int), Offset: 0, Count: 0}
}

/** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
func (this *RandomizedSet) Insert(val int) bool {
	v, ok := this.M[val]
	if !ok {
		this.A = append(this.A, val)
		this.Count++
		this.M[val] = &Val{Index: this.Count - 1, IsDelete: false}
		return true
	}
	if v.IsDelete {
		v.IsDelete = false
		delete(this.R, v.Index)
		return true
	}
	return false
}

/** Removes a value from the set. Returns true if the set contained the specified element. */
func (this *RandomizedSet) Remove(val int) bool {
	v, ok := this.M[val]
	if !ok {
		return false
	} else {
		if v.IsDelete {
			return false
		} else {
			v.IsDelete = true
			this.R[v.Index]++
			return true
		}
	}
}

/** Get a random element from the set. */
func (this *RandomizedSet) GetRandom() int {
	s := rand.Intn(this.Count)
	_, ok := this.R[s]
	for ok {
		s = rand.Intn(this.Count)
		_, ok = this.R[s]
	}
	return this.A[s]
}

func main() {
	o := Constructor()
	fmt.Println(o.Insert(10))
	fmt.Println(o.Insert(20))
	fmt.Println(o.Insert(30))
	fmt.Println(o.Insert(40))

	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())
	fmt.Println(o.GetRandom())

	// fmt.Println(o.Insert(2))
	// fmt.Println(o.GetRandom())
	// fmt.Println(o.GetRandom())
	// fmt.Println(o.GetRandom())
}
