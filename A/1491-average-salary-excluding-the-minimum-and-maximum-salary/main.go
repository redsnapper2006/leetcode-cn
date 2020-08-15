package main

import "fmt"

func average(salary []int) float64 {
	min, max := 10000000, -1
	sum := 0
	for i := 0; i < len(salary); i++ {
		sum += salary[i]
		if salary[i] < min {
			min = salary[i]
		}
		if salary[i] > max {
			max = salary[i]
		}
	}

	return float64(sum-min-max) / float64(len(salary)-2)
}

func main() {
	fmt.Println("a")
}
