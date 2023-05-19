package main

import (
    "flag"
    "fmt"
)
func main() {
    // Declare and collect CLI args
    var examplenum = flag.Int("ex", 1, "Select the example number you wish to run") 
    flag.Parse()

    fmt.Printf("Running example %d\n", *examplenum)
    return
}   


