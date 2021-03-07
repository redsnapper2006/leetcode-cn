package main

import "fmt"

func beautySum(s string) int {
	buf := make([][]int, len(s))
	for i := 0; i < len(s); i++ {
		buf[i] = make([]int, 26)
		if i > 0 {
			for j := 0; j < 26; j++ {
				buf[i][j] = buf[i-1][j]
			}
		}
		buf[i][s[i]-'a']++
	}
	ret := 0
	for i := 0; i < len(s); i++ {
		for j := i + 1; j < len(s); j++ {
			if i > 0 {
				max, min := 0, 0
				isFirst := true
				for m := 0; m < 26; m++ {
					if buf[j][m]-buf[i][m] > 0 {
						if isFirst {
							isFirst = false
							max = buf[j][m] - buf[i][m]
							min = buf[j][m] - buf[i][m]
						} else {
							if buf[j][m]-buf[i][m] > max {
								max = buf[j][m] - buf[i][m]
							}
							if buf[j][m]-buf[i][m] < min {
								min = buf[j][m] - buf[i][m]
							}
						}
					}
				}
				ret += max - min
			} else {
				max, min := 0, 0
				isFirst := true
				for m := 0; m < 26; m++ {
					if buf[j][m] > 0 {
						if isFirst {
							isFirst = false
							max = buf[j][m]
							min = buf[j][m]
						} else {
							if buf[j][m] > max {
								max = buf[j][m]
							}
							if buf[j][m] < min {
								min = buf[j][m]
							}
						}
					}
				}
				ret += max - min
			}
		}
	}
	return ret
}

func main() {
	fmt.Println("")
}
