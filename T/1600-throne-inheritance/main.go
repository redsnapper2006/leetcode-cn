package main

import "fmt"

type ThroneInheritance struct {
	K string
	M map[string]*Throne
	D map[string]int
}

type Throne struct {
	N    string
	TArr []*Throne
}

func Constructor(kingName string) ThroneInheritance {
	t := Throne{N: kingName, TArr: []*Throne{}}
	return ThroneInheritance{K: kingName, M: map[string]*Throne{kingName: &t}, D: map[string]int{}}
}

func (this *ThroneInheritance) Birth(parentName string, childName string) {
	t := Throne{N: childName, TArr: []*Throne{}}
	this.M[childName] = &t
	this.M[parentName].TArr = append(this.M[parentName].TArr, &t)
}

func (this *ThroneInheritance) Death(name string) {
	this.D[name]++
}

func (this *ThroneInheritance) GetInheritanceOrder() []string {
	var recur func(p *Throne, b *[]string)
	recur = func(p *Throne, b *[]string) {
		if _, ok := this.D[p.N]; !ok {
			*b = append(*b, p.N)
		}
		for _, v := range p.TArr {
			recur(v, b)
		}
	}
	buf := []string{}
	recur(this.M[this.K], &buf)
	return buf
}

func main() {
	fmt.Println()
}
