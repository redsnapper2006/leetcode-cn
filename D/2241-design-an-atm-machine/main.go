package main

type ATM struct {
	D []int
	V []int
}

func Constructor() ATM {
	return ATM{D: make([]int, 5), V: []int{20, 50, 100, 200, 500}}
}

func (this *ATM) Deposit(banknotesCount []int) {
	for i := 0; i < len(banknotesCount); i++ {
		this.D[i] += banknotesCount[i]
	}
}

func (this *ATM) Withdraw(amount int) []int {
	ret := make([]int, 5)
	remain := amount

	for i := len(this.V) - 1; i >= 0; i-- {
		if this.D[i] == 0 {
			continue
		}
		if remain < this.V[i] {
			continue
		}
		div := remain / this.V[i]
		if div > this.D[i] {
			div = this.D[i]
		}
		ret[i] = div
		remain -= div * this.V[i]
		if remain == 0 {
			break
		}
	}
	if remain > 0 {
		return []int{-1}
	}

	for i := 0; i < len(ret); i++ {
		this.D[i] -= ret[i]
	}
	return ret
}
