package main

import "fmt"

type SummaryRanges struct {
	M map[int]int
	S []int
}

func Constructor() SummaryRanges {
	return SummaryRanges{M: map[int]int{}, S: []int{}}
}

func (this *SummaryRanges) AddNum(val int) {
	_, ok := this.M[val]
	if ok {
		return
	}

	minus := val - 1
	plus := val + 1
	_, ok1 := this.M[minus]
	left := -1
	if !ok1 {
		left = val
	} else {
		left = this.M[minus]
	}

	_, ok2 := this.M[plus]
	right := -1
	if !ok2 {
		right = val
	} else {
		right = this.M[plus]
	}

	this.M[left] = right
	this.M[right] = left
	if left != val && right != val {
		this.M[val] = 1
	}

	if left == val {
		s, e := 0, len(this.S)-1
		for s <= e {
			m := s + (e-s)/2
			if this.S[m] > val {
				e = m - 1
			} else if this.S[m] < val {
				s = m + 1
			}
		}
		if s >= len(this.S) {
			this.S = append(this.S, val)
		} else {
			t := make([]int, len(this.S)+1)
			copy(t, this.S[0:s])
			t[s] = val
			copy(t[s+1:], this.S[s:])
			this.S = t
		}
	}

	if right != val {
		// fmt.Println(val, this.S, right, plus)
		s, e := 0, len(this.S)-1
		idx := -1
		for s <= e {
			m := s + (e-s)/2
			if this.S[m] > plus {
				e = m - 1
			} else if this.S[m] < plus {
				s = m + 1
			} else {
				idx = m
				break
			}
		}

		t := make([]int, len(this.S)-1)
		copy(t, this.S[0:idx])
		copy(t[idx:], this.S[idx+1:])
		this.S = t
	}
}

func (this *SummaryRanges) GetIntervals() [][]int {
	ret := [][]int{}
	for _, k := range this.S {
		ret = append(ret, []int{k, this.M[k]})
	}
	return ret
}

func main() {
	fmt.Println()
}
