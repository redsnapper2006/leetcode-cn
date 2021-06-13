package main

import "fmt"

func makeEqual(words []string) bool {
	sum := make([]int, 26)
	for i := 0; i < len(words); i++ {
		for j := 0; j < len(words[i]); j++ {
			idx := int(words[i][j] - 'a')
			sum[idx]++
		}
	}

	for i := 0; i < 26; i++ {
		if sum[i]%len(words) != 0 {
			return false
		}
	}
	return true

}

func main() {
	fmt.Println()
}
