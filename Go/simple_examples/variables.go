package main
import ( 
    "fmt"
    "math"
)

const s string = "constant string"

func main() {
    // variables
    var a = "initial"
    fmt.Println(a)

    var b, c int = 1, 2
    fmt.Println(b,c)

    // initialized ints start at 0
    var d int
    fmt.Println(d)

    // short hand for delcaring and initializing (:=)
    e := "ya mudda"
    fmt.Println(e)

    // constants
    const x = 500000000000
    const y = 3e20 / x 
    fmt.Println(y)
    fmt.Println(int64(y)) // cast to int64
    fmt.Println(int32(y)) // cast to int32
    fmt.Println(math.Sin(y)) // math.sin expects float64
}
