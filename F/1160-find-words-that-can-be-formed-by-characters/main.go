package main

import "fmt"

func countCharacters(words []string, chars string) int {
	M := make([]int, 26)

	for i := 0; i < len(chars); i++ {
		M[chars[i]-'a']++
	}

	sum := 0
	for i := 0; i < len(words); i++ {
		b := make([]int, 26)
		for j := 0; j < len(words[i]); j++ {
			b[words[i][j]-'a']++
		}
		isLess := true
		for m := 0; m < 26; m++ {
			if b[m] > M[m] {
				isLess = false
				break
			}
		}
		if isLess {
			sum += len(words[i])
		}
	}

	return sum

}

func main() {
	fmt.Println("a")
}
