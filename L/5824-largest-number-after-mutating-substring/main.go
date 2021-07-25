package main

import "fmt"

func maximumNumber(num string, change []int) string {
	buf := []byte(num)

	idx := 0
	for idx < len(buf) {
		o := int(buf[idx] - '0')
		if o < change[o] {
			for idx < len(buf) {
				o := int(buf[idx] - '0')
				if o <= change[o] {
					buf[idx] = byte(change[o] + '0')
					idx++
				} else {
					break
				}
			}
			break
		}
		idx++
	}

	return string(buf)
}

func main() {
	fmt.Println(maximumNumber("214010", []int{6, 7, 9, 7, 4, 0, 3, 4, 4, 7}))
}
