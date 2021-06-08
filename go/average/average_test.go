package average_test

import (
	"github.com/hsmtkk/super-duper-octo-potato/go/average"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestAverage(t *testing.T){
	scores := []int{10, 16, 20, 21, 68}
	want := 27
	got := average.Average(scores)
	assert.Equal(t, want, got)
}