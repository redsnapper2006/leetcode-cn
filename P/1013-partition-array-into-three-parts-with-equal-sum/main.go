package main

import "fmt"

func canThreePartsEqualSum(A []int) bool {
	if len(A) < 3 {
		return false
	}
	sum := 0
	for i := 0; i < len(A); i++ {
		sum += A[i]
	}
	count := sum / 3
	m := sum % 3
	if m > 0 {
		return false
	}
	s := 0
	t := -1
	for i := 0; i < len(A); i++ {
		s += A[i]
		if s == count {
			t = i
			break
		}
	}
	if t == -1 || t >= len(A)-2 {
		return false
	}
	s = 0
	n := t
	t = -1
	for i := n + 1; i < len(A); i++ {
		s += A[i]
		if s == count {
			t = i
			break
		}
	}
	if t == -1 || t >= len(A)-1 {
		return false
	}
	s = 0
	for i := t + 1; i < len(A); i++ {
		s += A[i]
	}
	if s == count {
		return true
	}
	return false
}

func main() {
	fmt.Println("a")
}
