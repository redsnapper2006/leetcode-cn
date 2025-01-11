package main

type MaxQueue struct {
	Q []int
	M [][]int
	S []int
}

func Constructor() MaxQueue {
	m := make([][]int, 100)
	s := make([]int, 100)
	for i := 0; i < len(m); i++ {
		m[i] = make([]int, 1000)
	}
	return MaxQueue{Q: []int{}, M: m, S: s}
}

func (this *MaxQueue) Max_value() int {
	if len(this.Q) == 0 {
		return -1
	}
	div := -1
	for i := 99; i >= 0; i-- {
		if this.S[i] > 0 {
			div = i
			break
		}
	}
	for i := 999; i >= 0; i-- {
		if this.M[div][i] > 0 {
			return div*1000 + i + 1
		}
	}
	return -1
}

func (this *MaxQueue) Push_back(value int) {
	this.Q = append(this.Q, value)
	div := (value - 1) / 1000
	offset := (value - 1) % 1000
	this.M[div][offset]++
	this.S[div]++
}

func (this *MaxQueue) Pop_front() int {
	if len(this.Q) == 0 {
		return -1
	}
	candi := this.Q[0]
	this.Q = this.Q[1:]
	div := (candi - 1) / 1000
	offset := (candi - 1) % 1000
	this.M[div][offset]--
	this.S[div]--
	return candi
}

func main() {

}
