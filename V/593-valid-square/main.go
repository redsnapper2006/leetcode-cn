package main

func validSquare(p1 []int, p2 []int, p3 []int, p4 []int) bool {
	d12 := (p1[0]-p2[0])*(p1[0]-p2[0]) + (p1[1]-p2[1])*(p1[1]-p2[1])
	d13 := (p1[0]-p3[0])*(p1[0]-p3[0]) + (p1[1]-p3[1])*(p1[1]-p3[1])
	d14 := (p1[0]-p4[0])*(p1[0]-p4[0]) + (p1[1]-p4[1])*(p1[1]-p4[1])

	if d12 != d13 && d13 != d14 && d14 != d12 || d12 == d13 && d13 == d14 && d14 == d12 {
		return false
	}

	var distance int
	var singluar []int
	var pair1 []int
	var pair2 []int
	base := p1
	if d12 == d13 {
		distance = d12
		pair1, pair2 = p2, p3
		singluar = p4
	} else if d12 == d14 {
		distance = d12
		pair1, pair2 = p2, p4
		singluar = p3
	} else {
		distance = d13
		pair1, pair2 = p3, p4
		singluar = p2
	}

	if (pair1[0]-singluar[0])*(pair1[0]-singluar[0])+(pair1[1]-singluar[1])*(pair1[1]-singluar[1]) != distance || (pair2[0]-singluar[0])*(pair2[0]-singluar[0])+(pair2[1]-singluar[1])*(pair2[1]-singluar[1]) != distance {
		return false
	}

	if base[0] == pair1[0] && base[1] == pair2[1] || base[0] == pair2[0] && base[1] == pair1[1] {
		if (pair1[1] == singluar[1] && pair2[0] == singluar[0]) || (pair2[1] == singluar[1] || pair1[0] == singluar[0]) {
			return true
		}
		return false
	}

	kbp1 := float64(base[1]-pair1[1]) / float64(base[0]-pair1[0])
	kbp2 := float64(base[1]-pair2[1]) / float64(base[0]-pair2[0])
	ksp1 := float64(singluar[1]-pair1[1]) / float64(singluar[0]-pair1[0])
	ksp2 := float64(singluar[1]-pair2[1]) / float64(singluar[0]-pair2[0])
	if kbp1*kbp2+1 >= 0.00000001 {
		return false
	}

	if kbp1 != ksp2 || kbp2 != ksp1 {
		return false
	}
	return true
}
