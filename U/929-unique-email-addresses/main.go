package main

import (
	"fmt"
	"strings"
)

func numUniqueEmails(emails []string) int {
	M := map[string]int{}
	for i := 0; i < len(emails); i++ {
		arr := strings.Split(emails[i], "@")
		domain := arr[1]
		es := arr[0]
		earr := strings.Split(es, "+")
		e := earr[0]
		fe := strings.ReplaceAll(e, ".", "")
		M[fe+"@"+domain]++
	}
	return len(M)
}

func main() {
	fmt.Println("a")
}
