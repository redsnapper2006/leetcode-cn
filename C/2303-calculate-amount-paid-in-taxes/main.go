package main

func calculateTax(brackets [][]int, income int) float64 {
	prev := 0

	var tax float64 = 0.0
	for i := 0; i < len(brackets); i++ {
		if brackets[i][0] >= income {
			break
		}
		remain := brackets[i][0] - prev
		if remain > income-prev {
			remain = income - prev
		}
		prev = brackets[i][0]
		tax += float64(remain*brackets[i][1]) / 100
	}
	return tax
}
