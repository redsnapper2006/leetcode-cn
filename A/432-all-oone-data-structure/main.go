package main

import "fmt"

type AllOne struct {
	M map[string]int
	Q []map[string]int
}

/** Initialize your data structure here. */
func Constructor() AllOne {
	return AllOne{M: make(map[string]int), Q: []map[string]int{}}
}

/** Inserts a new key <Key> with value 1. Or increments an existing key by 1. */
func (this *AllOne) Inc(key string) {
	this.M[key]++
	if this.M[key] > len(this.Q) {
		this.Q = append(this.Q, make(map[string]int))
	}
	this.Q[this.M[key]-1][key]++
	if this.M[key]-1 > 0 {
		delete(this.Q[this.M[key]-1-1], key)
	}
}

/** Decrements an existing key by 1. If Key's value is 1, remove it from the data structure. */
func (this *AllOne) Dec(key string) {
	prev, ok := this.M[key]
	if !ok {
		return
	}
	this.M[key]--

	if prev >= 1 {
		delete(this.Q[prev-1], key)
	}
	if this.M[key] == 0 {
		delete(this.M, key)
	} else {
		this.Q[this.M[key]-1][key]++
	}
}

/** Returns one of the keys with maximal value. */
func (this *AllOne) GetMaxKey() string {
	for i := len(this.Q) - 1; i >= 0; i-- {
		if len(this.Q[i]) > 0 {
			for k, _ := range this.Q[i] {
				return k
			}
		}
	}
	return ""
}

/** Returns one of the keys with Minimal value. */
func (this *AllOne) GetMinKey() string {
	for i := 0; i < len(this.Q); i++ {
		if len(this.Q[i]) > 0 {
			for k, _ := range this.Q[i] {
				return k
			}
		}
	}
	return ""
}

func main() {

	o := Constructor()
	o.Inc("a")
	o.Inc("b")
	o.Inc("b")
	o.Inc("c")
	o.Inc("c")
	o.Inc("c")
	o.Dec("b")
	o.Dec("b")
	fmt.Println(o.GetMinKey())
	o.Dec("a")
	fmt.Println(o.GetMaxKey())
	fmt.Println(o.GetMinKey())
}
