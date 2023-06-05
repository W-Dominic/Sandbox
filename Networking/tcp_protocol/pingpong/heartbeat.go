package main

import(
    "fmt"
    "net"
    "time"
    "context"
    "io"
)

const defaultPingInterval = 10 * time.Second

func Pinger(ctx context.Context, w io.Writer, reset <-chan time.Duration) {
    var interval time.Duration
    // Create ping interval by reset channel
    select {
    case <- ctx.Done():
        return
    case interval = <-reset:
    default:
    }
    if interval <= 0 {
        interval = defaultPingInterval
    }
    
    // create timer with graceful termination
    // drains the timer before exititng
    timer := time.NewTimer(interval)
    defer func() {
        if !timer.Stop() {
            <-timer.C 
        }
    }()

    // main loop
    for {
        select {
        case <-ctx.Done():
            return
        case newInterval := <-reset:
            if !timer.Stop() {
                <-timer.C
            }
            if newInterval > 0 {
               interval = newInterval 
            }
        case <-timer.C:
            if _, err := w.Write([]byte("Ping")); err != nil {
                // can track consecutive timeouts here
                return
            }
        }
        _ = timer.Reset(interval)
    }
}
