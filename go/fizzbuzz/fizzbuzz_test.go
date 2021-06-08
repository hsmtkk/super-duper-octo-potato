package fizzbuzz_test

import (
	"github.com/hsmtkk/super-duper-octo-potato/go/fizzbuzz"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestFizzBuzz(t *testing.T){
	assert.Equal(t, "1", fizzbuzz.FizzBuzz(1))
	assert.Equal(t, "Fizz", fizzbuzz.FizzBuzz(3))
	assert.Equal(t, "Buzz", fizzbuzz.FizzBuzz(5))
	assert.Equal(t, "FizzBuzz", fizzbuzz.FizzBuzz(15))
}
