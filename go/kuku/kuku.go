package main

import(
	"fmt"
)

func kuku() {
	for i:=1 ; i<=9 ; i++{
		for j:=1 ; j<=9 ; j++{
			fmt.Printf("%2d ", i * j)
		}
		fmt.Println()
	}
}

func main(){
	kuku()
}