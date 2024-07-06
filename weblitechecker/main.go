package main

import (
	"crypto/tls"
	"fmt"
	"log"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/spf13/cobra"
	"golang.org/x/net/html"
)

var rootCmd = &cobra.Command{
	Use:   "weblitechecker",
	Short: "WebLiteChecker: A lightweight tool to check website security",
	Long: `WebLiteChecker is a lightweight command-line tool for quick website security checks.
It analyzes security headers, HTTPS usage, and HTML elements to provide a basic security overview.`,
}

var checkCmd = &cobra.Command{
	Use:   "check [url]",
	Short: "Check a website's security",
	Long:  `Perform a quick security check on the specified website URL.`,
	Args:  cobra.ExactArgs(1),
	Run:   runCheck,
}

var (
	insecure bool
	timeout  int
)

func init() {
	rootCmd.AddCommand(checkCmd)
	checkCmd.Flags().BoolVarP(&insecure, "insecure", "k", false, "Skip TLS certificate verification")
	checkCmd.Flags().IntVarP(&timeout, "timeout", "t", 10, "Timeout in seconds for HTTP requests")
}

type checker struct {
	client *http.Client
}

func newChecker(insecure bool, timeout int) *checker {
	return &checker{
		client: &http.Client{
			Timeout: time.Duration(timeout) * time.Second,
			Transport: &http.Transport{
				TLSClientConfig: &tls.Config{InsecureSkipVerify: insecure},
			},
		},
	}
}

func (c *checker) checkWebsite(url string) map[string]string {
	results := make(map[string]string)

	resp, err := c.client.Get(url)
	if err != nil {
		log.Printf("Error fetching URL: %v\n", err)
		return results
	}
	defer resp.Body.Close()

	results["HTTPS Usage"] = c.checkStatus(strings.HasPrefix(url, "https://"))
	results["Strict-Transport-Security"] = c.checkHeader(resp, "Strict-Transport-Security")
	results["X-Frame-Options"] = c.checkHeader(resp, "X-Frame-Options")
	results["X-Content-Type-Options"] = c.checkHeaderValue(resp, "X-Content-Type-Options", "nosniff")
	results["Content-Security-Policy"] = c.checkHeader(resp, "Content-Security-Policy")
	results["Referrer-Policy"] = c.checkHeader(resp, "Referrer-Policy")
	results["Minimum TLS Version"] = c.checkTLSVersion(resp)

	doc, err := html.Parse(resp.Body)
	if err != nil {
		log.Printf("Error parsing HTML: %v\n", err)
		return results
	}

	htmlChecks := c.checkHTML(doc)
	for k, v := range htmlChecks {
		results[k] = v
	}

	return results
}

func (c *checker) checkStatus(condition bool) string {
	if condition {
		return "OK"
	}
	return "NG"
}

func (c *checker) checkHeader(resp *http.Response, header string) string {
	if resp.Header.Get(header) != "" {
		return "OK"
	}
	return "NG"
}

func (c *checker) checkHeaderValue(resp *http.Response, header, expectedValue string) string {
	if resp.Header.Get(header) == expectedValue {
		return "OK"
	}
	return "NG"
}

func (c *checker) checkTLSVersion(resp *http.Response) string {
	if resp.TLS != nil && resp.TLS.Version >= tls.VersionTLS12 {
		return "OK"
	}
	return "NG"
}

func (c *checker) checkHTML(n *html.Node) map[string]string {
	results := make(map[string]string)
	results["Title Tag"] = "NG"
	results["Meta Description"] = "NG"
	results["Lang Attribute"] = "NG"
	results["Favicon"] = "NG"
	results["Apple Touch Icon"] = "NG"

	var traverse func(*html.Node)
	traverse = func(n *html.Node) {
		if n.Type == html.ElementNode {
			switch n.Data {
			case "html":
				for _, attr := range n.Attr {
					if attr.Key == "lang" && attr.Val != "" {
						results["Lang Attribute"] = "OK"
					}
				}
			case "head":
				for c := n.FirstChild; c != nil; c = c.NextSibling {
					if c.Type == html.ElementNode {
						switch c.Data {
						case "title":
							results["Title Tag"] = "OK"
						case "meta":
							for _, attr := range c.Attr {
								if attr.Key == "name" && attr.Val == "description" {
									results["Meta Description"] = "OK"
								}
							}
						case "link":
							for _, attr := range c.Attr {
								if attr.Key == "rel" && attr.Val == "icon" {
									results["Favicon"] = "OK"
								}
								if attr.Key == "rel" && attr.Val == "apple-touch-icon" {
									results["Apple Touch Icon"] = "OK"
								}
							}
						}
					}
				}
			}
		}
		for c := n.FirstChild; c != nil; c = c.NextSibling {
			traverse(c)
		}
	}
	traverse(n)

	return results
}

func runCheck(cmd *cobra.Command, args []string) {
	url := args[0]
	checker := newChecker(insecure, timeout)
	results := checker.checkWebsite(url)

	fmt.Printf("WebLiteChecker Security Check Results for %s:\n\n", url)
	for check, result := range results {
		fmt.Printf("%-30s: %s\n", check, result)
	}
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
