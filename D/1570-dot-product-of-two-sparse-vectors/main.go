package main

type SparseVector struct {
	N int
	M map[int]int
}

func Constructor(nums []int) SparseVector {
	m := map[int]int{}
	for i, v := range nums {
		if v == 0 {
			continue
		}
		m[i] = v
	}
	return SparseVector{M: m, N: len(nums)}
}

// Return the dotProduct of two sparse vectors
func (this *SparseVector) dotProduct(vec SparseVector) int {
	sum := 0
	for i := 0; i < this.N; i++ {
		v1, ok := this.M[i]
		v2, ok2 := vec.M[i]
		if ok && ok2 {
			sum += v1 * v2
		}
	}
	return sum
}

/**
 * Your SparseVector object will be instantiated and called as such:
 * v1 := Constructor(nums1);
 * v2 := Constructor(nums2);
 * ans := v1.dotProduct(v2);
 */
