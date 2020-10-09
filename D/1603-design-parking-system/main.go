package main

import "fmt"

type ParkingSystem struct {
	BM int
	MM int
	SM int
	B  int
	M  int
	S  int
}

func Constructor(big int, medium int, small int) ParkingSystem {
	return ParkingSystem{BM: big, MM: medium, SM: small}
}

func (this *ParkingSystem) AddCar(carType int) bool {
	if carType == 1 {
		if this.B < this.BM {
			this.B++
			return true
		}
		return false
	}
	if carType == 2 {
		if this.M < this.MM {
			this.M++
			return true
		}
		return false
	}

	if this.S < this.SM {
		this.S++
		return true
	}
	return false

}

func main() {
	fmt.Println("a")
}
