package main

import (
	// "bufio"
	"fmt"
	// "io"
	"os"
	// "path/filepath"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type SKU struct {
	Symbol string
	Value int
}

func (Self SKU) Next() SKU {
	return SKU{
		Symbol: strconv.Itoa(Self.Value + 1),
		Value: Self.Value + 1,
	}
}

func (Self SKU) IsRepeatTwice() bool {
	length := len(Self.Symbol)
	if (length % 2 != 0) {
		return false
	}	

	half_length := length / 2
	
	first_half := Self.Symbol[0:half_length]
	second_half := Self.Symbol[half_length:]

	return first_half == second_half
}

type SKURange struct {
	From SKU
	To SKU
}

func (Self SKURange) Items() (func (yield func(SKU) bool)) {
	return func (yield func(SKU) bool) {
		current := Self.From
		for current.Value != Self.To.Value {
			if (!yield(current)) {
				return
			}
			current = current.Next()
		}
		yield(Self.To)
	}
}

func SKURangeFromString(str string) SKURange {
	parts := strings.Split(str, "-")

	fromVal, err := strconv.Atoi(strings.TrimSpace(parts[0]))
	check(err)
	toVal, err := strconv.Atoi(strings.TrimSpace(parts[1]))
	check(err)

	return SKURange{
		From: SKU{
			Symbol: parts[0],
			Value: fromVal,
		},
		To: SKU{
			Symbol: parts[1],
			Value: toVal,
		},
	}
}

func main() {
	input_file_path := os.Args[1]
	input, err := os.ReadFile(input_file_path)
	check(err)

	skuRangeStrings := strings.Split(string(input), ",")
	skuRanges := make([]SKURange, len(skuRangeStrings))
	for i, str := range skuRangeStrings {
		skuRanges[i] = SKURangeFromString(str)
	}

	total := 0
	for _, skuRange := range skuRanges {
		fmt.Println("Range from ", skuRange.From.Value, " to ", skuRange.To.Value)
		for sku := range skuRange.Items() {
			if (sku.IsRepeatTwice()) {
				total += sku.Value
			}
		}
	}

	fmt.Println("Total: ", total)
}