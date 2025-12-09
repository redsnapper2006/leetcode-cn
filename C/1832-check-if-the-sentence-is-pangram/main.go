package main

import "fmt"

func checkIfPangram(sentence string) bool {
	buf := make([]int, 26)
	for i := 0; i < len(sentence); i++ {
		buf[sentence[i]-'a']++
	}
	for i := 0; i < len(buf); i++ {
		if buf[i] == 0 {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println(checkIfPangram("thequickbrownfoxjumpsoverthelazydog"))
}
