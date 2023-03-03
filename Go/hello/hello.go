package main

import (
	"fmt"
	"example.com/greetings"
)

func main() {
	message := greetings.Hello("Dominic")
	fmt.Println(message)
}