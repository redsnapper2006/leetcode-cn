package main

import "fmt"

type MyHashSet struct {
	B [][]int
}

/** Initialize your data structure here. */
func Constructor() MyHashSet {
	B := make([][]int, 100)
	for i := 0; i < len(B); i++ {
		B[i] = make([]int, 10000)
	}
	return MyHashSet{B: B}
}

func (this *MyHashSet) Add(key int) {
	idx := key / 10000
	offset := key % 10000
	this.B[idx][offset] = 1
}

func (this *MyHashSet) Remove(key int) {
	idx := key / 10000
	offset := key % 10000
	this.B[idx][offset] = 0
}

/** Returns true if this set contains the specified element */
func (this *MyHashSet) Contains(key int) bool {
	idx := key / 10000
	offset := key % 10000
	return this.B[idx][offset] == 1
}

func main() {
	fmt.Println("a")
}
