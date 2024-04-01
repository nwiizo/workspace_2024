//go:build js && wasm
// +build js,wasm

package main

import (
	"strings"
	"syscall/js"
)

func countChars(this js.Value, args []js.Value) interface{} {
	text := args[0].String()
	withSpaces := len(text)
	withoutSpaces := len(strings.ReplaceAll(text, " ", ""))
	lines := len(strings.Split(text, "\n"))
	paragraphs := len(strings.Split(text, "\n\n"))

	return map[string]interface{}{
		"withSpaces":    withSpaces,
		"withoutSpaces": withoutSpaces,
		"lines":         lines,
		"paragraphs":    paragraphs,
	}
}

func main() {
	js.Global().Set("countChars", js.FuncOf(countChars))
	<-make(chan bool)
}
