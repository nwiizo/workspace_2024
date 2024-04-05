package main

import "github.com/go-rod/rod"

func main() {
	browser := rod.New().MustConnect().MustPage("https://3-shake.com/")
	browser.MustWaitStable().MustScreenshot("3-shake.png")
	defer browser.MustClose()
}
