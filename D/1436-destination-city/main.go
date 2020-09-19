package main

import "fmt"

func destCity(paths [][]string) string {
	in, out := map[string]int{}, map[string]int{}

	for _, v := range paths {
		s, d := v[0], v[1]
		out[s]++
		in[d]++
	}
	for k, v := range in {
		_, ok := out[k]
		if !ok {
			return k
		}
	}
	return ""
}

func main() {
	fmt.Println("a")
}
