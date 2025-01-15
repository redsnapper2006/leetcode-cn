package main

import "fmt"

type MovingAverage struct {
	Sum   int
	Count int
	Arr   []int
	Size  int
}

func Constructor(size int) MovingAverage {
	return MovingAverage{Sum: 0, Count: 0, Arr: []int{}, Size: size}
}

func (this *MovingAverage) Next(val int) float64 {
	if this.Count < this.Size {
		this.Arr = append(this.Arr, val)
		this.Sum += val
		this.Count++
	} else {
		this.Sum -= this.Arr[0]
		this.Arr = append(this.Arr, val)
		this.Arr = this.Arr[1:]
		this.Sum += val
	}
	return float64(this.Sum) / float64(this.Count)
}

func main() {
	fmt.Println()
}
