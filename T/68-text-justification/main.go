package main

import "fmt"

func fullJustify(words []string, maxWidth int) []string {
	sum := 0
	idx := 0
	var b []string
	for i := 0; i < len(words); i++ {
		if sum+len(words[i]) > maxWidth {
			wordCount := i - idx
			spaceCount := maxWidth - sum + wordCount
			if wordCount == 1 {
				s := ""
				for m := 0; m < spaceCount; m++ {
					s += " "
				}
				b = append(b, words[idx]+s)
			} else {
				t := ""
				for j := idx; j < i-1; j++ {
					s := ""
					spaces := spaceCount / (wordCount - 1)
					extra := spaceCount % (wordCount - 1)
					if extra > 0 {
						spaces++
					}
					for m := 0; m < spaces; m++ {
						s += " "
					}
					t += words[j] + s
					wordCount--
					spaceCount -= spaces
				}
				b = append(b, t+words[i-1])
			}
			sum = 0
			idx = i
		}
		sum += len(words[i]) + 1
	}
	if idx < len(words) {
		wordCount := len(words) - idx
		spaceCount := maxWidth - sum + wordCount
		if wordCount == 1 {
			s := ""
			for m := 0; m < spaceCount; m++ {
				s += " "
			}
			b = append(b, words[idx]+s)
		} else {
			t := ""
			for j := idx; j < len(words); j++ {
				t += words[j] + " "
			}
			t = t[0 : len(t)-1]
			for i := len(t); i < maxWidth; i++ {
				t += " "
			}
			b = append(b, t)
		}
	}
	return b
}

func main() {
	fmt.Println(fullJustify([]string{"This", "is", "an", "example", "of", "text", "justification."}, 16))
}
