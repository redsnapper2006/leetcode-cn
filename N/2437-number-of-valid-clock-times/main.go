package main

func countTime(time string) int {
	hour := 1
	minutes := 1
	if time[0] == '?' && time[1] == '?' {
		hour = 24
	} else if time[0] == '?' {
		if time[1] <= '3' && time[1] >= '0' {
			hour = 3
		} else {
			hour = 2
		}
	} else if time[1] == '?' {
		if time[0] == '2' {
			hour = 4
		} else {
			hour = 10
		}
	}

	if time[3] == '?' {
		minutes *= 6
	}
	if time[4] == '?' {
		minutes *= 10
	}
	return hour * minutes
}

func main() {

}
