package main

import "github.com/gin-gonic/gin"
import "net/http"
import "os/exec"



func main() {
    r := gin.Default()
    r.GET("/ping", func(c *gin.Context) {
        c.JSON(200, gin.H{
            "message": "pong",
        })
    })
    r.GET("/sensors", func(c *gin.Context) {
        // calls 'sensors' and saves output
        cmd := exec.Command("sensors")
        outputBytes, err := cmd.Output()
        if err != nil {
            c.JSON(http.StatusInternalServerError, gin.H{
                "status": "internal server error",
            })
        }
        outputString := string(outputBytes)
        c.JSON(http.StatusOK, gin.H{
            "data": outputString,
        })
    })
    http.ListenAndServe(":1337", r)
}
