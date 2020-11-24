package main

import (
	"fmt"
	"time"
)

func daysBetweenDates(date1 string, date2 string) int {
	t1, _ := time.Parse("2006-01-02", date1)
	t2, _ := time.Parse("2006-01-02", date2)
	r := t1.Sub(t2).Day()
	if r < 0 {
		return -r
	}
	return r
}

func main() {
	fmt.Println(daysBetweenDates("2019-02-04", "a"))
}
