package main

import "fmt"

type MyCalendar struct {
	Start []int
	End   []int
}

func Constructor() MyCalendar {
	return MyCalendar{Start: []int{}, End: []int{}}
}

func (this *MyCalendar) Book(start int, end int) bool {
	if len(this.Start) == 0 {
		this.Start = append(this.Start, start)
		this.End = append(this.End, end)
		return true
	}
	idx := -1
	for i := 0; i < len(this.Start); i++ {
		if this.Start[i] > start {
			idx = i
			break
		}
	}
	if idx == -1 {
		if this.End[len(this.End)-1] <= start {
			this.Start = append(this.Start, start)
			this.End = append(this.End, end)
			return true
		} else {
			return false
		}
	} else {
		if idx == 0 {
			if this.Start[0] >= end {
				this.Start = append([]int{start}, this.Start...)
				this.End = append([]int{end}, this.End...)
				return true
			} else {
				return false
			}
		} else {
			if this.End[idx-1] <= start && this.Start[idx] >= end {
				s, e := make([]int, len(this.Start)+1), make([]int, len(this.End)+1)
				copy(s, this.Start[0:idx])
				copy(e, this.End[0:idx])
				s[idx] = start
				e[idx] = end
				copy(s[idx+1:], this.Start[idx:])
				copy(e[idx+1:], this.End[idx:])

				this.Start = s
				this.End = e
				return true
			} else {
				return false
			}
		}
	}
}

func main() {
	fmt.Println("a")
}
