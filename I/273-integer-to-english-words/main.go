package main

import (
	"fmt"
	"strconv"
)

func numberToWords(num int) string {
	OnethMap := map[byte]string{
		'0': "Zero",
		'1': "One",
		'2': "Two",
		'3': "Three",
		'4': "Four",
		'5': "Five",
		'6': "Six",
		'7': "Seven",
		'8': "Eight",
		'9': "Nine",
	}
	TwoTenthMap := map[byte]string{
		'2': "Twenty",
		'3': "Thirty",
		'4': "Forty",
		'5': "Fifty",
		'6': "Sixty",
		'7': "Seventy",
		'8': "Eighty",
		'9': "Ninety",
	}
	TenthMap := map[byte]string{
		'0': "Ten",
		'1': "Eleven",
		'2': "Twelve",
		'3': "Thirteen",
		'4': "Fourteen",
		'5': "Fifteen",
		'6': "Sixteen",
		'7': "Seventeen",
		'8': "Eighteen",
		'9': "Nineteen",
	}
	LevelMap := map[int]string{
		0: "",
		1: "Thousand",
		2: "Million",
		3: "Billion",
	}

	str := strconv.FormatInt(int64(num), 10)
	s := len(str) - 1
	ret := ""
	level := 0
	for s >= 0 {
		onethS := OnethMap[str[s]]
		tenthS := ""
		if s-1 >= 0 {
			tenthS = OnethMap[str[s-1]]
		}
		hundredthS := ""
		if s-2 >= 0 {
			hundredthS = OnethMap[str[s-2]]
		}
		t := ""
		if hundredthS != "" && hundredthS != "Zero" {
			t += hundredthS + " Hundred "
		}
		if tenthS != "" && tenthS != "Zero" {
			if tenthS != "One" {
				t += TwoTenthMap[str[s-1]] + " "
			} else {
				t += TenthMap[str[s]] + " "
			}
		}
		if tenthS != "One" && onethS != "Zero" {
			t += onethS + " "
		}
		if len(t) > 0 {
			t += LevelMap[level] + " "
		}
		ret = t + ret
		level++
		s -= 3
	}
	if ret == "" {
		return "Zero"
	}
	idx := len(ret)
	for i := len(ret) - 1; i >= 0; i-- {
		if ret[i] != ' ' {
			idx = i
			break
		}
	}
	return ret[0 : idx+1]
}

func main() {
	fmt.Println(numberToWords(3))
	fmt.Println(numberToWords(23))
	fmt.Println(numberToWords(123))
	fmt.Println(numberToWords(12345))
	fmt.Println(numberToWords(1234567))
	fmt.Println(numberToWords(1234567891))
}
