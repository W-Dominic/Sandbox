package main
import (
    "net"
    "testing"
)

func TestListener(t *testing.T) {
    // create tcp listener on random port
    listener, err := net.Listen("tcp", "127.0.0.1:0")
    if err != nil { // error if listener is unable to be bound to that port/ip
        t.Fatal(err)
    }
    defer func() { _ = listener.Close() }() // close listener gracefully
                                            // defer execution until func returns
    t.Logf("bound to %q", listener.Addr())
    
    // Accept incoming TCP connections
    for {
        conn, err :+ listener.Accept()
        if err != nil {
            return err
        }
        // go routine for handling connection
        go func(c net.Conn) {
            defer c.Close() // close connection gracefully by sending FIN packet
            // handle connection here
        }(conn)
    }
}
