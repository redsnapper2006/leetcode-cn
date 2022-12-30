package main

type List struct {
	S int
	E int
	N *List
}
type ExamRoom struct {
	N int
	B *List
}

func Constructor(n int) ExamRoom {
	return ExamRoom{N: n, B: nil}
}

func (this *ExamRoom) Seat() int {
	if this.B == nil {
		this.B = &List{S: 0}
		return 0
	}
	if this.B.S == 0 && this.B.E == 0 && this.B.N == nil {
		this.B.E = this.N - 1
		return this.N - 1
	}

	p := this.B
	var target *List = nil
	max := 0
	for p != nil {
		s, e := p.S, p.E
		if s == -1 {
			if e > max {
				max = e
				target = p
			}
		} else if e == this.N {
			if this.N-1-s > max {
				max = this.N - 1 - s
				target = p
			}
		} else if e-s > 1 {
			diff := s + (e-s)/2 - s
			if diff > max {
				max = diff
				target = p
			}
		}
		p = p.N
	}
	if target == nil {
		return 0
	}
	if target.S == -1 {
		target.S = 0
		return 0
	} else if target.E == this.N {
		target.E = this.N - 1
		return this.N - 1
	}
	next := target.N
	s, e := target.S, target.E
	target.E = s + (e-s)/2
	target.N = &List{S: s + (e-s)/2, E: e, N: next}
	return target.E
}

func (this *ExamRoom) Leave(p int) {
	// fmt.Println(p)
	if p == 0 {
		this.B.S = -1
		return
	} else if p == this.N-1 {
		ptr := this.B

		for ptr != nil {
			if ptr.E == this.N-1 {
				ptr.E = this.N
				return
			}
			ptr = ptr.N
		}
		return
	}
	ptr := this.B
	var pp *List = nil
	for ptr != nil {
		if ptr.S == p {
			break
		}
		pp = ptr
		ptr = ptr.N
	}
	pp.E = ptr.E
	pp.N = ptr.N
}
