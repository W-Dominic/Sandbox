package main
import "fmt"

func zeroval(ival int){
    ival = 0
}

func zeroptr(iptr *int) {
    *iptr = 0
}

func main(){
    i := 1
    fmt.Println("initial:", i) // prints 1
    
    zeroval(i)
    fmt.Println("zeroval:", i) // prints 1
                               // pass by value

    zeroptr(&i)
    fmt.Println("zeroptr:", i) // prints 0
                               // pass by ref

    fmt.Println("pointer:", &i) // mem addr of i ptr
}
