package main

type TwoSum struct {
	M map[int]int
}

func Constructor() TwoSum {
	return TwoSum{M: map[int]int{}}
}

func (this *TwoSum) Add(number int) {
	this.M[number]++
}

func (this *TwoSum) Find(value int) bool {
	for k, v := range this.M {
		d := value - k
		// fmt.Println(k,d, this.M)
		if d != k {
			_, ok := this.M[d]
			if ok {
				return true
			}
		} else {
			if v >= 2 {
				return true
			}
		}
	}
	return false
}
