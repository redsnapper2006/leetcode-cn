package main

import "fmt"

func pathInZigZagTree(label int) []int {
	table := []int{2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192,
		16384, 32768, 65536, 131072, 262144, 524288}

	b := label
	var ret []int
	for b > 1 {
		ret = append(ret, b)
		idx := -1
		for i := len(table) - 1; i >= 0; i-- {
			if table[i] <= b {
				idx = i
				break
			}
		}
		diff := b - table[idx] + 2
		diff /= 2
		b = table[idx] - diff
	}
	ret = append(ret, 1)
	s, e := 0, len(ret)-1
	for s < e {
		ret[s], ret[e] = ret[e], ret[s]
		s++
		e--
	}
	return ret
}

func main() {
	fmt.Println()
}
