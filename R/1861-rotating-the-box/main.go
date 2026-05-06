package main

import "fmt"

func rotateTheBox(box [][]byte) [][]byte {
	buf := make([][]byte, len(box[0]))
	for i := 0; i < len(buf); i++ {
		buf[i] = make([]byte, len(box))
	}

	for j := len(box[0]) - 1; j >= 0; j-- {
		for i := 0; i < len(box); i++ {
			org := box[i][j]
			if org == byte('*') || org == byte('.') {
				buf[j][len(box)-1-i] = org
				continue
			}

			orgJ := j
			for orgJ < len(box[0])-1 {
				if buf[orgJ+1][len(box)-1-i] == byte('.') {
					buf[orgJ][len(box)-1-i] = byte('.')
					orgJ++
				} else {
					break
				}
			}
			buf[orgJ][len(box)-1-i] = byte('#')
		}
	}
	return buf
}

func main() {
	fmt.Println()
}
