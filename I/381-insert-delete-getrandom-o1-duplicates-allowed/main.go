package main

import (
	"fmt"
	"math/rand"
)

type RandomizedCollection struct {
	M map[int]map[int]int
	T []int
}

func Constructor() RandomizedCollection {
	return RandomizedCollection{M: map[int]map[int]int{}, T: []int{}}
}

func (this *RandomizedCollection) Insert(val int) bool {
	this.T = append(this.T, val)

	_, ok := this.M[val]
	if !ok {
		this.M[val] = map[int]int{}
	}
	this.M[val][len(this.T)-1] = 1

	if !ok {
		return true
	}
	return false
}

func (this *RandomizedCollection) Remove(val int) bool {
	_, ok := this.M[val]
	if !ok {
		return false
	}
	idx := -1
	for i, k := range this.T {
		if k == val {
			idx = i
			break
		}
	}
	this.T[idx] = this.T[len(this.T)-1]
	delete(this.M[val], idx)
	delete(this.M[this.T[idx]], len(this.T)-1)
	if idx < len(this.T)-1 {
		this.M[this.T[idx]][idx] = 1
	}
	if len(this.M[val]) == 0 {
		delete(this.M, val)
	}
	this.T = this.T[0 : len(this.T)-1]
	return true
}

func (this *RandomizedCollection) GetRandom() int {
	return this.T[rand.Intn(len(this.T))]
}

func main() {
	fmt.Println("a")
}
