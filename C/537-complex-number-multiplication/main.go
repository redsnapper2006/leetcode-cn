package main

import (
	"fmt"
	"strconv"
	"strings"
)

func complexNumberMultiply(a string, b string) string {
	arrA := strings.Split(a, "+")
	da, _ := strconv.ParseInt(arrA[0], 10, 32)
	ra, _ := strconv.ParseInt(arrA[1][0:len(arrA[1])-1], 10, 32)

	arrB := strings.Split(b, "+")
	db, _ := strconv.ParseInt(arrB[0], 10, 32)
	rb, _ := strconv.ParseInt(arrB[1][0:len(arrB[1])-1], 10, 32)

	dr := da*db - ra*rb
	rr := ra*db + rb*da
	return fmt.Sprintf("%d+%di", dr, rr)
}

func main() {
	fmt.Println()
}
