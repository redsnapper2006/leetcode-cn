package main

import (
	"fmt"
	"sort"
)

type IntSlice []int

func (is IntSlice) Len() int {
	return len(is)
}

func (is IntSlice) Swap(i, j int) {
	is[i], is[j] = is[j], is[i]
}

func (is IntSlice) Less(i, j int) bool {
	if is[i] < is[j] {
		return false
	} else {
		return true
	}
}

type KthLargest struct {
	B []int
	K int
	C int
}

func Constructor(k int, nums []int) KthLargest {
	var b []int
	for i := 0; i < len(nums) && i < k; i++ {
		b = append(b, nums[i])
	}
	sort.Sort(IntSlice(b))
	for i := k; i < len(nums); i++ {
		v := nums[i]
		start, end := 0, len(b)-1
		for start < end {
			idx := (start + end) / 2
			if b[idx] > v {
				start = idx + 1
			} else if b[idx] < v {
				end = idx
			} else {
				break
			}
		}
		if start == end {
			if b[start] < v {
				copy(b[start+1:], b[start:len(b)-1])
				b[start] = v
			}
		} else {
			idx := (start + end) / 2
			copy(b[idx+1:], b[idx:len(b)-1])
			b[idx] = v
		}
	}
	return KthLargest{B: b, K: k, C: 0}
}

func (this *KthLargest) Add(val int) int {

	if len(this.B) == 0 {
		this.B = append(this.B, val)
		return val
	} else if len(this.B) == 1 {
		if this.B[0] > val {
			this.B = []int{this.B[0], val}
		} else {
			this.B = []int{val, this.B[0]}
		}
	} else if len(this.B) <= this.K {
		start, end := 0, len(this.B)-1
		for start < end {
			idx := (start + end) / 2
			if this.B[idx] > val {
				start = idx + 1
			} else if this.B[idx] < val {
				end = idx
			} else {
				break
			}
		}
		if len(this.B) < this.K {
			this.B = append(this.B, 0)
		}
		if start == end {
			if this.B[start] < val {
				copy(this.B[start+1:], this.B[start:len(this.B)-1])
				this.B[start] = val
			}
		} else {
			idx := (start + end) / 2
			copy(this.B[idx+1:], this.B[idx:len(this.B)-1])
			this.B[idx] = val
		}
	}

	if len(this.B) > this.K {
		this.B = this.B[0:this.K]
	}

	return this.B[len(this.B)-1]
}

func main() {
	o := Constructor(2, []int{0})
	fmt.Println(o.Add(-1))
	fmt.Println(o.Add(1))
	fmt.Println(o.Add(-2))
	fmt.Println(o.Add(-4))
	fmt.Println(o.Add(3))
	fmt.Println(o.Add(11))
}
