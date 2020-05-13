package main

func maxChunksToSorted(arr []int) int {

	M := make(map[int]int)
	for i := 0; i < len(arr); i++ {
		M[arr[i]] = i
	}

	chunks := 0
	maxIdx := 0
	for i := 0; i < len(arr); i++ {
		if M[i] > maxIdx {
			maxIdx = M[i]
		}
		if i == maxIdx {
			chunks++
		}
	}
	return chunks
}

func main() {
	fmt.Println("a")
}
