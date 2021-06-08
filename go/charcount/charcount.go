package charcount

func CountChar(text string) map[string]int {
	result := map[string]int{}
	for _, ch := range(text) {
		s := string(ch)
		if s == "\n" {
			continue
		}
		_, ok := result[s]
		if ok {
			result[s] += 1
		} else {
			result[s] = 1
		}
	}
	return result
}