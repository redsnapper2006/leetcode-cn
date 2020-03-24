package main

import "fmt"

func validUtf8(data []int) bool {
	if len(data) == 0 {
		return false
	}

	for len(data) > 0 {
		d1 := data[0]
		if d1&(1<<7) == 0 {
			data = data[1:]
			continue
		}
		oneCount := 0
		for i := 0; i < 4; i++ {
			if d1&(1<<(7-i)) == 1<<(7-i) {
				oneCount++
			} else {
				break
			}
		}
		// fmt.Println("onecou", oneCount)
		if oneCount < 2 || oneCount > 4 || d1&(1<<(7-oneCount)) != 0 || len(data) < oneCount {
			return false
		}

		for i := 1; i < oneCount; i++ {
			if data[i] < 1<<7 {
				return false
			}
		}
		data = data[oneCount:]
	}

	return true
}

func main() {
	fmt.Println(validUtf8([]int{197, 130, 1}))
}
