package sharedstate

import (
    "fmt"
    "time"
)
func producer(resources *int) {
    for {
        if (*resources > 9) {
            break
        }
        *resources += 1
        fmt.Println("Produced")
    }
    return
}

func consumer(resources *int) {
    for {
        if *resources > 0 {
            *resources -- 
            fmt.Println("Consumed")
        } else {
            break
        }
    }
    return
}

func ProducersConsumers(resources int) int{
    go producer(&resources)
    go consumer(&resources)
    go consumer(&resources)
    go consumer(&resources)
    go consumer(&resources)

    time.Sleep(2 * time.Second)
    return resources
}
