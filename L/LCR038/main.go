package main

func dailyTemperatures(temperatures []int) []int {
	idxStack := []int{}
	stack := []int{}
	ret := make([]int, len(temperatures))

	idxStack = append(idxStack, 0)
	stack = append(stack, temperatures[0])
	for i := 1; i < len(temperatures); i++ {
		for len(stack) > 0 && stack[len(stack)-1] < temperatures[i] {
			idx := idxStack[len(idxStack)-1]
			ret[idx] = i - idx
			idxStack = idxStack[0 : len(idxStack)-1]
			stack = stack[0 : len(stack)-1]
		}
		idxStack = append(idxStack, i)
		stack = append(stack, temperatures[i])
	}
	return ret
}
