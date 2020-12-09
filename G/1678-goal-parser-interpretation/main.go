package main

import "fmt"

func interpret(command string) string {
	var ret []byte
	// inPar := false
	isA := false
	for _, b := range command {
		if b == 'G' {
			ret = append(ret, 'G')
		} else if b == 'a' {
			isA = true
		} else if b == ')' {
			if isA {
				ret = append(ret, 'a', 'l')
			} else {
				ret = append(ret, 'o')
			}
			// inPar = false
			isA = false
		}
	}
	return string(ret)
}

func main() {
	fmt.Println()
}
