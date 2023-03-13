package main
import (
    "fmt"
    "math/rand"
    "time"
)

func randInt(min, max int) int {
    return min + rand.Intn(max-min)
}

func main(){
    // random even/odd using if-else
    rand.Seed(time.Now().UnixNano())
    
    if n := randInt(5,10); n%2 == 0 {
        fmt.Printf("%d is even\n", n)
    } else {
        fmt.Printf("%d is odd\n", n)
    }

    // switch case example
    m := randInt(5,10)
    res := m%2
    switch res{
        case 0: 
            fmt.Printf("%d is even\n", m)
        default: 
            fmt.Printf("%d is odd\n", m)
    }
    
    // switch on type
    whatAmI := func(i interface{}) {
        switch t := i.(type) {
        case bool:
            fmt.Println("I'm a bool")
        case int:
            fmt.Println("I'm an int")
        default:
            fmt.Printf("Don't know type %T\n", t)
        }
    }
    whatAmI(true)
    whatAmI(1)
    whatAmI("hey")
}
