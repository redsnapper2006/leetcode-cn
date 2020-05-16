package main

import (
	"fmt"
	"math"
	"math/rand"
	"time"
)

type Solution struct {
	RAND *rand.Rand
	R    float64
	X    float64
	Y    float64
}

func Constructor(radius float64, x_center float64, y_center float64) Solution {
	return Solution{R: radius, X: x_center, Y: y_center, RAND: rand.New(rand.NewSource(time.Now().UnixNano()))}
}

func (this *Solution) RandPoint() []float64 {

	d := this.R * math.Sqrt(this.RAND.Float64())
	t := this.RAND.Float64() * 2 * math.Pi

	return []float64{d*math.Cos(t) + this.X, d*math.Sin(t) + this.Y}
}

func main() {
	fmt.Println("a")
}
