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
            return
        }
        outputString := string(outputBytes)
        c.JSON(http.StatusOK, gin.H{
            "data": outputString,
        })
    })

    r.GET("/sensors/cpu", func(c *gin.Context) {
        cmd := exec.Command("sensors", "coretemp-isa-0000")
        outputBytes, err := cmd.Output()
        if err != nil {
            c.JSON(http.StatusInternalServerError, gin.H{
                "status": "internal server error",
            })
            return
        }
        outputString := string(outputBytes)
        c.JSON(http.StatusOK, gin.H{
            "data": outputString,
        })
    })

    r.GET("/sensors/gpu", func(c *gin.Context) {
        cmd := exec.Command("sensors", "nouveau-pci-0100")
        outputBytes, err := cmd.Output()
        if err != nil {
            c.JSON(http.StatusInternalServerError, gin.H{
                "status": "internal server error",
            })
            return
        }
        outputString := string(outputBytes)
        c.JSON(http.StatusOK, gin.H{
            "data": outputString,
        })
    })

    http.ListenAndServe(":1337", r)
}
