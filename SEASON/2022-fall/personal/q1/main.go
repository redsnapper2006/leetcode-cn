package main

func temperatureTrend(temperatureA []int, temperatureB []int) int {
	trendA, trendB := make([]int, len(temperatureA)-1), make([]int, len(temperatureB)-1)

	for i := 0; i < len(trendA); i++ {
		if temperatureA[i] < temperatureA[i+1] {
			trendA[i] = -1
		} else if temperatureA[i] > temperatureA[i+1] {
			trendA[i] = 1
		}
		if temperatureB[i] < temperatureB[i+1] {
			trendB[i] = -1
		} else if temperatureB[i] > temperatureB[i+1] {
			trendB[i] = 1
		}
	}

	cnt := 0
	max := 0
	for i := 0; i < len(trendA); i++ {
		if trendA[i] == trendB[i] {
			cnt++
		} else {
			if max < cnt {
				max = cnt
			}
			cnt = 0
		}
	}
	if max < cnt {
		max = cnt
	}
	return max
}
