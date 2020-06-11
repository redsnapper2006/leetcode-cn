package main

import "fmt"

func verifyPostorder(postorder []int) bool {
	var recur func(po []int) bool
	recur = func(po []int) bool {
		if len(po) <= 2 {
			return true
		}
		root := po[len(po)-1]
		lIdx := -1
		for i := 0; i < len(po); i++ {
			if po[i] >= root {
				lIdx = i
				break
			}
		}
		for i := lIdx; i < len(po)-1; i++ {
			if po[i] < root {
				return false
			}
		}
		return recur(po[0:lIdx]) && recur(po[lIdx:len(po)-1])
	}

	return recur(postorder)
}

func main() {
	fmt.Println("a")
}
