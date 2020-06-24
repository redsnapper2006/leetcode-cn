package main

import "fmt"

func addStrings(num1 string, num2 string) string {
	l := len(num1)
	if len(num1) < len(num2) {
		l = len(num2)
	}
	buf := make([]byte, l+1)
	for i := 0; i < len(buf); i++ {
		buf[i] = ' '
	}
	i1, i2 := 0, 0
	isPlus := false
	for i1 < len(num1) && i2 < len(num2) {
		c1 := int(num1[len(num1)-1-i1] - '0')
		c2 := int(num2[len(num2)-1-i2] - '0')
		c := c1 + c2
		if isPlus {
			c++
		}
		if c > 9 {
			isPlus = true
			c -= 10
		} else {
			isPlus = false
		}
		buf[l-i1] = byte(int('0') + c)
		i1++
		i2++
	}
	if i1 == len(num1) {
		for i := i1; i < len(num2); i++ {
			c := int(num2[len(num2)-1-i] - '0')
			if isPlus {
				c++
			}
			if c > 9 {
				isPlus = true
				c -= 10
			} else {
				isPlus = false
			}
			buf[l-i] = byte(int('0') + c)
		}
	}
	if i2 == len(num2) {
		for i := i1; i < len(num1); i++ {
			c := int(num1[len(num1)-1-i] - '0')
			if isPlus {
				c++
			}
			if c > 9 {
				isPlus = true
				c -= 10
			} else {
				isPlus = false
			}
			buf[l-i] = byte(int('0') + c)
		}
	}
	if isPlus {
		buf[0] = '1'
	}

	if buf[0] == ' ' {
		buf = buf[1:]
	}
	return string(buf)
}

func main() {
	fmt.Println("a")
}
