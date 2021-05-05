package main

import "fmt"

type StreamRank struct {
	B [][]int
	S []int
}

func Constructor() StreamRank {
	b := make([][]int, 501)
	for i := 0; i < 501; i++ {
		b[i] = make([]int, 100)
	}
	return StreamRank{B: b, S: make([]int, 501)}
}

func (sr *StreamRank) Track(x int) {
	idx := x / 100
	off := x % 100
	sr.B[idx][off]++
	sr.S[idx]++
}

func (sr *StreamRank) GetRankOfNumber(x int) int {
	idx := x / 100
	off := x % 100
	sum := 0
	for i := 0; i < idx; i++ {
		sum += sr.S[i]
	}
	for i := 0; i <= off; i++ {
		sum += sr.B[idx][i]
	}
	return sum
}

func main() {
	fmt.Println()
}
