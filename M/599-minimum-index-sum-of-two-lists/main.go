package main

func findRestaurant(list1 []string, list2 []string) []string {
	base, loop := list1, list2
	if len(list1) < len(list2) {
		base, loop = list2, list1
	}
	M := make(map[string]int)
	for i := 0; i < len(base); i++ {
		M[base[i]] = i
	}

	min := len(base) + len(loop)
	R := []string{}
	for i := 0; i < len(loop); i++ {
		if idx, ok := M[loop[i]]; ok {
			if min > idx+i {
				R = []string{loop[i]}
				min = idx + i
			} else if min == idx+i {
				R = append(R, loop[i])
			}
		}
	}
	return R
}

func main() {
	fmt.Println("a")
}
