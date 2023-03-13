package main
import "fmt"

type person struct {
    name string
    age int
}
func newPerson(name string) *person {
    p := person{name: name}
    p.age = 42
    return &p
}

func main() {
    fmt.Println(person{"Ya mudda", 99})
    fmt.Println(person{"Ur mom", 99})
    
    // create a struct instance and store referance in variable
    s := person{name: "Ya mudda", age: 99}
    fmt.Println(s.name)

    // reference to first struct
    sp := &s
    fmt.Println(sp.age)
    
    // changing the reference affects the original
    sp.age = 100
    fmt.Println(sp.age)
    fmt.Println(s.age)

    // changing the original affects the reference
    s.age = 101
    fmt.Println(sp.age)
    fmt.Println(s.age)
    
}   
