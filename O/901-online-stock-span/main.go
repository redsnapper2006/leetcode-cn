package main

type StockSpanner struct {
	NS []int
	IS []int
	C  int
}

func Constructor() StockSpanner {
	return StockSpanner{NS: []int{}, IS: []int{}}
}

func (this *StockSpanner) Next(price int) int {
	this.C++
	if len(this.NS) == 0 {
		this.NS = append(this.NS, price)
		this.IS = append(this.IS, this.C)
		return 1
	}

	idx := len(this.NS) - 1
	for idx >= 0 && this.NS[idx] <= price {
		idx--
	}

	v := this.C
	if idx >= 0 {
		v = this.C - this.IS[idx]
	}

	this.NS = this.NS[0 : idx+1]
	this.IS = this.IS[0 : idx+1]
	this.NS = append(this.NS, price)
	this.IS = append(this.IS, this.C)
	return v
}
