package main

func findRepeatedDnaSequences(s string) []string {
	M := make(map[string]int)
	for i := 0; i < len(s)-10+1; i++ {
		M[s[i:i+10]]++
	}
	var ret []string
	for k, v := range M {
		if v > 1 {
			ret = append(ret, k)
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
