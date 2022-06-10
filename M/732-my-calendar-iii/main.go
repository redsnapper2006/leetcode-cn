package main

type MyCalendarThree struct {
	S int
	E int
	C int
	M int
	L *MyCalendarThree
	R *MyCalendarThree
}

func Constructor() MyCalendarThree {
	return MyCalendarThree{}
}

func (this *MyCalendarThree) Book(start int, end int) int {
	if this.S == this.E {
		this.C++
		this.S = start
		this.E = end
		return this.C
	}

	var recur func(p, pp *MyCalendarThree, start, end int)
	recur = func(p, pp *MyCalendarThree, start, end int) {
		if p == nil {
			p = &MyCalendarThree{S: start, E: end, C: 1}
			if pp.S >= end {
				pp.L = p
			} else {
				pp.R = p
			}
			if this.M < p.C {
				this.M = p.C
			}
			return
		}
		if p.S >= end {
			if this.M < p.C {
				this.M = p.C
			}
			recur(p.L, p, start, end)
		} else if p.E >= end {
			if p.E == end {
				if p.S == start {
					p.C++
					if this.M < p.C {
						this.M = p.C
					}
				} else if p.S < start {
					nl := &MyCalendarThree{S: p.S, E: start, C: p.C, L: p.L}
					p.L = nl
					p.S = start
					p.C++
					if this.M < p.C {
						this.M = p.C
					}
				} else {
					p.C++
					if this.M < p.C {
						this.M = p.C
					}
					recur(p.L, p, start, p.S)
				}
			} else {
				if p.S == start {
					nr := &MyCalendarThree{S: end, E: p.E, C: p.C, R: p.R}
					p.R = nr
					p.E = end
					p.C++
					if this.M < p.C {
						this.M = p.C
					}
				} else if p.S < start {
					nl := &MyCalendarThree{S: p.S, E: start, C: p.C, L: p.L}
					p.L = nl
					p.S = start
					nr := &MyCalendarThree{S: end, E: p.E, C: p.C, R: p.R}
					p.R = nr
					p.E = end
					p.C++
					if this.M < p.C {
						this.M = p.C
					}
				} else {
					nr := &MyCalendarThree{S: end, E: p.E, C: p.C, R: p.R}
					p.R = nr
					p.E = end
					p.C++
					if this.M < p.C {
						this.M = p.C
					}
					recur(p.L, p, start, p.S)
				}
			}
		} else {
			if p.S > start {
				recur(p.L, p, start, p.S)
				recur(p.R, p, p.E, end)
				p.C++
				if this.M < p.C {
					this.M = p.C
				}
			} else if p.S == start {
				recur(p.R, p, p.E, end)
				p.C++
				if this.M < p.C {
					this.M = p.C
				}
			} else if p.E > start {
				recur(p.R, p, p.E, end)
				nl := &MyCalendarThree{S: p.S, E: start, C: p.C, L: p.L}
				p.L = nl
				p.S = start
				p.C++
				if this.M < p.C {
					this.M = p.C
				}
			} else {
				recur(p.R, p, start, end)
			}
		}
	}
	recur(this, this, start, end)
	return this.M
}
