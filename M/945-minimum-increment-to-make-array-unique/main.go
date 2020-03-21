package main

func minIncrementForUnique(A []int) int {
	if len(A) <= 1 {
		return 0
	}
	M := make(map[int]int)
	min, max := A[0], A[0]
	for i := 0; i < len(A); i++ {
		if A[i] < min {
			min = A[i]
		}
		if A[i] > max {
			max = A[i]
		}
		M[A[i]]++
	}

	steps := 0
	for i := min; i < max+len(A); i++ {
		if v, ok := M[i]; ok && v > 1 {
			M[i+1] += v - 1
			steps += v - 1
		}
	}
	return steps
}

func main() {
	fmt.Println("a")
}
