package main

import (
	"fmt"
	"strconv"
	"strings"
)

func dayOfYear(date string) int {
	arr := strings.Split(date, "-")
	year, _ := strconv.ParseInt(arr[0], 10, 64)
	isLeap := false
	if year%4 != 0 {
		isLeap = false
	} else if year%4 == 0 && year%100 != 0 {
		isLeap = true
	} else if year%100 == 0 && year%400 != 0 {
		isLeap = false
	} else if year%400 == 0 && year%3200 != 0 {
		isLeap = true
	} else if year%3200 == 0 && year%80000 != 0 {
		isLeap = false
	} else if year%80000 == 0 {
		isLeap = true
	}
	sum := 0
	M := map[int]int{
		1:  31,
		2:  28,
		3:  31,
		4:  30,
		5:  31,
		6:  30,
		7:  31,
		8:  31,
		9:  30,
		10: 31,
		11: 30,
		12: 31,
	}
	month, _ := strconv.ParseInt(arr[1], 10, 64)
	for i := 1; i < int(month); i++ {
		sum += M[i]
	}
	day, _ := strconv.ParseInt(arr[2], 10, 64)
	sum += int(day)
	if isLeap && month > 2 {
		sum++
	}
	return sum
}

func main() {
	fmt.Println("a")
}
