package main

import "fmt"

func maxLength(arr []string) int {
	buf := make([][]int, 1)
	buf[0] = make([]int, 26)
	for i := 0; i < len(arr); i++ {
		size := len(buf)
		letters := make([]int, 26)
		for j := 0; j < len(arr[i]); j++ {
			letters[int(arr[i][j]-'a')]++
		}
		for j := 0; j < size; j++ {
			t := make([]int, 26)
			// copy(t, buf[j])
			isCopy := true
			for m := 0; m < 26; m++ {
				if buf[j][m] > 0 && letters[m] > 0 {
					isCopy = false
					break
				}
				t[m] = buf[j][m] + letters[m]
			}
			if isCopy {
				buf = append(buf, t)
			}
		}
	}
	max := 0
	for i := 0; i < len(buf); i++ {
		count := 0
		for j := 0; j < 26; j++ {
			count += buf[i][j]
		}
		if count > max {
			max = count
		}
	}
	return max
}

func main() {
	fmt.Println()
}
