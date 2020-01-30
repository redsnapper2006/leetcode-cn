package main

import (
	"fmt"
)

type MedianFinder struct {
	Less []int // desc
	More []int // asc
}

func Constructor() MedianFinder {
	return MedianFinder{Less: []int{}, More: []int{}}
}

func (this *MedianFinder) AddNum(num int) {

	lSize := len(this.Less)
	mSize := len(this.More)

	if lSize == 0 {
		this.Less = append(this.Less, num)
		return
	}
	if mSize == 0 {
		if this.Less[0] > num {
			this.More = append(this.More, this.Less[0])
			this.Less[0] = num
		} else {
			this.More = append(this.More, num)
		}
		return
	}

	if num > this.Less[0] {

		start, end := 0, len(this.More)-1
		isFound := false
		for start < end {
			if this.More[(start+end)/2] > num {
				end = (start+end)/2 - 1
			}
			if this.More[(start+end)/2] < num {
				start = (start+end)/2 + 1
			}
			if this.More[(start+end)/2] == num {
				isFound = true
				break
			}
		}
		index := -1
		if isFound {
			index = (start + end) / 2
		} else {
			if this.More[start] < num {
				index = start + 1
			} else {
				index = start
			}
		}
		t := make([]int, len(this.More)+1)
		copy(t, this.More[0:index])
		copy(t[index+1:], this.More[index:])
		t[index] = num
		this.More = t

		if len(this.More)-len(this.Less) > 1 {
			t := make([]int, len(this.Less)+1)
			copy(t[1:], this.Less)
			this.Less = t
			this.Less[0] = this.More[0]
			this.More = this.More[1:]
		}
	} else {
		// num <= this.Less[0]

		start, end := 0, len(this.Less)-1
		isFound := false
		for start < end {
			if this.Less[(start+end)/2] > num {
				start = (start+end)/2 + 1
			}
			if this.Less[(start+end)/2] < num {
				end = (start+end)/2 - 1
			}
			if this.Less[(start+end)/2] == num {
				isFound = true
				break
			}
		}
		index := -1
		if isFound {
			index = (start + end) / 2
		} else {
			if this.Less[start] > num {
				index = start + 1
			} else {
				index = start
			}
		}
		t := make([]int, len(this.Less)+1)
		copy(t, this.Less[0:index])
		copy(t[index+1:], this.Less[index:])
		t[index] = num
		this.Less = t
		if len(this.Less)-len(this.More) > 1 {
			t := make([]int, len(this.More)+1)
			copy(t[1:], this.More)
			this.More = t
			this.More[0] = this.Less[0]
			this.Less = this.Less[1:]
		}
	}

}

func (this *MedianFinder) AddNumV2(num int) {

	lSize := len(this.Less)
	mSize := len(this.More)

	if lSize == 0 {
		this.Less = append(this.Less, num)
		return
	}
	if mSize == 0 {
		if this.Less[0] > num {
			this.More = append(this.More, this.Less[0])
			this.Less[0] = num
		} else {
			this.More = append(this.More, num)
		}
		return
	}

	if num > this.Less[0] {
		if lSize > mSize && num <= this.More[0] {
			t := make([]int, mSize+1)
			copy(t[1:], this.More)
			this.More = t
			this.More[0] = num
		}

		if lSize <= mSize && num <= this.More[0] {
			t := make([]int, lSize+1)
			copy(t[1:], this.Less)
			this.Less = t
			this.Less[0] = num
		}

		// if lSize > mSize && num > this.More[0] {
		// if lSize <= mSize && num > this.More[0] {
		if num > this.More[0] {

			if lSize <= mSize {
				t := make([]int, lSize+1)
				copy(t[1:], this.Less)
				this.Less = t
				this.Less[0] = this.More[0]
				this.More = this.More[1:]
			}

			if len(this.More) == 0 {
				this.More = append(this.More, num)
				return
			}

			start, end := 0, len(this.More)-1
			isFound := false
			for start < end {
				if this.More[(start+end)/2] > num {
					end = (start+end)/2 - 1
				}
				if this.More[(start+end)/2] < num {
					start = (start+end)/2 + 1
				}
				if this.More[(start+end)/2] == num {
					isFound = true
					break
				}
			}
			index := -1
			if isFound {
				index = (start + end) / 2
			} else {
				if this.More[start] < num {
					index = start + 1
				} else {
					index = start
				}
			}
			t := make([]int, len(this.More)+1)
			copy(t, this.More[0:index])
			copy(t[index+1:], this.More[index:])
			t[index] = num
			this.More = t
		}
	} else {
		// num <= this.Less[0]
		if lSize <= mSize && num == this.Less[0] {
			t := make([]int, lSize+1)
			copy(t[1:], this.Less)
			this.Less = t
			this.Less[0] = num
		}

		if lSize > mSize && num == this.Less[0] {
			t := make([]int, mSize+1)
			copy(t[1:], this.More)
			this.More = t
			this.More[0] = num
		}

		if num < this.Less[0] {
			// if lSize > mSize && num < this.Less[0] {
			// if lSize <= mSize && num < this.Less[0] {
			if lSize > mSize {
				t := make([]int, mSize+1)
				copy(t[1:], this.More)
				this.More = t
				this.More[0] = this.Less[0]
				this.Less = this.Less[1:]
			}
			if len(this.Less) == 0 {
				this.Less = append(this.Less, num)
				return
			}
			start, end := 0, len(this.Less)-1
			isFound := false
			for start < end {
				if this.Less[(start+end)/2] > num {
					start = (start+end)/2 + 1
				}
				if this.Less[(start+end)/2] < num {
					end = (start+end)/2 - 1
				}
				if this.Less[(start+end)/2] == num {
					isFound = true
					break
				}
			}
			index := -1
			if isFound {
				index = (start + end) / 2
			} else {
				if this.Less[start] > num {
					index = start + 1
				} else {
					index = start
				}
			}
			t := make([]int, len(this.Less)+1)
			copy(t, this.Less[0:index])
			copy(t[index+1:], this.Less[index:])
			t[index] = num
			this.Less = t
		}
	}

}

func (this *MedianFinder) FindMedian() float64 {
	lSize := len(this.Less)
	mSize := len(this.More)

	fmt.Println(this.Less, this.More)
	size := lSize + mSize
	if size%2 == 0 {
		return float64(this.Less[0]+this.More[0]) / 2
	} else {
		if lSize > mSize {
			return float64(this.Less[0])
		} else {
			return float64(this.More[0])
		}
	}
}

func main() {
	o := Constructor()
	// o.AddNum(1)
	// o.AddNum(2)
	// o.AddNum(14)
	// o.AddNum(3)
	// o.AddNum(4)
	// o.AddNum(-1)
	// fmt.Println(o.FindMedian())
	// o.AddNum(-2)
	// fmt.Println(o.FindMedian())
	// o.AddNum(-3)
	// fmt.Println(o.FindMedian())
	// o.AddNum(-4)
	// fmt.Println(o.FindMedian())
	// o.AddNum(-5)
	// fmt.Println(o.FindMedian())
	o.AddNum(1)
	fmt.Println(o.FindMedian())
	o.AddNum(2)
	fmt.Println(o.FindMedian())
	o.AddNum(3)
	fmt.Println(o.FindMedian())

}
