package main

import (
    "fmt"
    "net"
)
func main() {
    // Open a listener on a random port, and wait for a connection
    listener, err := net.Listen("tcp", "127.0.0.1:")
    if err != nil {
        fmt.Errorf("Error: %w\n", err)
        return
    }
    done := make(chan struct{})
    go func() {
        conn, err := listener.Accept()
        if err != nil {
            fmt.Errorf("Error: %w\n", err)
            return
        }
        defer conn.Close()

        // Read any ping messages coming in
        buf := make([]byte, 1024)
        n, err := conn.Read(buf)
        if err != nil {
            fmt.Errorf("Error: %w\n", err)
            return
        }
        fmt.Printf("Recieved on %s: %s\n", listener.Addr().String(), buf[:n]) 
        done <- struct{}{}
    }()
    
    // Initiate Connnection to the listner
    conn, err := net.Dial("tcp", listener.Addr().String())
    if err != nil {
        fmt.Errorf("Error: %w\n", err)
        return
    }
    defer conn.Close()
    
    // Write Ping to the Listener
    n, err := conn.Write([]byte("Ping!!!"))
    fmt.Printf("Sent %v bytes to %s\v\n", n, listener.Addr().String())
    if err != nil { 
        fmt.Errorf("Error: %w\n", err)
        return
    }
    
    <-done
    listener.Close()

}
