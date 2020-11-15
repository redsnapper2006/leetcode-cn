package main

import "fmt"

func decrypt(code []int, k int) []int {
	if k == 0 {
		return make([]int, len(code))
	}

	sum := make([]int, len(code))
	for i := 0; i < len(code); i++ {
		if i == 0 {
			sum[i] = code[i]
		} else {
			sum[i] = sum[i-1] + code[i]
		}
	}

	var ret []int
	for i := 0; i < len(code); i++ {
		if k > 0 {
			if i+k < len(code) {
				ret = append(ret, sum[i+k]-sum[i])
			} else {
				ret = append(ret, sum[len(sum)-1]-sum[i]+sum[k-len(sum)+i])
			}
		} else {
			if i > -k {
				ret = append(ret, sum[i-1]-sum[i+k-1])
			} else {
				if i > 0 {
					ret = append(ret, sum[i-1]+sum[len(sum)-1]-sum[len(sum)-1+k+i])
				} else {
					ret = append(ret, sum[len(sum)-1]-sum[len(sum)-1+k-i])
				}
			}
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
