package main

import (
	"fmt"
	"strings"
)

func respace(dictionary []string, sentence string) int {
	buf := make([]int, len(sentence)+1)
	buf[len(sentence)] = 0
	for i := len(sentence) - 1; i >= 0; i-- {
		b := sentence[i:]
		min := buf[i+1] + 1
		for j := 0; j < len(dictionary); j++ {
			if strings.HasPrefix(b, dictionary[j]) {
				if min > buf[i+len(dictionary[j])] {
					min = buf[i+len(dictionary[j])]
				}
			}
		}
		buf[i] = min
	}
	return buf[0]
}

func main() {
	fmt.Println(respace([]string{"looked", "just", "like", "her", "brother"}, "jesslookedjustliketimherbrother"))
}
