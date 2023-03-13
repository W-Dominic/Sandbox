package main
import "fmt"

func main(){
    // initialize empty int array of size 5
    var a [5]int
    fmt.Println("empty array:", a)

    // set and get values
    a[4] = 100
    fmt.Println("set:", a)
    fmt.Println("get:", a[4])
    
    // length of array
    fmt.Println("len:", len(a))
    
    // string array
    var b [3]string
    b[1] = "hello there"
    b[2] = "oh, h3llo"
    b[0] = "hey"
    fmt.Println("string array:", b)

    // delcare and initialize array
    c := [2]float64{300.3, 2}
    fmt.Println("float array:", c)
    
    // 2d array
    var twoD [2][3]int
    rows := len(twoD)
    cols := len(twoD[0])
    fmt.Printf("2d array of %d rows and %d cols\n", rows, cols)
    for i := 0; i < rows; i++ {
        for j := 0; j < cols; j++ {
            twoD[i][j] = 0
        }
    }
    fmt.Println("2d array: ", twoD)
}
