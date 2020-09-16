package main

import "fmt"

func distanceBetweenBusStops(distance []int, start int, destination int) int {
	x, y := start, destination
	if start > destination {
		x, y = destination, start

	}
	sum, clock := 0, 0
	for i := 0; i < len(distance); i++ {
		sum += distance[i]
		if i >= x && i < y {
			clock += distance[i]
		}
	}
	if clock > sum-clock {
		return sum - clock
	}
	return clock
}

func main() {
	fmt.Println("a")
}
