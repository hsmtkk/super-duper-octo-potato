package calctotal

type Calculator interface {
	Calculate(price int) int
}

type calculatorImpl struct{}

func New() Calculator {
	return &calculatorImpl{}
}

func (c *calculatorImpl) Calculate(price int) int {
	taxed := float64(price) * 1.1
	total := taxed
	if taxed < 2000 {
		total += 350
	}
	return int(total)
}
