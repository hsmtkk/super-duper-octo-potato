package fibonacci_test

import (
	"testing"

	"github.com/hsmtkk/super-duper-octo-potato/go/fibonacci"
	"github.com/stretchr/testify/assert"
)

func TestFibonacci(t *testing.T) {
	want := 3736710778780434371
	got := fibonacci.New().Fibonacci(100)
	assert.Equal(t, want, got)
}
