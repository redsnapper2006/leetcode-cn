package main

import (
	"fmt"
	"strings"
)

func mostCommonWord(paragraph string, banned []string) string {

	paragraph = strings.ReplaceAll(paragraph, "!", " ")
	paragraph = strings.ReplaceAll(paragraph, "?", " ")
	paragraph = strings.ReplaceAll(paragraph, "'", " ")
	paragraph = strings.ReplaceAll(paragraph, ",", " ")
	paragraph = strings.ReplaceAll(paragraph, ";", " ")
	paragraph = strings.ReplaceAll(paragraph, ".", " ")
	paragraph = strings.ToLower(paragraph)

	B := map[string]int{}
	for _, k := range banned {
		B[k]++
	}
	M := map[string]int{}
	arr := strings.Split(paragraph, " ")
	max := -1
	ret := ""
	for _, v := range arr {
		if v == "" {
			continue
		}
		_, ok := B[v]
		if ok {
			continue
		}
		M[v]++
		if M[v] > max {
			max = M[v]
			ret = v
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
