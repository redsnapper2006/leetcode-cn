package main

import "fmt"

type Employee struct {
	Id           int
	Importance   int
	Subordinates []int
}

func getImportance(employees []*Employee, id int) int {
	M := make(map[int]int)
	S := make(map[int]*Employee)
	for i := 0; i < len(employees); i++ {
		if len(employees[i].Subordinates) == 0 {
			M[employees[i].Id] = employees[i].Importance
		} else {
			S[employees[i].Id] = employees[i]
		}
	}

	var recur func(M map[int]int, S map[int]*Employee, id int) int
	recur = func(M map[int]int, S map[int]*Employee, id int) int {
		v, ok := M[id]
		if ok {
			return v
		}

		accum := S[id].Importance
		for i := 0; i < len(S[id].Subordinates); i++ {
			accum += recur(M, S, S[id].Subordinates[i])
		}
		M[id] = accum
		return accum
	}

	return recur(M, S, id)
}

func main() {
	fmt.Println("a")
}
