package main

func uniqueOccurrences(arr []int) bool {
	buf := make(map[int]int)
	for i := 0; i < len(arr); i++ {
		buf[arr[i]]++
	}
	ret := make(map[int]int)
	for _, v := range buf {
		_, ok := ret[v]
		if !ok {
			ret[v]++
		} else {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
