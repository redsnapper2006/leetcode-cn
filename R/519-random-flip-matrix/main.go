package main

import (
	"fmt"
	"math/rand"
)

type Solution struct {
	R int
	C int
	M map[int]int
	S *rand.Rand
}

func Constructor(n_rows int, n_cols int) Solution {
	return Solution{R: n_rows, C: n_cols, M: map[int]int{}, S: rand.New(rand.NewSource(99))}
}

func (this *Solution) Flip() []int {
	v := -1
	for {
		v = this.S.Intn(this.R * this.C)
		_, ok := this.M[v]
		if !ok {
			this.M[v]++
			break
		}
	}

	r := v / this.C
	c := v % this.C
	return []int{r, c}

}

func (this *Solution) Reset() {
	this.M = map[int]int{}
}

func main() {
	fmt.Println("a")
}
