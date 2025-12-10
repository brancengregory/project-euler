package main

import(
    "fmt";
    "io/ioutil";
    "math/big";
    "strings"
)

func main() {
    
    byt, err := ioutil.ReadFile("input.txt")
    if err != nil {
        fmt.Print(err)
    }
    
    str := string(byt)
    strarr := strings.Split(str, "\n")
    
    sum := new(big.Int).SetInt64(0)
    for _, val := range strarr {
        if temp, ok := new(big.Int).SetString(val, 10); ok {
            sum.Add(sum, temp)
        }
    }
    
    fmt.Println(sum.String()[0:10])
}