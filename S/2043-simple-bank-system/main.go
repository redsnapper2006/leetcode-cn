package main

import "fmt"

type Bank struct {
	V []int64
}

func Constructor(balance []int64) Bank {
	return Bank{V: balance}
}

func (this *Bank) Transfer(account1 int, account2 int, money int64) bool {
	if account1 < 1 || account1 > len(this.V) ||
		account2 < 1 || account2 > len(this.V) ||
		this.V[account1-1] < money {
		return false
	}

	this.V[account1-1] -= money
	this.V[account2-1] += money
	return true
}

func (this *Bank) Deposit(account int, money int64) bool {
	if account < 1 || account > len(this.V) {
		return false
	}

	this.V[account-1] += money
	return true
}

func (this *Bank) Withdraw(account int, money int64) bool {
	if account < 1 || account > len(this.V) || this.V[account-1] < money {
		return false
	}
	this.V[account-1] -= money
	return true
}

func main() {
	fmt.Println()
}
