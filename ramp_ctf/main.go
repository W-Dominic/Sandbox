package main

import (
	"bufio"
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
)

// flag: citadel
func decodehash() {
    file, _ := os.Open("./dict.txt")

    defer file.Close()

    var lines []string
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }   

    salt := "5a961fe77203"
    md5hash := "53d9ab1fa0d3ff9557b162ddd1b3d0d5"
    for _, line := range lines {
        hashed := md5.Sum([]byte(line+salt))
        hashedstr := hex.EncodeToString(hashed[:])
        if hashedstr == md5hash {
            fmt.Println(line)
            fmt.Print(hashedstr)
        }
    }
}

// flag: audibly
func getstream() {
    letters := []string{}
    for i := 0; i<100; i++ {
        apiurl := "https://0ijq1i6sp1.execute-api.us-east-1.amazonaws.com/dev/stream"
        req, _ := http.NewRequest("GET", apiurl, nil)
        req.Header.Add("Content-type", "text/html")

        resp, _ := http.DefaultClient.Do(req)
        respBody, _ := ioutil.ReadAll(resp.Body)
        // fmt.Printf("%s\n", respBody)

        exists := false
        for _, s := range letters{
            if s == string(respBody) {
                exists = true 
            } 
        }
        if !exists {
            letters = append(letters, string(respBody))
        }
    }
    
    for _, s := range letters {
        fmt.Printf("%s", s)
    }

}

// flag: amusing
func addstofortytwo() {
    file, _ := os.Open("./dict.txt")

    defer file.Close()

    var lines []string
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }   
    
    for _, s := range lines {
        val := 0
        for _, c := range s {
            val += int(c) - 96 
        }
        if val == 42 {
            fmt.Printf("%s\n", s)
        }
    }
}

// flag: caution
func useragentrequest() {
    apiurl := "https://0ijq1i6sp1.execute-api.us-east-1.amazonaws.com/dev/browser"
    req, _ := http.NewRequest("GET", apiurl, nil)
    req.Header.Add("User-agent", "Mozilla/8.8 (Macintosh; Intel Mac OS X 8888_8888) AppleWebKit/888.8.88 (KHTML, like Gecko) Version/88.8.8 Safari/888.8.88")

    resp, _ := http.DefaultClient.Do(req)
    respBody, _ := ioutil.ReadAll(resp.Body)
    fmt.Printf("%s\n", respBody)
}


func main() {
    // decodehash()
    // getstream()
    // addstofortytwo()
    useragentrequest()
}
