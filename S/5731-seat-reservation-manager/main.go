package main

import "fmt"

type SeatManager struct {
	A []int
}

func Constructor(n int) SeatManager {
	b := make([]int, n)
	for i := 0; i < n; i++ {
		b[i] = i + 1
	}
	return SeatManager{A: b}
}

func (this *SeatManager) Reserve() int {
	c := this.A[0]
	this.A = this.A[1:]
	return c
}

func (this *SeatManager) Unreserve(seatNumber int) {
	s, e := 0, len(this.A)-1
	for s <= e {
		m := s + (e-s)/2
		if this.A[m] > seatNumber {
			e = m - 1
		} else {
			s = m + 1
		}
	}
	if s >= len(this.A) {
		this.A = append(this.A, seatNumber)
	} else {
		// fmt.Println("here", this.A)
		this.A = append(this.A, 0)
		// fmt.Println("here22", this.A)
		// fmt.Println("here", this.A[s:len(this.A)-1])
		copy(this.A[s+1:], this.A[s:len(this.A)-1])
		this.A[s] = seatNumber
		// fmt.Println("here2", this.A)
	}

}

func main() {
	o := Constructor(5)
	fmt.Println(o.Reserve())
	fmt.Println(o.Reserve())
	o.Unreserve(2)
	fmt.Println(o.A)
	fmt.Println(o.Reserve())
	fmt.Println(o.A)
	fmt.Println(o.Reserve())
	fmt.Println(o.A)
	fmt.Println(o.Reserve())
	fmt.Println(o.A)
	fmt.Println(o.Reserve())
	fmt.Println(o.A)
	o.Unreserve(5)
	fmt.Println(o.A)
}
