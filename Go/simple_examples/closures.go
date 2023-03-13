package main
import "fmt"

// function which returns a function which returns an int
func intSeq() func() int {
    i := 0
    return func() int {
        i ++
        return i
    }
}

func main() {
    nextInt := intSeq()
    for i := 0; i<10; i++ {
        fmt.Println(nextInt())
    }

    newInts := intSeq()
    fmt.Println(newInts())
}

