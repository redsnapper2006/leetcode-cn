package main

import "fmt"

func sumNums(n int) int {
	// n*(n+1)/2
	p1, p2 := n, n+1

	var n1, n2 int
	assign := func(n, p *int) bool {
		*n = *p
		return true
	}
	assign2 := func(n, p *int) bool {
		*n = (*p) >> 1
		return true
	}
	t1 := ((p1 & 1) == 1) && assign(&n1, &p1)
	t2 := ((p1 & 1) == 1) && assign2(&n2, &p2)
	t1 = !((p1 & 1) == 1) && assign2(&n2, &p1)
	t2 = !((p1 & 1) == 1) && assign(&n1, &p2)

	fmt.Println(n1, n2)
	shift := func(sum *int, n, offset int) bool {
		*sum += n << offset
		return true
	}

	sum := 0
	t1 = ((n2 & (1 << 13)) > 0) && shift(&sum, n1, 13)
	t1 = ((n2 & (1 << 12)) > 0) && shift(&sum, n1, 12)
	t1 = ((n2 & (1 << 11)) > 0) && shift(&sum, n1, 11)
	t1 = ((n2 & (1 << 10)) > 0) && shift(&sum, n1, 10)
	t1 = ((n2 & (1 << 9)) > 0) && shift(&sum, n1, 9)
	t1 = ((n2 & (1 << 8)) > 0) && shift(&sum, n1, 8)
	t1 = ((n2 & (1 << 7)) > 0) && shift(&sum, n1, 7)
	t1 = ((n2 & (1 << 6)) > 0) && shift(&sum, n1, 6)
	t1 = ((n2 & (1 << 5)) > 0) && shift(&sum, n1, 5)
	t1 = ((n2 & (1 << 4)) > 0) && shift(&sum, n1, 4)
	t1 = ((n2 & (1 << 3)) > 0) && shift(&sum, n1, 3)
	t1 = ((n2 & (1 << 2)) > 0) && shift(&sum, n1, 2)
	t1 = ((n2 & (1 << 1)) > 0) && shift(&sum, n1, 1)
	t1 = ((n2 & (1 << 0)) > 0) && shift(&sum, n1, 0)
	t1, t2 = t2, t1

	return sum
}

func main() {
	fmt.Println(sumNums(9))
}
