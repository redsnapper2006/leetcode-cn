package main

import "fmt"

func regionsBySlashes(grid []string) int {
	buf := make([][]int, len(grid))
	for i, g := range grid {
		buf[i] = []int{}
		for _, b := range g {
			if b == '\\' {
				buf[i] = append(buf[i], 2)
			} else {
				if b == '/' {
					buf[i] = append(buf[i], 1)
				} else {
					buf[i] = append(buf[i], 0)
				}
			}
		}
	}
	table := make([][][]int, len(grid))
	for i := range buf {
		table[i] = make([][]int, len(buf[i]))
		for j := range table[i] {
			table[i][j] = []int{0, 0, 0, 0}
		}
	}
	var dfs func(table [][][]int, buf [][]int, i, j, si, sj int, cnt int)
	dfs = func(table [][][]int, buf [][]int, i, j, si, sj int, cnt int) {
		// fmt.Println(i,j,si,sj, buf,table)
		if buf[i][j] == 0 && table[i][j][0] == 0 {
			table[i][j] = []int{cnt, cnt, cnt, cnt}

			if j > 0 && ((table[i][j-1][0] == 0 && buf[i][j-1] == 0) || (table[i][j-1][2] == 0 && buf[i][j-1] == 1) || (table[i][j-1][1] == 0 && buf[i][j-1] == 2)) {
				dfs(table, buf, i, j-1, i, j, cnt)
			}
			if j < len(table[i])-1 && ((table[i][j+1][0] == 0 && buf[i][j+1] == 0) || (table[i][j+1][0] == 0 && buf[i][j+1] == 1) || (table[i][j+1][3] == 0 && buf[i][j+1] == 2)) {
				dfs(table, buf, i, j+1, i, j, cnt)
			}
			if i > 0 && ((table[i-1][j][0] == 0 && buf[i-1][j] == 0) || (table[i-1][j][2] == 0 && buf[i-1][j] == 1) || (table[i-1][j][3] == 0 && buf[i-1][j] == 2)) {
				dfs(table, buf, i-1, j, i, j, cnt)
			}
			if i < len(table)-1 && ((table[i+1][j][0] == 0 && buf[i+1][j] == 0) || (table[i+1][j][0] == 0 && buf[i+1][j] == 1) || (table[i+1][j][1] == 0 && buf[i+1][j] == 2)) {
				dfs(table, buf, i+1, j, i, j, cnt)
			}
		} else if buf[i][j] == 1 && (table[i][j][0] == 0 || table[i][j][2] == 0) {
			// table[0][0] = []int{1, 0, 2, 0}
			if table[i][j][0] == 0 && (j-sj == 1 || i-si == 1) {
				table[i][j][0] = cnt
				if j > 0 && ((table[i][j-1][0] == 0 && buf[i][j-1] == 0) || (table[i][j-1][2] == 0 && buf[i][j-1] == 1) || (table[i][j-1][1] == 0 && buf[i][j-1] == 2)) {
					dfs(table, buf, i, j-1, i, j, cnt)
				}
				if i > 0 && ((table[i-1][j][0] == 0 && buf[i-1][j] == 0) || (table[i-1][j][2] == 0 && buf[i-1][j] == 1) || (table[i-1][j][3] == 0 && buf[i-1][j] == 2)) {
					dfs(table, buf, i-1, j, i, j, cnt)
				}
			}
			if table[i][j][2] == 0 && (j+1 == sj || i+1 == si) {
				table[i][j][2] = cnt
				if j < len(table[i])-1 && ((table[i][j+1][0] == 0 && buf[i][j+1] == 0) || (table[i][j+1][0] == 0 && buf[i][j+1] == 1) || (table[i][j+1][3] == 0 && buf[i][j+1] == 2)) {
					dfs(table, buf, i, j+1, i, j, cnt)
				}

				if i < len(table)-1 && ((table[i+1][j][0] == 0 && buf[i+1][j] == 0) || (table[i+1][j][0] == 0 && buf[i+1][j] == 1) || (table[i+1][j][1] == 0 && buf[i+1][j] == 2)) {
					dfs(table, buf, i+1, j, i, j, cnt)
				}
			}
		} else if buf[i][j] == 2 && (table[i][j][1] == 0 || table[i][j][3] == 0) {
			if table[i][j][1] == 0 && (i-si == 1 || j+1 == sj) {
				table[i][j][1] = cnt
				if j < len(table[i])-1 && ((table[i][j+1][0] == 0 && buf[i][j+1] == 0) || (table[i][j+1][0] == 0 && buf[i][j+1] == 1) || (table[i][j+1][3] == 0 && buf[i][j+1] == 2)) {
					dfs(table, buf, i, j+1, i, j, cnt)
				}
				if i > 0 && ((table[i-1][j][0] == 0 && buf[i-1][j] == 0) || (table[i-1][j][2] == 0 && buf[i-1][j] == 1) || (table[i-1][j][3] == 0 && buf[i-1][j] == 2)) {
					dfs(table, buf, i-1, j, i, j, cnt)
				}
			}
			if table[i][j][3] == 0 && (j-sj == 1 || i+1 == si) {
				table[i][j][3] = cnt
				if j > 0 && ((table[i][j-1][0] == 0 && buf[i][j-1] == 0) || (table[i][j-1][2] == 0 && buf[i][j-1] == 1) || (table[i][j-1][1] == 0 && buf[i][j-1] == 2)) {
					dfs(table, buf, i, j-1, i, j, cnt)
				}
				if i < len(table)-1 && ((table[i+1][j][0] == 0 && buf[i+1][j] == 0) || (table[i+1][j][0] == 0 && buf[i+1][j] == 1) || (table[i+1][j][1] == 0 && buf[i+1][j] == 2)) {
					dfs(table, buf, i+1, j, i, j, cnt)
				}
			}
		}
	}

	cnt := 0
	for i := range table {
		for j := range table[i] {
			if buf[i][j] == 0 && table[i][j][0] == 0 {
				cnt++
				table[i][j] = []int{cnt, cnt, cnt, cnt}

				if j > 0 && ((table[i][j-1][0] == 0 && buf[i][j-1] == 0) || (table[i][j-1][2] == 0 && buf[i][j-1] == 1) || (table[i][j-1][1] == 0 && buf[i][j-1] == 2)) {
					dfs(table, buf, i, j-1, i, j, cnt)
				}
				if j < len(table[i])-1 && ((table[i][j+1][0] == 0 && buf[i][j+1] == 0) || (table[i][j+1][0] == 0 && buf[i][j+1] == 1) || (table[i][j+1][3] == 0 && buf[i][j+1] == 2)) {
					dfs(table, buf, i, j+1, i, j, cnt)
				}
				if i > 0 && ((table[i-1][j][0] == 0 && buf[i-1][j] == 0) || (table[i-1][j][2] == 0 && buf[i-1][j] == 1) || (table[i-1][j][3] == 0 && buf[i-1][j] == 2)) {
					dfs(table, buf, i-1, j, i, j, cnt)
				}
				if i < len(table)-1 && ((table[i+1][j][0] == 0 && buf[i+1][j] == 0) || (table[i+1][j][0] == 0 && buf[i+1][j] == 1) || (table[i+1][j][1] == 0 && buf[i+1][j] == 2)) {
					dfs(table, buf, i+1, j, i, j, cnt)
				}
			} else if buf[i][j] == 1 && (table[i][j][0] == 0 || table[i][j][2] == 0) {
				// table[0][0] = []int{1, 0, 2, 0}
				if table[i][j][0] == 0 {
					cnt++
					table[i][j][0] = cnt
					if j > 0 && ((table[i][j-1][0] == 0 && buf[i][j-1] == 0) || (table[i][j-1][2] == 0 && buf[i][j-1] == 1) || (table[i][j-1][1] == 0 && buf[i][j-1] == 2)) {
						dfs(table, buf, i, j-1, i, j, cnt)
					}
					if i > 0 && ((table[i-1][j][0] == 0 && buf[i-1][j] == 0) || (table[i-1][j][2] == 0 && buf[i-1][j] == 1) || (table[i-1][j][3] == 0 && buf[i-1][j] == 2)) {
						dfs(table, buf, i-1, j, i, j, cnt)
					}
				}
				if table[i][j][2] == 0 {
					cnt++
					table[i][j][2] = cnt
					if j < len(table[i])-1 && ((table[i][j+1][0] == 0 && buf[i][j+1] == 0) || (table[i][j+1][0] == 0 && buf[i][j+1] == 1) || (table[i][j+1][3] == 0 && buf[i][j+1] == 2)) {
						dfs(table, buf, i, j+1, i, j, cnt)
					}

					if i < len(table)-1 && ((table[i+1][j][0] == 0 && buf[i+1][j] == 0) || (table[i+1][j][0] == 0 && buf[i+1][j] == 1) || (table[i+1][j][1] == 0 && buf[i+1][j] == 2)) {
						dfs(table, buf, i+1, j, i, j, cnt)
					}
				}
			} else if buf[i][j] == 2 && (table[i][j][1] == 0 || table[i][j][3] == 0) {
				if table[i][j][1] == 0 {
					cnt++
					table[i][j][1] = cnt
					if j < len(table[i])-1 && ((table[i][j+1][0] == 0 && buf[i][j+1] == 0) || (table[i][j+1][0] == 0 && buf[i][j+1] == 1) || (table[i][j+1][3] == 0 && buf[i][j+1] == 2)) {
						dfs(table, buf, i, j+1, i, j, cnt)
					}
					if i > 0 && ((table[i-1][j][0] == 0 && buf[i-1][j] == 0) || (table[i-1][j][2] == 0 && buf[i-1][j] == 1) || (table[i-1][j][3] == 0 && buf[i-1][j] == 2)) {
						dfs(table, buf, i-1, j, i, j, cnt)
					}
				}
				if table[i][j][3] == 0 {
					cnt++
					table[i][j][3] = cnt
					if j > 0 && ((table[i][j-1][0] == 0 && buf[i][j-1] == 0) || (table[i][j-1][2] == 0 && buf[i][j-1] == 1) || (table[i][j-1][1] == 0 && buf[i][j-1] == 2)) {
						dfs(table, buf, i, j-1, i, j, cnt)
					}
					if i < len(table)-1 && ((table[i+1][j][0] == 0 && buf[i+1][j] == 0) || (table[i+1][j][0] == 0 && buf[i+1][j] == 1) || (table[i+1][j][1] == 0 && buf[i+1][j] == 2)) {
						dfs(table, buf, i+1, j, i, j, cnt)
					}
				}
			}
		}
	}
	// fmt.Println("final", buf, table)
	return cnt
}

func main() {
	fmt.Println()
}
