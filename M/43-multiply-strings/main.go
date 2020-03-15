package main

import "fmt"

import "strconv"

func multiply(num1 string, num2 string) string {
	if num1 == "0" || num2 == "0" {
		return "0"
	}
	size := len(num1) + len(num2)

	b1, b2 := num1, num2
	if len(b1) < len(b2) {
		b1, b2 = b2, b1
	}

	buf := make([][]int, len(b2))
	for i := 0; i < len(b2); i++ {
		buf[i] = make([]int, size)
	}

	for i := len(b2) - 1; i >= 0; i-- {
		extra := 0
		for j := len(b1) - 1; j >= 0; j-- {
			r := int(b2[i]-'0') * int(b1[j]-'0')
			r += extra
			extra = r / 10
			buf[len(b2)-1-i][size-1-(len(b2)-1-i+len(b1)-1-j)] = r % 10
		}
		buf[len(b2)-1-i][size-1-(len(b2)-1-i+len(b1)-1)-1] = extra
	}

	result := make([]int, size)
	extra := 0
	for i := size - 1; i >= 0; i-- {
		sum := extra
		for j := 0; j < len(b2); j++ {
			sum += buf[j][i]
		}
		extra = sum / 10
		result[i] = sum % 10
	}
	sidx := -1
	for i := 0; i < size; i++ {
		if result[i] != 0 {
			sidx = i
			break
		}
	}
	s := ""
	for i := sidx; i < size; i++ {
		s += strconv.FormatInt(int64(result[i]), 10)
	}

	return s
}

func main() {
	fmt.Println(multiply("123", "456"))
}
