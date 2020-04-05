package main

func partitionLabels(S string) []int {
	M := make(map[byte]int)
	for i := 0; i < len(S); i++ {
		M[S[i]] = i
	}
	var r []int
	idx := 0
	for idx < len(S) {
		sIdx, eIdx := idx, M[S[idx]]
		for sIdx < eIdx {
			if M[S[sIdx]] > eIdx {
				eIdx = M[S[sIdx]]
			}
			sIdx++
		}
		r = append(r, eIdx-idx+1)
		idx = eIdx + 1
	}
	return r
}

func main() {
	fmt.Println("a")
}
