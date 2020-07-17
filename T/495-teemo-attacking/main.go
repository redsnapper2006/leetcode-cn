package main

import "fmt"

func findPoisonedDuration(timeSeries []int, duration int) int {
	sum := 0
	steps := 0
	for i := 0; i < len(timeSeries); i++ {
		if timeSeries[i] >= steps {
			steps = timeSeries[i] + duration
			sum += duration
		} else if timeSeries[i]+duration > steps {
			sum += timeSeries[i] + duration - steps
			steps = timeSeries[i] + duration
		}
	}
	return sum
}

func main() {
	fmt.Println("a")
}
