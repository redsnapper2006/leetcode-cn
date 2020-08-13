package main

import "fmt"

func uniqueMorseRepresentations(words []string) int {
	MORSEMAP := []string{".-", "-...", "-.-.", "-..", ".", "..-.", "--.",
		"....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-",
		".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--.."}

	rm := map[string]int{}
	for i := 0; i < len(words); i++ {
		t := ""
		for j := 0; j < len(words[i]); j++ {
			t += MORSEMAP[words[i][j]-'a']
		}

		rm[t]++
	}

	return len(rm)
}

func main() {
	fmt.Println("a")
}
