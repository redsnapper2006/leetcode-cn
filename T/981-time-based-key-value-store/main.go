package main

import (
	"fmt"
)

type Value struct {
	Timestamp []int
	V         []string
}
type TimeMap struct {
	M map[string]*Value
}

func Constructor() TimeMap {
	return TimeMap{M: make(map[string]*Value)}
}

func (this *TimeMap) Set(key string, value string, timestamp int) {
	v, ok := this.M[key]
	if !ok {
		this.M[key] = &Value{Timestamp: []int{timestamp}, V: []string{value}}
	} else {
		v.Timestamp = append(v.Timestamp, timestamp)
		v.V = append(v.V, value)
	}
}

func (this *TimeMap) Get(key string, timestamp int) string {
	v, ok := this.M[key]
	if !ok {
		return ""
	}
	s, e := 0, len(v.Timestamp)-1
	for s < e {
		mid := s + (e-s)/2
		if v.Timestamp[mid] >= timestamp {
			e = mid
		} else {
			s = mid + 1
		}
	}
	if v.Timestamp[s] > timestamp {
		if s == 0 {
			return ""
		}
		return v.V[s-1]
	}
	return v.V[s]
}

func main() {
	o := Constructor()
	o.Set("foo", "bar", 1)
	fmt.Println(o.Get("foo", 1))
	fmt.Println(o.Get("foo", 3))
	o.Set("foo", "bar2", 4)
	fmt.Println(o.Get("foo", 4))
	fmt.Println(o.Get("foo", 5))
	fmt.Println("a")
}
