package main

import "fmt"

func reorderedPowerOf2(n int) bool {
	// EXP := []int{1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192,
	// 	16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432,
	// 	67108864, 134217728, 268435456, 536870912}
	// occu := make([][]int, len(EXP))
	// for i, v := range EXP {
	// 	occu[i] = make([]int, 10)
	// 	t := v
	// 	for t > 0 {
	// 		m := t % 10
	// 		occu[i][m]++
	// 		t /= 10
	// 	}
	// }
	// fmt.Println(occu)

	occu := [][]int{{0, 1, 0, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 1, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 0, 1, 0}, {0, 1, 0, 0, 0, 0, 1, 0, 0, 0}, {0, 0, 1, 1, 0, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 1, 0, 1, 0, 0, 0}, {0, 1, 1, 0, 0, 0, 0, 0, 1, 0}, {0, 0, 1, 0, 0, 1, 1, 0, 0, 0}, {0, 1, 1, 0, 0, 1, 0, 0, 0, 0}, {1, 1, 1, 0, 1, 0, 0, 0, 0, 0}, {1, 0, 1, 0, 1, 0, 0, 0, 1, 0}, {1, 0, 0, 0, 1, 0, 1, 0, 0, 1}, {0, 1, 1, 0, 0, 0, 0, 0, 1, 1}, {0, 1, 0, 1, 1, 0, 1, 0, 1, 0}, {0, 0, 1, 1, 0, 0, 1, 1, 1, 0}, {0, 0, 0, 1, 0, 2, 2, 0, 0, 0}, {1, 2, 1, 1, 0, 0, 0, 1, 0, 0}, {0, 1, 2, 0, 2, 0, 1, 0, 0, 0}, {0, 0, 2, 0, 1, 1, 0, 0, 2, 0}, {1, 1, 0, 0, 1, 1, 1, 1, 1, 0}, {1, 1, 2, 0, 0, 1, 0, 1, 0, 1}, {1, 1, 0, 1, 3, 0, 0, 0, 0, 1}, {1, 0, 0, 1, 0, 0, 1, 0, 4, 0}, {0, 2, 1, 0, 0, 0, 2, 3, 0, 0}, {0, 0, 1, 3, 2, 2, 0, 0, 0, 0}, {1, 1, 0, 0, 1, 0, 2, 1, 2, 0}, {0, 2, 2, 1, 1, 0, 0, 2, 1, 0}, {0, 0, 1, 1, 2, 2, 2, 0, 1, 0}, {1, 1, 1, 1, 0, 1, 1, 1, 1, 1}}

	buf := make([]int, 10)
	t := n
	for t > 0 {
		m := t % 10
		buf[m]++
		t /= 10
	}

	for _, o := range occu {
		isTarget := true
		for i := 0; i < 10; i++ {
			if o[i] != buf[i] {
				isTarget = false
				break
			}
		}
		if isTarget {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println(reorderedPowerOf2(1))
}