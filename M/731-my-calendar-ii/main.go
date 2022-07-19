package main

type MyCalendarTwo struct {
	S int
	E int
	C int
	L *MyCalendarTwo
	R *MyCalendarTwo
}

func Constructor() MyCalendarTwo {
	return MyCalendarTwo{}
}

func (this *MyCalendarTwo) Book(start int, end int) bool {
	if this.S == this.E {
		this.C++
		this.S = start
		this.E = end
		return true
	}

	var check func(p, pp *MyCalendarTwo, start, end int) bool
	check = func(p, pp *MyCalendarTwo, start, end int) bool {
		if p == nil {
			return true
		}
		if p.S >= end {
			return check(p.L, p, start, end)
		} else if p.E >= end {
			if p.S <= start {
				return p.C < 2
			} else {
				if p.C >= 2 {
					return false
				}
				return check(p.L, p, start, p.S)
			}
		} else {
			if p.S > start {
				if p.C >= 2 {
					return false
				}
				return check(p.L, p, start, p.S) && check(p.R, p, p.E, end)
			} else if p.E > start {
				if p.C >= 2 {
					return false
				}
				return check(p.R, p, p.E, end)
			} else {
				return check(p.R, p, start, end)
			}
		}
	}

	if !check(this, this, start, end) {
		return false
	}

	var recur func(p, pp *MyCalendarTwo, start, end int)
	recur = func(p, pp *MyCalendarTwo, start, end int) {
		if p == nil {
			p = &MyCalendarTwo{S: start, E: end, C: 1}
			if pp.S >= end {
				pp.L = p
			} else {
				pp.R = p
			}
			return
		}
		if p.S >= end {
			recur(p.L, p, start, end)
		} else if p.E >= end {
			if p.E == end {
				if p.S == start {
					p.C++
				} else if p.S < start {
					nl := &MyCalendarTwo{S: p.S, E: start, C: p.C, L: p.L}
					p.L = nl
					p.S = start
					p.C++
				} else {
					p.C++
					recur(p.L, p, start, p.S)
				}
			} else {
				if p.S == start {
					nr := &MyCalendarTwo{S: end, E: p.E, C: p.C, R: p.R}
					p.R = nr
					p.E = end
					p.C++
				} else if p.S < start {
					nl := &MyCalendarTwo{S: p.S, E: start, C: p.C, L: p.L}
					p.L = nl
					p.S = start
					nr := &MyCalendarTwo{S: end, E: p.E, C: p.C, R: p.R}
					p.R = nr
					p.E = end
					p.C++
				} else {
					nr := &MyCalendarTwo{S: end, E: p.E, C: p.C, R: p.R}
					p.R = nr
					p.E = end
					p.C++
					recur(p.L, p, start, p.S)
				}
			}
		} else {
			if p.S > start {
				recur(p.L, p, start, p.S)
				recur(p.R, p, p.E, end)
				p.C++
			} else if p.S == start {
				recur(p.R, p, p.E, end)
				p.C++
			} else if p.E > start {
				recur(p.R, p, p.E, end)
				nl := &MyCalendarTwo{S: p.S, E: start, C: p.C, L: p.L}
				p.L = nl
				p.S = start
				p.C++
			} else {
				recur(p.R, p, start, end)
			}
		}
	}

	recur(this, this, start, end)
	return true
}
