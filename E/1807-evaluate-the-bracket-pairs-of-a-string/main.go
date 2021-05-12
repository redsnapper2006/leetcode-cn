package main

import "fmt"

func evaluate(s string, knowledge [][]string) string {
	M := map[string]string{}
	for i := 0; i < len(knowledge); i++ {
		M[knowledge[i][0]] = knowledge[i][1]
	}

	ret := []byte{}
	key := []byte{}
	isIn := false
	for i := 0; i < len(s); i++ {
		if s[i] == byte('(') {
			isIn = true
			key = []byte{}
		} else if s[i] == byte(')') {
			isIn = false
			v, ok := M[string(key)]
			if !ok {
				ret = append(ret, '?')
			} else {
				for j := 0; j < len(v); j++ {
					ret = append(ret, v[j])
				}
			}
		} else if !isIn {
			ret = append(ret, s[i])
		} else {
			key = append(key, s[i])
		}
	}

	return string(ret)
}

func main() {
	fmt.Println()
}
