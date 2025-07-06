type MapSum struct {
	V int
	P []*MapSum
	M map[string]int
}

/** Initialize your data structure here. */
func Constructor() MapSum {
	return MapSum{V: 0, P: []*MapSum{}, M: map[string]int{}}
}

func (this *MapSum) Insert(key string, val int) {
	diff := val
	if _, ok := this.M[key]; ok {
		diff = val - this.M[key]
	}
	this.M[key] = val
	p := this

	for _, b := range []byte(key) {
		if len(p.P) == 0 {
			p.P = make([]*MapSum, 26)
		}
		off := b - 'a'
		if p.P[off] == nil {
			p.P[off] = &MapSum{}
		}
		p.P[off].V += diff
		p = p.P[off]
	}
}

func (this *MapSum) Sum(prefix string) int {
	p := this

	ans := 0
	for _, b := range []byte(prefix) {

		off := b - 'a'
		if len(p.P) == 0 || p.P[off] == nil {
			return 0
		}
		ans = p.P[off].V
		p = p.P[off]
	}
	return ans
}
