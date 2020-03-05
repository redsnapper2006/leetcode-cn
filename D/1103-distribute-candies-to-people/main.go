package main

func distributeCandies(candies int, num_people int) []int {
	r := make([]int, num_people)

	s := 0
	i := 0
	for {
		s++
		if s*(s+1)/2 > candies {
			r[i] += candies - s*(s-1)/2
			break
		} else {
			r[i] += s
		}
		i = (i + 1) % num_people
	}
	return r
}

func main() {

}
