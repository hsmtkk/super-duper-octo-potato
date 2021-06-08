package calctotal_test

import (
	"github.com/hsmtkk/super-duper-octo-potato/go/calctotal"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestCalculate(t *testing.T) {
	c := calctotal.New()
	want := 900
	got := c.Calculate(500)
	assert.Equal(t, want, got)
	want = 3300
	got = c.Calculate(3000)
	assert.Equal(t, want, got)
}
