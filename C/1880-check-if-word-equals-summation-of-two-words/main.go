package main

import "fmt"

func isSumEqual(firstWord string, secondWord string, targetWord string) bool {

	s1 := 0
	for i := 0; i < len(firstWord); i++ {
		s1 = s1*10 + int(firstWord[i]-'a')
	}
	s2 := 0
	for i := 0; i < len(secondWord); i++ {
		s2 = s2*10 + int(secondWord[i]-'a')
	}
	s3 := 0
	for i := 0; i < len(targetWord); i++ {
		s3 = s3*10 + int(targetWord[i]-'a')
	}
	return s1+s2 == s3
}

func main() {
	fmt.Println()
}
