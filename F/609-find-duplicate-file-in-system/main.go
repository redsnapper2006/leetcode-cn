package main

import (
	"fmt"
	"strings"
)

func findDuplicate(paths []string) [][]string {
	BUF := make(map[string][]string)
	for i := 0; i < len(paths); i++ {
		firstArr := strings.Split(paths[i], " ")
		path := firstArr[0]
		for i := 1; i < len(firstArr); i++ {
			secondArr := strings.Split(firstArr[i], "(")
			fileName := secondArr[0]
			content := secondArr[1][0:len(secondArr[1])]
			_, ok := BUF[content]
			if !ok {
				BUF[content] = []string{path + "/" + fileName}
			} else {
				BUF[content] = append(BUF[content], path+"/"+fileName)
			}
		}

	}
	var ret [][]string
	for _, v := range BUF {
		if len(v) > 1 {
			ret = append(ret, v)
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
