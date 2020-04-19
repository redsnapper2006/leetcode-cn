package main

import (
	"fmt"
	"sort"
	"strconv"
)

func displayTable(orders [][]string) [][]string {
	M := make(map[string]map[string]int)
	MOrder := make(map[string]int)
	MTable := make(map[string]int)

	for i := 0; i < len(orders); i++ {
		table, order := orders[i][1], orders[i][2]
		MTable[table]++
		MOrder[order]++
		_, ok := M[table]
		if !ok {
			t := make(map[string]int)
			t[order]++
			M[table] = t
		} else {
			M[table][order]++
		}
	}
	TableList := make([]int, len(MTable))
	idx := 0
	for k, _ := range MTable {
		v, _ := strconv.ParseInt(k, 10, 32)
		TableList[idx] = int(v)
		idx++
	}
	sort.Ints(TableList)
	OrderList := make([]string, len(MOrder))
	idx = 0
	for k, _ := range MOrder {
		OrderList[idx] = k
		idx++
	}
	sort.Strings(OrderList)

	H := []string{"Table"}
	H = append(H, OrderList...)

	ret := [][]string{H}
	for i := 0; i < len(TableList); i++ {
		tbl := TableList[i]
		sTbl := strconv.FormatInt(int64(tbl), 10)
		v := M[sTbl]
		r := []string{sTbl}
		for i := 1; i < len(H); i++ {
			r = append(r, strconv.FormatInt(int64(v[H[i]]), 10))
		}
		ret = append(ret, r)
	}

	return ret
}

func main() {
	fmt.Println("a")
}
