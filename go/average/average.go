package average

func Average(scores []int) int {
	sum := 0
	for _, s := range scores {
		sum += s
	}
	return int(sum / len(scores))
}
