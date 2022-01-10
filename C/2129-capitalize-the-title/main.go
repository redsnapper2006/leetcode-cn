package main

import (
	"fmt"
	"strings"
)

func capitalizeTitle(title string) string {
	arr := strings.Split(title, " ")
	buf := []string{}
	for _, v := range arr {
		if len(v) <= 2 {
			buf = append(buf, strings.ToLower(v))
		} else {
			buf = append(buf, strings.Title(strings.ToLower(v)))
		}
	}
	return strings.Join(buf, " ")
}

func main() {
	fmt.Println()
}
