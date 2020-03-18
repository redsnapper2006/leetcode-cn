package main

func isRectangleOverlap(rec1 []int, rec2 []int) bool {
	x11, y11, x12, y12 := rec1[0], rec1[1], rec1[2], rec1[3]
	x21, y21, x22, y22 := rec2[0], rec2[1], rec2[2], rec2[3]

	return ((x12 <= x22 && x12 > x21) || (x22 <= x12 && x22 > x11)) && ((y12 <= y22 && y12 > y21) || (y22 <= y12 && y22 > y11))
}

func main() {
	fmt.Println("a")
}
