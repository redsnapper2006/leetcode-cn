package main

type TH struct {
	Zero  *TH
	One   *TH
	Index []int
	Level []int
}

func getCoprimes(nums []int, edges [][]int) []int {
	// 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47
	m := map[int]int{
		2: 0, 3: 1, 5: 2, 7: 3, 11: 4, 13: 5, 17: 6, 19: 7, 23: 8, 29: 9, 31: 10, 37: 11, 41: 12, 43: 13, 47: 14}

	factor := make([]int, len(nums))
	for idx, v := range nums {
		for i := 2; i <= v; i++ {
			if _, ok := m[i]; ok && v%i == 0 {
				factor[idx] |= 1 << m[i]
			}
		}
	}
	egress := map[int][]int{}
	for _, v := range edges {
		egress[v[0]] = append(egress[v[0]], v[1])
		egress[v[1]] = append(egress[v[1]], v[0])
	}

	var recur func(int, int, *TH) (int, int)
	recur = func(target int, times int, aggr *TH) (int, int) {
		if times == 15 {
			if len(aggr.Level) == 0 {
				return 20000, -1
			}
			return aggr.Index[len(aggr.Index)-1], aggr.Level[len(aggr.Level)-1]
		}
		bit := target % 2
		index := 20000
		level := -1
		if aggr.Zero != nil {
			zeroIndex, zeroLevel := recur(target/2, times+1, aggr.Zero)
			if zeroLevel > level {
				index = zeroIndex
				level = zeroLevel
			}
		}
		if bit == 0 && aggr.One != nil {
			oneIndex, oneLevel := recur(target/2, times+1, aggr.One)
			if oneLevel > level {
				index = oneIndex
				level = oneLevel
			}
		}
		return index, level
	}

	var build func(int, int, int, int, *TH)
	build = func(target int, index int, level int, times int, aggr *TH) {
		if times == 15 {
			if len(aggr.Index) == 0 {
				aggr.Index = []int{}
				aggr.Level = []int{}
			}
			aggr.Index = append(aggr.Index, index)
			aggr.Level = append(aggr.Level, level)
			return
		}
		bit := target % 2
		if bit == 0 {
			if aggr.Zero == nil {
				aggr.Zero = &TH{}
			}
			build(target/2, index, level, times+1, aggr.Zero)
		} else {
			if aggr.One == nil {
				aggr.One = &TH{}
			}
			build(target/2, index, level, times+1, aggr.One)
		}
	}
	var clean func(int, int, *TH)
	clean = func(target int, times int, aggr *TH) {
		if times == 15 {
			aggr.Index = aggr.Index[:len(aggr.Index)-1]
			aggr.Level = aggr.Level[:len(aggr.Level)-1]
			return
		}
		bit := target % 2
		if bit == 0 {
			clean(target/2, times+1, aggr.Zero)
		} else {
			clean(target/2, times+1, aggr.One)
		}
	}

	var dfs func([]int, int, int, map[int][]int, *TH, *map[int]bool, *[]int)
	dfs = func(factor []int, index int, level int, egress map[int][]int,
		aggr *TH, visited *map[int]bool, ret *[]int) {
		(*visited)[index] = true

		copIndex, _ := recur(factor[index], 0, aggr)
		if copIndex == 20000 {
			(*ret)[index] = -1
		} else {
			(*ret)[index] = copIndex
		}

		build(factor[index], index, level, 0, aggr)

		for _, edge := range egress[index] {
			if (*visited)[edge] {
				continue
			}
			dfs(factor, edge, level+1, egress, aggr, visited, ret)
		}

		clean(factor[index], 0, aggr)
	}

	visited := map[int]bool{}
	aggr := &TH{}
	ret := make([]int, len(nums))
	for i := range ret {
		ret[i] = -1
	}
	dfs(factor, 0, 0, egress, aggr, &visited, &ret)
	return ret
}
