package main
import "fmt"

func main() {
    i := 1
    for i <= 3 {
        fmt.Println(i)
        i += 1
    }

    for j := 0; j <= 2; j++ {
        fmt.Println(j)
    }
    // supports break, continue 
}
