package main

import "fmt"

type TopVotedCandidate struct {
	Max   []int
	Times []int
}

func Constructor(persons []int, times []int) TopVotedCandidate {
	pm := map[int]int{}

	max := 0
	person := -1
	maxArr := make([]int, len(persons))
	for i := 0; i < len(persons); i++ {
		pm[persons[i]]++
		cur := pm[persons[i]]
		if cur >= max {
			max = cur
			person = persons[i]
		}
		maxArr[i] = person
	}

	Max := make([]int, times[len(times)-1]+1)
	idx := 0
	for i := times[0]; i < len(Max); i++ {
		if i == times[idx] {
			idx++
		}
		if idx < len(times) && i <= times[idx] {
			Max[i] = maxArr[idx-1]
		} else if idx >= len(times) {
			Max[i] = maxArr[idx-1]
		}
	}
	return TopVotedCandidate{Max: Max, Times: times}
}

func (this *TopVotedCandidate) Q(t int) int {
	if t >= this.Times[len(this.Times)-1] {
		return this.Max[this.Times[len(this.Times)-1]]
	}
	return this.Max[t]
}

func main() {
	fmt.Println()
}
