package main

import "fmt"

func minDistance(word1 string, word2 string) int {
	n1 := len(word1)
	n2 := len(word2)
	buf := make([][]int, n1+1)
	for i := 0; i < n1+1; i++ {
		buf[i] = make([]int, n2+1)
	}

	for i := 1; i <= n2; i++ {
		buf[0][i] = buf[0][i-1] + 1
	}

	for i := 1; i <= n1; i++ {
		buf[i][0] = buf[i-1][0] + 1
	}

	for i := 1; i <= n1; i++ {
		for j := 1; j <= n2; j++ {
			if word1[i-1] == word2[j-1] {
				buf[i][j] = buf[i-1][j-1]
			} else {
				r := buf[i-1][j-1]
				if r > buf[i][j-1] {
					r = buf[i][j-1]
				}
				if r > buf[i-1][j] {
					r = buf[i-1][j]
				}
				buf[i][j] = r + 1
			}
		}
	}
	return buf[n1][n2]
}

func minDistanceV2(word1 string, word2 string) int {
	if len(word1) == 0 {
		return len(word2)
	}
	if len(word2) == 0 {
		return len(word1)
	}
	M := make(map[byte][]int)
	for i := 0; i < len(word1); i++ {
		_, ok := M[word1[i]]
		if !ok {
			M[word1[i]] = []int{i}
		} else {
			M[word1[i]] = append(M[word1[i]], i)
		}
	}

	minCount := len(word1)
	if len(word2) > len(word1) {
		minCount = len(word2)
	}
	var recur func(M1 map[byte][]int, w2 string, w2off1, w2off2 int, w1off int, oper int) int
	recur = func(M1 map[byte][]int, w2 string, w2off1, w2off2 int, w1off int, oper int) int {
		if len(w2) == w2off2 {
			var r int
			if w2off2 == w2off1 {
				r = oper + len(word1) - w1off - 1
			} else {
				steps := len(word1) - w1off - 1
				if w2off2-w2off1 > len(word1)-w1off-1 {
					steps = w2off2 - w2off1
				}
				r = oper + steps
			}
			// fmt.Println("end", w2, w2off1, w2off2, w1off, r)
			if minCount > r {
				minCount = r
			}
			return r
		}
		// fmt.Println(w2, w2off1, w2off2, w1off, oper)
		if oper > minCount {
			return 0
		}
		b := w2[w2off2]
		v, ok := M1[b]
		if ok {
			for i := 0; i < len(v); i++ {
				if v[i] > w1off {
					var steps int
					if w2off2 == w2off1 {
						steps = v[i] - w1off - 1
					} else {
						steps = v[i] - w1off - 1
						if w2off2-w2off1 > v[i]-w1off-1 {
							steps = w2off2 - w2off1
						}
					}
					if oper+steps > minCount {
						continue
					}
					recur(M1, w2, w2off2+1, w2off2+1, v[i], oper+steps)
				}
			}
		}

		recur(M1, w2, w2off1, w2off2+1, w1off, oper)
		return 0
	}
	recur(M, word2, 0, 0, -1, 0)

	return minCount
}

func main() {
	// fmt.Println(minDistance("horse", "ros"))
	// fmt.Println(minDistance("intention", "execution"))

	// fmt.Println(minDistance("trinitrophenylmethylnitramine", "dinitrophenylhydrazine"))
	// fmt.Println(minDistance("mart", "karma"))
	fmt.Println(minDistance("abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdef", "bcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefg"))
}
