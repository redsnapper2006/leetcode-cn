package main

type Cashier struct {
	N int
	D int
	P map[int]int
	C int
}

func Constructor(n int, discount int, products []int, prices []int) Cashier {
	m := map[int]int{}
	for i := 0; i < len(products); i++ {
		m[products[i]] = prices[i]
	}
	return Cashier{N: n, D: discount, P: m}
}

func (this *Cashier) GetBill(product []int, amount []int) float64 {
	this.C++

	total := 0
	for i := 0; i < len(product); i++ {
		total += this.P[product[i]] * amount[i]
	}
	sum := float64(total)
	if this.C == this.N {
		sum = sum * float64(100-this.D) / 100
		this.C = 0
	}
	return sum
}

func main() {

}
