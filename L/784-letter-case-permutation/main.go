package main

import "fmt"

func letterCasePermutation(S string) []string {
	var buf [][]byte
	if (S[0] >= 'a' && S[0] <= 'z') || (S[0] >= 'A' && S[0] <= 'Z') {
		if S[0] >= 'a' && S[0] <= 'z' {
			buf = append(buf, []byte{S[0]})
			buf = append(buf, []byte{'A' + S[0] - 'a'})
		} else {
			buf = append(buf, []byte{S[0]})
			buf = append(buf, []byte{'a' + S[0] - 'A'})
		}
	} else {
		buf = append(buf, []byte{S[0]})
	}

	for i := 1; i < len(S); i++ {
		if (S[i] >= 'a' && S[i] <= 'z') || (S[i] >= 'A' && S[i] <= 'Z') {
			size := len(buf)
			for m := 0; m < size; m++ {
				t := make([]byte, len(buf[m]))
				copy(t, buf[m])
				buf = append(buf, t)
			}
			if S[i] >= 'a' && S[i] <= 'z' {
				for m := 0; m < size; m++ {
					buf[m] = append(buf[m], S[i])
				}
				for m := size; m < 2*size; m++ {
					buf[m] = append(buf[m], 'A'+S[i]-'a')
				}
			} else {
				for m := 0; m < size; m++ {
					buf[m] = append(buf[m], S[i])
				}
				for m := size; m < 2*size; m++ {
					buf[m] = append(buf[m], 'a'+S[i]-'A')
				}
			}
		} else {
			for m := 0; m < len(buf); m++ {
				buf[m] = append(buf[m], S[i])
			}
		}
	}
	var ret []string
	for i := 0; i < len(buf); i++ {
		ret = append(ret, string(buf[i]))
	}
	return ret
}

func main() {
	fmt.Println("a")
}
