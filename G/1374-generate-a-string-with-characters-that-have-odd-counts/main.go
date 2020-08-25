package main

import "fmt"

func generateTheString(n int) string {
	if n == 1 {
		return "a"
	}

	if n%2 == 0 {
		a := []byte{}
		for i := 0; i < n-1; i++ {
			a = append(a, 'a')
		}
		return string(append(a, 'b'))
	}

	a := []byte{}
	for i := 0; i < n-2; i++ {
		a = append(a, 'a')
	}
	return string(append(a, 'b', 'c'))

}

func main() {
	fmt.Println("a")
}
