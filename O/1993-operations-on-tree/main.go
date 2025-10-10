package main

import "fmt"

type LockingTree struct {
	M [][]int
	L map[int]int
	P map[int][]int
}

func Constructor(parent []int) LockingTree {
	pp := map[int][]int{}
	for i, v := range parent {
		_, ok := pp[v]
		if !ok {
			pp[v] = []int{}
		}
		pp[v] = append(pp[v], i)
	}
	matrix := make([][]int, len(parent))
	matrix[0] = []int{}
	candi := []int{0}
	for len(candi) > 0 {
		t := []int{}
		for _, c := range candi {
			next, ok := pp[c]
			if !ok {
				continue
			}
			t = append(t, next...)
			for _, n := range next {
				tt := make([]int, len(matrix[c])+1)
				copy(tt, matrix[c])
				tt[len(tt)-1] = c
				matrix[n] = tt
			}
		}
		candi = t
	}
	return LockingTree{P: pp, M: matrix, L: map[int]int{}}
}

func (this *LockingTree) Lock(num int, user int) bool {
	v, ok := this.L[num]
	if ok && v > 0 {
		return false
	}

	this.L[num] = user
	return true
}

func (this *LockingTree) Unlock(num int, user int) bool {
	v, ok := this.L[num]
	if ok && v == user {
		this.L[num] = 0
		return true
	}
	return false
}

func (this *LockingTree) Upgrade(num int, user int) bool {
	v := this.L[num]
	if v > 0 {
		return false
	}
	for _, v := range this.M[num] {
		if this.L[v] > 0 {
			return false
		}
	}

	candi := []int{num}
	isValid := false
	for len(candi) > 0 {
		t := []int{}
		for _, c := range candi {
			next, ok := this.P[c]
			if !ok {
				continue
			}
			t = append(t, next...)
			for _, n := range next {
				if this.L[n] > 0 {
					isValid = true
					break
				}
			}
			if isValid {
				break
			}
		}
		if isValid {
			break
		}
		candi = t
	}
	if !isValid {
		return false
	}

	this.L[num] = user
	candi = []int{num}
	for len(candi) > 0 {
		t := []int{}
		for _, c := range candi {
			next, ok := this.P[c]
			if !ok {
				continue
			}
			t = append(t, next...)
			for _, n := range next {
				this.L[n] = 0
			}
		}
		candi = t
	}
	return true
}

func main() {
	o := Constructor([]int{-1, 6, 5, 5, 7, 0, 7, 0, 0, 6})
	fmt.Println(o.Upgrade(5, 3))
	fmt.Println(o.Upgrade(2, 3))
	fmt.Println(o.Upgrade(7, 39))
	fmt.Println(o.Upgrade(1, 32))
	fmt.Println(o.Unlock(5, 44))
	fmt.Println(o.Unlock(2, 15))
	fmt.Println(o.Upgrade(1, 11))
	fmt.Println(o.Upgrade(1, 18))
	fmt.Println(o.Upgrade(3, 7))
	fmt.Println(o.Lock(5, 36))
	fmt.Println(o.Lock(5, 42))
	fmt.Println(o.Upgrade(8, 5))
	fmt.Println(o.Upgrade(1, 19))
	fmt.Println(o.Unlock(3, 38))
	fmt.Println(o.Upgrade(0, 27))
	fmt.Println(o.Upgrade(4, 11))
	fmt.Println(o.Upgrade(9, 2))
	fmt.Println(o.Upgrade(8, 41))
	fmt.Println(o.Unlock(5, 36))
	fmt.Println(o.Unlock(7, 29))
}
