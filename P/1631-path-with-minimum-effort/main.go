package main

import "fmt"

func minimumEffortPath(heights [][]int) int {
	if len(heights) == 1 && len(heights[0]) == 1 {
		return 0
	}

	buf := make([][]int, len(heights))
	for i := 0; i < len(heights); i++ {
		buf[i] = make([]int, len(heights[0]))
		for j := 0; j < len(heights[0]); j++ {
			buf[i][j] = -1
		}
	}

	var dfs func(heights, buf [][]int, r, c, max int, candi *[][]int)
	dfs = func(heights, buf [][]int, r, c, max int, candi *[][]int) {
		if buf[r][c] == 10000000 {
			return
		}

		if r > 0 && buf[r-1][c] == -1 {
			distance := heights[r-1][c] - heights[r][c]
			if distance < 0 {
				distance = -distance
			}
			if distance <= max {
				buf[r-1][c] = max
				*candi = append(*candi, []int{r - 1, c, r, c})
				dfs(heights, buf, r-1, c, max, candi)
			}
		}
		if c > 0 && buf[r][c-1] == -1 {
			distance := heights[r][c-1] - heights[r][c]
			if distance < 0 {
				distance = -distance
			}
			if distance <= max {
				buf[r][c-1] = max
				*candi = append(*candi, []int{r, c - 1, r, c})
				dfs(heights, buf, r, c-1, max, candi)
			}
		}
		if r < len(heights)-1 && buf[r+1][c] == -1 {
			distance := heights[r+1][c] - heights[r][c]
			if distance < 0 {
				distance = -distance
			}
			if distance <= max {
				buf[r+1][c] = max
				*candi = append(*candi, []int{r + 1, c, r, c})
				dfs(heights, buf, r+1, c, max, candi)
			}
		}
		// fmt.Println(t, min)
		if c < len(heights[0])-1 && buf[r][c+1] == -1 {
			distance := heights[r][c+1] - heights[r][c]
			if distance < 0 {
				distance = -distance
			}
			if distance <= max {
				buf[r][c+1] = max
				*candi = append(*candi, []int{r, c + 1, r, c})
				dfs(heights, buf, r, c+1, max, candi)
			}
		}
	}
	visit := [][]int{{0, 0}}
	buf[0][0] = 0
	for {
		var t [][]int
		min := 10000000
		for i := 0; i < len(visit); i++ {
			r, c := visit[i][0], visit[i][1]
			if buf[r][c] == 10000000 {
				continue
			}
			isTarget := false
			if r > 0 && buf[r-1][c] == -1 {
				isTarget = true
				distance := heights[r-1][c] - heights[r][c]
				if distance < 0 {
					distance = -distance
				}
				if distance < min {
					min = distance
					t = [][]int{{r - 1, c, r, c}}
				} else if distance == min {
					t = append(t, []int{r - 1, c, r, c})
				}
			}
			if c > 0 && buf[r][c-1] == -1 {
				isTarget = true
				distance := heights[r][c-1] - heights[r][c]
				if distance < 0 {
					distance = -distance
				}
				if distance < min {
					min = distance
					t = [][]int{{r, c - 1, r, c}}
				} else if distance == min {
					t = append(t, []int{r, c - 1, r, c})
				}
			}
			if r < len(heights)-1 && buf[r+1][c] == -1 {
				isTarget = true
				distance := heights[r+1][c] - heights[r][c]
				if distance < 0 {
					distance = -distance
				}
				if distance < min {
					min = distance
					t = [][]int{{r + 1, c, r, c}}
				} else if distance == min {
					t = append(t, []int{r + 1, c, r, c})
				}
			}
			// fmt.Println(t, min)
			if c < len(heights[0])-1 && buf[r][c+1] == -1 {
				isTarget = true
				distance := heights[r][c+1] - heights[r][c]
				if distance < 0 {
					distance = -distance
				}
				if distance < min {
					min = distance
					t = [][]int{{r, c + 1, r, c}}
				} else if distance == min {
					t = append(t, []int{r, c + 1, r, c})
				}
			}
			if !isTarget {
				buf[r][c] = 10000000
			}
		}

		for i := 0; i < len(t); i++ {
			r, c, or, oc := t[i][0], t[i][1], t[i][2], t[i][3]
			if r == len(heights)-1 && c == len(heights[0])-1 {
				ret := buf[or][oc]
				if ret < min {
					ret = min
				}
				return ret
			}
			buf[r][c] = buf[or][oc]
			if buf[or][oc] < min {
				buf[r][c] = min
			}
			dfs(heights, buf, r, c, buf[r][c], &t)
		}

		if len(t) == 0 {
			break
		}
		visit = append(visit, t...)
	}
	return buf[len(heights)-1][len(heights[0])-1]
}

func main() {
	fmt.Println()
}
