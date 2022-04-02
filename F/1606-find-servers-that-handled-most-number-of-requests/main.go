package main

import (
	"container/heap"
	"fmt"
	"sort"
)

func busiestServers2(k int, arrival []int, load []int) []int {
	max := 0
	start := map[int]int{}
	end := map[int][]int{}
	guide := []int{}
	for i := 0; i < len(arrival); i++ {
		if max < arrival[i]+load[i] {
			max = arrival[i] + load[i]
		}
		start[arrival[i]] = i
		guide = append(guide, arrival[i], arrival[i]+load[i])
	}
	sort.Ints(guide)

	free := map[int]int{}
	for i := 0; i < k; i++ {
		free[i] = 1
	}

	busy := map[int]int{}
	// task := map[int]int{}
	for j := 0; j < len(guide); j++ {
		if j > 0 && guide[j] == guide[j-1] {
			continue
		}
		i := guide[j]
		rel, ok := end[i]
		if ok {
			for _, r := range rel {
				free[r] = 1
			}
		}

		arrIdx, ok := start[i]
		if !ok {
			continue
		}
		if len(free) == 0 {
			// fmt.Println("no available", arrIdx)
			continue
		}
		server := arrIdx % k
		// fmt.Println(arrIdx, free)
		for {
			_, ok := free[server]
			if ok {
				break
			}
			server++
			server %= k
		}
		// fmt.Println(server)
		delete(free, server)
		busy[server]++
		// task[server] = arrIdx
		_, ok2 := end[arrival[arrIdx]+load[arrIdx]]
		if !ok2 {
			end[arrival[arrIdx]+load[arrIdx]] = []int{}
		}
		end[arrival[arrIdx]+load[arrIdx]] = append(end[arrival[arrIdx]+load[arrIdx]], server)
	}

	maxBusy := 0
	ret := []int{}
	for k, v := range busy {
		if v > maxBusy {
			maxBusy = v
			ret = []int{}
			ret = append(ret, k)
		} else if v == maxBusy {
			ret = append(ret, k)
		}
	}

	return ret
}

func busiestServers(k int, arrival, load []int) (ans []int) {
	available := hi{make([]int, k)}
	for i := 0; i < k; i++ {
		available.IntSlice[i] = i
	}
	busy := hp{}
	requests := make([]int, k)
	maxRequest := 0
	for i, t := range arrival {
		for len(busy) > 0 && busy[0].end <= t {
			heap.Push(&available, i+((busy[0].id-i)%k+k)%k) // 保证得到的是一个不小于 i 的且与 id 同余的数
			heap.Pop(&busy)
		}
		if available.Len() == 0 {
			continue
		}
		id := heap.Pop(&available).(int) % k
		requests[id]++
		if requests[id] > maxRequest {
			maxRequest = requests[id]
			ans = []int{id}
		} else if requests[id] == maxRequest {
			ans = append(ans, id)
		}
		heap.Push(&busy, pair{t + load[i], id})
	}
	return
}

type hi struct{ sort.IntSlice }

func (h *hi) Push(v interface{}) { h.IntSlice = append(h.IntSlice, v.(int)) }
func (h *hi) Pop() interface{} {
	a := h.IntSlice
	v := a[len(a)-1]
	h.IntSlice = a[:len(a)-1]
	return v
}

type pair struct{ end, id int }
type hp []pair

func (h hp) Len() int            { return len(h) }
func (h hp) Less(i, j int) bool  { return h[i].end < h[j].end }
func (h hp) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v interface{}) { *h = append(*h, v.(pair)) }
func (h *hp) Pop() interface{}   { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }

func main() {
	fmt.Println(busiestServers(3, []int{1, 2, 3, 4, 5}, []int{5, 2, 3, 3, 3}))

	fmt.Println(busiestServers(3, []int{1, 2, 3, 4}, []int{1, 2, 1, 2}))

	fmt.Println(busiestServers(3, []int{1, 2, 3}, []int{10, 12, 11}))

	fmt.Println(busiestServers(3, []int{1, 2, 3, 4, 8, 9, 10}, []int{5, 2, 10, 3, 1, 2, 2}))

	fmt.Println(busiestServers(1, []int{1}, []int{1}))
}
