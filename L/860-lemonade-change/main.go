package main

import "fmt"

func lemonadeChange(bills []int) bool {
	buf := make([]int, 3)
	quan := []int{5, 10, 20}
	for i := 0; i < len(bills); i++ {
		if bills[i] == 5 {
			buf[0]++
		} else {
			change := bills[i] - 5

			for change > 0 {
				isFound := false
				for j := 2; j >= 0; j-- {
					if quan[j] <= change && buf[j] > 0 {
						change -= quan[j]
						buf[j]--
						isFound = true
						break
					}
				}
				if !isFound {
					return false
				}
			}
			if bills[i] == 10 {
				buf[1]++
			} else {
				buf[2]++
			}
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
