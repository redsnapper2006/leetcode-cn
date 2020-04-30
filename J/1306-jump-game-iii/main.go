package main

func canReach(arr []int, start int) bool {
	buf := make([]int, len(arr))
	stack := []int{start}

	for len(stack) > 0 {
		var t []int
		for i := 0; i < len(stack); i++ {
			if buf[stack[i]] == 1 {
				continue
			}
			buf[stack[i]] = 1
			if stack[i]+arr[stack[i]] < len(arr) && stack[i]+arr[stack[i]] > -1 {
				t = append(t, stack[i]+arr[stack[i]])
			}
			if stack[i]-arr[stack[i]] < len(arr) && stack[i]-arr[stack[i]] > -1 {
				t = append(t, stack[i]-arr[stack[i]])
			}
		}
		stack = t
	}

	for i := 0; i < len(arr); i++ {
		if arr[i] == 0 && buf[i] == 1 {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println("a")
}
