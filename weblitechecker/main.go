package main

import (
	"crypto/tls"
	"fmt"
	"log"
	"net/http"
	"net/url"
	"os"
	"strings"
	"sync"
	"time"

	"github.com/spf13/cobra"
	"golang.org/x/net/html"
)

var rootCmd = &cobra.Command{
	Use:   "weblitechecker",
	Short: "WebLiteChecker: A lightweight tool to check website security",
	Long: `WebLiteChecker is a lightweight command-line tool for quick website security checks.
It analyzes security headers, HTTPS usage, cookie attributes, and HTML elements to provide a comprehensive security overview.`,
}

var checkCmd = &cobra.Command{
	Use:   "check [url]",
	Short: "Check a website's security",
	Long:  `Perform a comprehensive security check on the specified website URL and its subpages.`,
	Args:  cobra.ExactArgs(1),
	Run:   runCheck,
}

var (
	insecure   bool
	timeout    int
	maxDepth   int
	cookieName string
)

func init() {
	rootCmd.AddCommand(checkCmd)
	checkCmd.Flags().BoolVarP(&insecure, "insecure", "k", false, "Skip TLS certificate verification")
	checkCmd.Flags().IntVarP(&timeout, "timeout", "t", 10, "Timeout in seconds for HTTP requests")
	checkCmd.Flags().IntVarP(&maxDepth, "depth", "d", 1, "Maximum depth for crawling subpages")
	checkCmd.Flags().StringVarP(&cookieName, "cookie", "c", "", "Specific cookie name to check")
}

type checker struct {
	client  *http.Client
	results sync.Map
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

func (c *checker) checkWebsite(siteURL string, depth int) {
	if depth > maxDepth {
		return
	}

	resp, err := c.client.Get(siteURL)
	if err != nil {
		log.Printf("Error fetching URL: %v\n", err)
		return
	}
	defer resp.Body.Close()

	results := make(map[string]string)

	results["HTTPS Usage"] = c.checkStatus(strings.HasPrefix(siteURL, "https://"))
	results["Strict-Transport-Security"] = c.checkHeader(resp, "Strict-Transport-Security")
	results["X-Frame-Options"] = c.checkHeader(resp, "X-Frame-Options")
	results["X-Content-Type-Options"] = c.checkHeaderValue(resp, "X-Content-Type-Options", "nosniff")
	results["Content-Security-Policy"] = c.checkHeader(resp, "Content-Security-Policy")
	results["Referrer-Policy"] = c.checkHeader(resp, "Referrer-Policy")
	results["Minimum TLS Version"] = c.checkTLSVersion(resp)

	c.checkCookies(resp, results)

	doc, err := html.Parse(resp.Body)
	if err != nil {
		log.Printf("Error parsing HTML: %v\n", err)
		c.results.Store(siteURL, results)
		return
	}

	htmlChecks := c.checkHTML(doc)
	for k, v := range htmlChecks {
		results[k] = v
	}

	c.results.Store(siteURL, results)

	c.crawlSubpages(siteURL, depth+1, doc)
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
	if resp.TLS != nil {
		switch resp.TLS.Version {
		case tls.VersionTLS13:
			return "TLS 1.3"
		case tls.VersionTLS12:
			return "TLS 1.2"
		case tls.VersionTLS11:
			return "TLS 1.1"
		case tls.VersionTLS10:
			return "TLS 1.0"
		default:
			return "Unknown"
		}
	}
	return "N/A"
}

func (c *checker) checkCookies(resp *http.Response, results map[string]string) {
	for _, cookie := range resp.Cookies() {
		if cookieName != "" && cookie.Name != cookieName {
			continue
		}

		prefix := fmt.Sprintf("Cookie (%s)", cookie.Name)

		results[prefix+" - Secure"] = c.checkStatus(cookie.Secure)
		results[prefix+" - HttpOnly"] = c.checkStatus(cookie.HttpOnly)
		results[prefix+" - SameSite"] = c.checkSameSite(cookie)

		sameSiteNone := (cookie.SameSite == http.SameSiteNoneMode)
		if sameSiteNone && (!cookie.Secure || resp.TLS == nil) {
			results[prefix+" - SameSite=None requires Secure"] = "NG"
		}
	}
}

func (c *checker) checkSameSite(cookie *http.Cookie) string {
	switch cookie.SameSite {
	case http.SameSiteStrictMode:
		return "Strict"
	case http.SameSiteLaxMode:
		return "Lax"
	case http.SameSiteNoneMode:
		return "None"
	default:
		return "Default"
	}
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

func (c *checker) crawlSubpages(siteURL string, depth int, doc *html.Node) {
	var urls []string

	var findURLs func(*html.Node)
	findURLs = func(n *html.Node) {
		if n.Type == html.ElementNode && n.Data == "a" {
			for _, attr := range n.Attr {
				if attr.Key == "href" {
					url, err := url.Parse(attr.Val)
					if err == nil {
						if url.IsAbs() && url.Host == "" {
							url.Host = siteURL
						}
						if url.Host == siteURL {
							urls = append(urls, url.String())
						}
					}
				}
			}
		}
		for c := n.FirstChild; c != nil; c = c.NextSibling {
			findURLs(c)
		}
	}
	findURLs(doc)

	for _, url := range urls {
		c.checkWebsite(url, depth)
	}
}

func runCheck(cmd *cobra.Command, args []string) {
	siteURL := args[0]
	if !strings.HasPrefix(siteURL, "http://") && !strings.HasPrefix(siteURL, "https://") {
		siteURL = "https://" + siteURL
	}
	parsedURL, err := url.Parse(siteURL)
	if err != nil {
		log.Fatalf("Invalid URL: %v\n", err)
	}
	siteURL = parsedURL.Scheme + "://" + parsedURL.Host

	checker := newChecker(insecure, timeout)
	checker.checkWebsite(siteURL, 0)

	fmt.Printf("WebLiteChecker Security Check Results for %s:\n\n", siteURL)

	checker.results.Range(func(key, value interface{}) bool {
		url := key.(string)
		results := value.(map[string]string)

		if url != siteURL {
			fmt.Printf("\nSubpage: %s\n", url)
		}

		for check, result := range results {
			fmt.Printf("%-40s: %s\n", check, result)
		}
		return true
	})
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
