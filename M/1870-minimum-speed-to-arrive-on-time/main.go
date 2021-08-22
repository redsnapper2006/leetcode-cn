package main

import "fmt"

func minSpeedOnTime(dist []int, hour float64) int {
	if hour <= float64(len(dist)-1) {
		return -1
	}

	min, max := 1, 1<<16-1
	for min <= max {
		speed := min + (max-min)/2
		var hours float64 = 0.0
		for i := 0; i < len(dist)-1; i++ {
			h := dist[i] / speed
			if h*speed < dist[i] {
				h++
			}
			hours += float64(h)
		}
		hours += float64(dist[len(dist)-1]) / float64(speed)
		if hours <= hour {
			max = speed - 1
		} else {
			min = speed + 1
		}
	}
	return min
}

func main() {
	fmt.Println()
}
