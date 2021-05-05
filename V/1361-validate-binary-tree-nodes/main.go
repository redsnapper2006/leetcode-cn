package main

import "fmt"

func validateBinaryTreeNodes(n int, leftChild []int, rightChild []int) bool {

	visit := map[int]int{}

	var recur func(n int, child []int, visit map[int]int, link map[int]int) bool
	recur = func(n int, child []int, visit map[int]int, link map[int]int) bool {
		_, ok := link[n]
		if ok {
			return false
		}
		_, ok2 := visit[n]
		if ok2 {
			return true
		}
		visit[n] = 1
		link[n] = 1
		l := true
		if child[n] != -1 {
			l = recur(child[n], child, visit, link)
		}
		return l
	}
	l := map[int]int{}
	for i := 0; i < len(leftChild); i++ {
		if leftChild[i] == -1 {
			continue
		}
		l[leftChild[i]]++
	}
	for i := 0; i < len(rightChild); i++ {
		if rightChild[i] == -1 {
			continue
		}
		if l[rightChild[i]] > 0 {
			return false
		}
	}

	for i := 0; i < len(leftChild); i++ {
		if leftChild[i] == -1 {
			continue
		}
		_, ok := visit[leftChild[i]]
		if ok {
			continue
		}
		link := map[int]int{}
		r := recur(leftChild[i], leftChild, visit, link)
		if !r {
			// fmt.Println("e2")
			return false
		}
	}
	for i := 0; i < len(rightChild); i++ {
		if rightChild[i] == -1 {
			continue
		}
		_, ok := visit[rightChild[i]]
		if ok {
			continue
		}
		link := map[int]int{}
		r := recur(rightChild[i], rightChild, visit, link)
		if !r {
			// fmt.Println("e4")
			return false
		}
	}
	// fmt.Println(visit)
	buf := make([]int, n)
	for i := 0; i < n; i++ {
		_, ok := visit[i]
		if ok {
			buf[i] = 1
		}
	}
	isFirst := true
	for i := 0; i < n; i++ {
		if buf[i] == 0 {
			if isFirst {
				isFirst = false
			} else {
				return false
			}
		}
	}

	return !isFirst
}

func main() {
	fmt.Println(validateBinaryTreeNodes(5,
		[]int{1, 3, -1, -1, -1},
		[]int{-1, 2, 4, -1, -1}))

	fmt.Println(validateBinaryTreeNodes(3,
		[]int{1, -1, 0},
		[]int{-1, -1, -1}))
}
