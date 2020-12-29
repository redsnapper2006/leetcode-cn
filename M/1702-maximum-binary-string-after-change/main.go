package main

import "fmt"

func maximumBinaryString(binary string) string {
	idx := -1
	for i := 0; i < len(binary); i++ {
		if byte(binary[i]) == '0' {
			if idx == -1 || idx == i-1 {
				idx = i
			} else {
				idx++
			}
		}
	}
	buf := []byte(binary)
	for i := 0; i < len(buf); i++ {
		if i == idx {
			buf[i] = '0'
		} else {
			buf[i] = '1'
		}
	}

	return string(buf)
}

func main() {
	fmt.Println()
}
