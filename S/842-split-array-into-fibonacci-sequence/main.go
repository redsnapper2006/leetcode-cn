package main

import "fmt"

func splitIntoFibonacci(S string) []int {
	buf := []byte(S)
	var recur func(buf []byte, accum []int) []int
	recur = func(buf []byte, accum []int) []int {
		// fmt.Println("begin", string(buf), accum)
		if len(buf) == 0 {
			if len(accum) <= 2 {
				return nil
			}
			return accum
		}
		if buf[0] == '0' {
			if len(accum) >= 2 {
				if accum[len(accum)-2]+accum[len(accum)-1] == 0 {
					t := make([]int, len(accum)+1)
					copy(t, accum)
					t[len(t)-1] = 0
					result := recur(buf[1:], t)
					return result
				}
				return nil
			}
			t := make([]int, len(accum)+1)
			copy(t, accum)
			t[len(t)-1] = 0
			return recur(buf[1:], t)
		}
		sum := 0
		for i, b := range buf {
			if sum*10 < 1<<31-1-int(b-'0') {
				sum = sum*10 + int(b-'0')
				// fmt.Println("sum goes to", sum, string(b), accum)
				if len(accum) >= 2 {
					if accum[len(accum)-2]+accum[len(accum)-1] == sum {
						t := make([]int, len(accum)+1)
						copy(t, accum)
						t[len(t)-1] = sum
						result := recur(buf[i+1:], t)
						return result
					} else if accum[len(accum)-2]+accum[len(accum)-1] > sum {
						continue
					} else {
						return nil
					}
				} else {
					t := make([]int, len(accum)+1)
					copy(t, accum)
					t[len(t)-1] = sum
					// fmt.Println("less 2", string(buf[i+1:]), t)
					result := recur(buf[i+1:], t)
					if result != nil {
						return result
					}
				}
			}
		}
		return nil
	}

	return recur(buf, []int{})
}

func main() {
	// fmt.Println(splitIntoFibonacci("539834657215398346785398346991079669377161950407626991734534318677529701785098211336528511"))

	fmt.Println(splitIntoFibonacci("123456579"))
}
