package main

import(
    "fmt"
    "time"
    "context"
    "io"
)

func main() {
    ctx, cancel := context.WithCancel(context.Background())
    r, w := io.Pipe()
    done := make(chan struct{})
    resetTimer := make(chan time.Duration, 1)
    resetTimer <- time.Second //initial ping interval

    go func() {
        Pinger(ctx, w, resetTimer)
        close(done)
    }()

    receivePing := func(d time.Duration, r io.Reader) {
        if d >= 0 {
            fmt.Printf("reseting timer (%s)\n", d)
            resetTimer <- d
        }

        now := time.Now()
        buf := make([]byte, 1024)
        n, err := r.Read(buf)
        if err != nil {
            fmt.Println(err)
        }
        fmt.Printf("received %q (%s)\n", buf[:n], time.Since(now).Round(100*time.Millisecond))
    }

    for i, v := range []int64{0,200,500,0,-1,-1,-1} {
        fmt.Printf("Run %d:\n", i)
        receivePing(time.Duration(v)*time.Millisecond, r)
    }

    cancel()
    <-done // ensure that the pinger exits after the context is cancelled
}

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
