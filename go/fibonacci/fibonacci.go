package fibonacci

type Calculator interface {
	Fibonacci(n int) int64
}

type calculatorImpl struct {
	memo map[int]int64
}

func New() Calculator {
	memo := map[int]int64{}
	return &calculatorImpl{memo: memo}
}

func (c *calculatorImpl) Fibonacci(n int) int64 {
	if n <= 2 {
		return 1
	}
	val, ok := c.memo[n]
	if ok {
		return val
	} else {
		result := c.Fibonacci(n-1) + c.Fibonacci(n-2)
		c.memo[n] = result
		return result
	}
}
