package main

import (
    "flag"
    "fmt"
    "cli.com/packages/sharedstate"
)
func main() {
    // Declare and collect CLI args
    var examplenum = flag.Int("ex", 1, "Select the example number you wish to run") 
    flag.Parse()

    fmt.Printf("Running example %d\n", *examplenum)
    
    switch *examplenum {
        case 1:
            var res int = sharedstate.ProducersConsumers(5) 
            fmt.Printf("Result: %d\n", res)
        default:
            fmt.Printf("Error: Unknown Example %d\n", *examplenum)
            return
    }
    return
}   


