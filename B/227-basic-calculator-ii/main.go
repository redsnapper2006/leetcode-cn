package main

import (
	"fmt"
)

func calculate(s string) int {
	var o []int
	var op []int

	accum := 0
	for _, v := range s {
		if v == 32 {
			continue
		}

		if v >= 48 && v <= 57 {
			accum = accum*10 + int(v) - 48
		} else if v == 43 || v == 42 || v == 45 || v == 47 {
			o = append(o, accum)
			accum = 0
			op = append(op, int(v))
		}
	}
	o = append(o, accum)

	if len(op) == 0 {
		return accum
	} else {

		for i := 0; i < len(op); i++ {
			if op[i] == 42 || op[i] == 47 {

				o1, o2 := o[i], o[i+1]
				var t int
				if op[i] == 42 {
					t = o1 * o2
				}
				if op[i] == 47 {
					t = o1 / o2
				}
				op = append(op[0:i], op[i+1:]...)
				o = append(o[0:i+1], o[i+2:]...)
				o[i] = t
				i--
			}
		}

		for len(op) > 0 {
			o1, o2 := o[0], o[1]
			var t int
			if op[0] == 43 {
				t = o1 + o2
			}
			if op[0] == 45 {
				t = o1 - o2
			}
			op = op[1:]
			o = o[1:]
			o[0] = t
		}
		return o[0]
	}
}

func main() {
	// fmt.Println(calculate(" 1234567890  "))
	// fmt.Println(calculate("3+2*2"))
	// fmt.Println(calculate("3+2*2+1"))
	fmt.Println(calculate("14/3*2"))
	// fmt.Println(calculate("3+5 / 2"))

	// 48-57 0-9
	// 43+
	// 45-
	// 42*
	// 47/
}
