package main

import "fmt"

func maxValue(n string, x int) string {
	buf := []byte{}
	isDesc := true
	if n[0] == byte('-') {
		isDesc = false
	}
	idx := 0
	if isDesc {
		for idx < len(n) {
			if int(n[idx]-'0') >= x {

				buf = append(buf, n[idx])
				idx++
			} else {
				break
			}
		}
		buf = append(buf, byte(int('0')+x))
		for idx < len(n) {
			buf = append(buf, n[idx])
			idx++
		}
	} else {
		buf = append(buf, '-')
		idx = 1
		for idx < len(n) {
			if int(n[idx]-'0') <= x {
				buf = append(buf, n[idx])
				idx++
			} else {
				break
			}
		}
		buf = append(buf, byte(int('0')+x))
		for idx < len(n) {
			buf = append(buf, n[idx])
			idx++
		}
	}
	return string(buf)
}

func main() {
	fmt.Println()
}
