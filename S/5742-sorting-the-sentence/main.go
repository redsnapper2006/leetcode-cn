package main

import (
	"fmt"
	"strings"
)

func sortSentence(s string) string {
	arr := strings.Split(s, " ")
	buf := make([]string, len(arr))
	for i := 0; i < len(arr); i++ {
		idx := arr[i][len(arr[i])-1] - byte('0')
		buf[idx-1] = arr[i][0 : len(arr[i])-1]
	}
	return strings.Join(buf, " ")
}

func main() {
	fmt.Println()
}
