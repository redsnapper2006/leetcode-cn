package main

func giveGem(gem []int, operations [][]int) int {
	for _, o := range operations {
		s, d := o[0], o[1]
		v := gem[s] / 2
		gem[s] -= v
		gem[d] += v
	}
	min, max := gem[0], gem[0]
	for i := 1; i < len(gem); i++ {
		if gem[i] > max {
			max = gem[i]
		}
		if gem[i] < min {
			min = gem[i]
		}
	}
	return max - min
}
