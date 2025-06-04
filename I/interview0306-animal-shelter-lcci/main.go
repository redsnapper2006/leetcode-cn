package main

type AnimalShelf struct {
	Q  [][]int
	CC int
	DC int
}

func Constructor() AnimalShelf {
	return AnimalShelf{Q: [][]int{}, CC: 0, DC: 0}
}

func (this *AnimalShelf) Enqueue(animal []int) {
	this.Q = append(this.Q, animal)
	if animal[1] == 0 {
		this.CC++
	} else {
		this.DC++
	}
}

func (this *AnimalShelf) DequeueAny() []int {
	if len(this.Q) == 0 {
		return []int{-1, -1}
	}
	r := this.Q[0]
	if r[1] == 0 {
		this.CC--
	} else {
		this.DC--
	}
	this.Q = this.Q[1:]
	return r
}

func (this *AnimalShelf) DequeueDog() []int {
	if this.DC == 0 {
		return []int{-1, -1}
	}
	idx := -1
	for i := 0; i < len(this.Q); i++ {
		if this.Q[i][1] == 1 {
			idx = i
			break
		}
	}
	r := this.Q[idx]
	this.Q = append(this.Q[0:idx], this.Q[idx+1:]...)
	this.DC--
	return r
}

func (this *AnimalShelf) DequeueCat() []int {
	if this.CC == 0 {
		return []int{-1, -1}
	}
	idx := -1
	for i := 0; i < len(this.Q); i++ {
		if this.Q[i][1] == 0 {
			idx = i
			break
		}
	}
	r := this.Q[idx]
	this.Q = append(this.Q[0:idx], this.Q[idx+1:]...)
	this.CC--
	return r
}

func main() {

}
