package main

import (
	"fmt"
	"log"
	"os"
	"strings"
    "unicode"
)


func main() {
    log.SetPrefix("greetings: ")
    log.SetFlags(0)
    data, err := os.ReadFile("input.txt") 
    if err != nil {
        log.Fatal(err)
    }
    sum := extractLines(string(data))
    fmt.Println(sum)
}

func extractLines(data string) int {
    data = strings.TrimSpace(data)
    lines := strings.Split(data, "\n")
    var first_digit = 0
    var second_digit = 0
    var sum = 0

    for _, line := range lines {
        for _, char := range line {
           if unicode.IsDigit(char) {
               first_digit = int( char - '0')
               break
           }
        }
        
        runes := []rune(line)
        size := len(runes)
        
        for i := size - 1; i >=0 ; i-- {
            char := runes[i] 
           if unicode.IsDigit(char) {
               second_digit = int( char - '0')
               break
           }
        }
        
        sum += (first_digit * 10) + second_digit
    }
    
    return sum
}


