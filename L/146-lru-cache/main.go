package main

type DLL struct {
	Key  int
	Val  int
	Prev *DLL
	Next *DLL
}

type LRUCache struct {
	M map[int]*DLL
	H *DLL
	E *DLL
	O int
	C int
}

func Constructor(capacity int) LRUCache {
	return LRUCache{M: make(map[int]*DLL), H: nil, E: nil, O: 0, C: capacity}
}

func (this *LRUCache) Get(key int) int {
	v, ok := this.M[key]
	if !ok {
		return -1
	}

	if this.E != v {
		prev := v.Prev
		next := v.Next
		if prev != nil {
			prev.Next = next
		}
		if next != nil {
			next.Prev = prev
		}

		if this.H == v {
			this.H = v.Next
		}
		this.E.Next = v
		v.Prev = this.E
		v.Next = nil
		this.E = v
	}

	return v.Val
}

func (this *LRUCache) Put(key int, value int) {
	_, ok := this.M[key]
	if ok {
		this.M[key].Val = value
		this.Get(key)
		return
	}
	if this.O < this.C {
		t := DLL{Key: key, Val: value}
		this.M[key] = &t
		if this.O == 0 {
			this.H = &t
		} else {
			t.Prev = this.E
			this.E.Next = &t
		}
		this.E = &t
		this.O++
	} else {
		h := this.H
		delete(this.M, h.Key)
		k := DLL{Key: key, Val: value}
		this.M[key] = &k
		if this.H != this.E {
			this.H = h.Next
			this.H.Prev = nil
			this.E.Next = &k
			k.Prev = this.E
			k.Next = nil
			this.E = &k
		} else {
			this.H = &k
			this.E = &k
		}
	}
}

func main() {
	// lru := Constructor(1)
	// lru.Put(2, 1)
	// lru.Get(2)
	// lru.Put(3, 2)
	// lru.Get(2)
	// lru.Get(3)

	lru := Constructor(2)
	lru.Put(2, 1)
	lru.Put(2, 2)
	lru.Get(2)
	lru.Put(1, 1)
	lru.Put(4, 1)
	lru.Get(2)
}
