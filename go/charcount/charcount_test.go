package charcount_test

import (
	"os"
	"github.com/hsmtkk/super-duper-octo-potato/go/charcount"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestCountChar(t *testing.T){
	text, err := os.ReadFile("./example.txt")
	assert.Nil(t, err)
	want := map[string]int{"a":4, "b":1, "c":1, "e":1, "h":2, "i":1, "l":2, "o":1, "p":1, "r":2, "v":1}
	got := charcount.CountChar(string(text))
	assert.Equal(t, want, got)
}
