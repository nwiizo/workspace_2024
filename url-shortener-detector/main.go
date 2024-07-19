package main

import (
	"context"
	"encoding/csv"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"os/signal"
	"regexp"
	"strings"
	"sync"
	"sync/atomic"
	"syscall"
	"time"

	"github.com/spf13/cobra"
	"golang.org/x/net/html"
	"golang.org/x/net/publicsuffix"
)

type Result struct {
	URL            string   `json:"url"`
	ShortURLs      []string `json:"short_urls,omitempty"`
	ShortenerTypes []string `json:"shortener_types,omitempty"`
}

var (
	baseURL        string
	outputFile     string
	outputFormat   string
	maxDepth       int
	maxConcurrency int
	userAgent      string
	visitedURLs    = make(map[string]bool)
	results        = make([]Result, 0)
	mutex          = &sync.Mutex{}
	wg             sync.WaitGroup
	urlCount       int64
	totalURLs      int64
	ctx            context.Context
	cancel         context.CancelFunc
	baseURLParsed  *url.URL
	baseDomain     string
)

var rootCmd = &cobra.Command{
	Use:   "url-shortener-detector",
	Short: "Detect URL shorteners in a website",
	Long:  `A tool to crawl a website and detect various URL shorteners, including goo.gl`,
	Run:   runDetector,
}

func init() {
	rootCmd.Flags().StringVarP(&baseURL, "url", "u", "", "Base URL to start crawling (required)")
	rootCmd.MarkFlagRequired("url")
	rootCmd.Flags().StringVarP(&outputFile, "output", "o", "results", "Output file name (without extension)")
	rootCmd.Flags().StringVarP(&outputFormat, "format", "f", "json", "Output format (json or csv)")
	rootCmd.Flags().IntVarP(&maxDepth, "depth", "d", 3, "Maximum crawl depth")
	rootCmd.Flags().IntVarP(&maxConcurrency, "concurrency", "c", 10, "Maximum number of concurrent requests")
	rootCmd.Flags().StringVarP(&userAgent, "user-agent", "a", "URLShortenerDetector/1.0", "User-Agent string")
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}

func runDetector(cmd *cobra.Command, args []string) {
	var err error
	if !strings.HasPrefix(baseURL, "http://") && !strings.HasPrefix(baseURL, "https://") {
		baseURL = "http://" + baseURL
	}

	baseURLParsed, err = url.Parse(baseURL)
	if err != nil {
		fmt.Printf("Error parsing base URL: %v\n", err)
		return
	}

	baseDomain, err = publicsuffix.EffectiveTLDPlusOne(baseURLParsed.Hostname())
	if err != nil {
		fmt.Printf("Error determining base domain: %v\n", err)
		return
	}

	ctx, cancel = context.WithCancel(context.Background())
	defer cancel()

	fmt.Println("Crawling started. Enjoy your onigiri while waiting!")

	fmt.Print("\n\033[s")

	urlCount = 0
	totalURLs = 1
	wg.Add(1)
	go crawl(ctx, baseURL, 0)

	done := make(chan bool)
	go showProgress(done)

	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, os.Interrupt, syscall.SIGTERM)
	go func() {
		<-sigChan
		fmt.Println("\nReceived interrupt signal. Gracefully shutting down...")
		cancel()
	}()

	wg.Wait()
	done <- true
	<-done

	fmt.Println("\nCrawling completed. Enjoy your onigiri!")

	saveResults()
}

func showProgress(done chan bool) {
	ticker := time.NewTicker(100 * time.Millisecond)
	defer ticker.Stop()

	for {
		select {
		case <-done:
			fmt.Print("\r\033[K")
			done <- true
			return
		case <-ticker.C:
			drawOnigiriProgress()
		}
	}
}

func drawOnigiriProgress() {
	width := 50
	count := atomic.LoadInt64(&urlCount)
	total := atomic.LoadInt64(&totalURLs)
	var progress float64
	if total > 0 {
		progress = float64(count) / float64(total)
	}
	completed := int(progress * float64(width))
	remaining := width - completed

	fmt.Print("\r\033[K")

	fmt.Print("[")
	for i := 0; i < completed; i++ {
		fmt.Print("🍙")
	}
	for i := 0; i < remaining; i++ {
		fmt.Print(" ")
	}
	fmt.Printf("] %.2f%% (%d/%d)", progress*100, count, total)
}

func crawl(ctx context.Context, urlStr string, depth int) {
	defer wg.Done()

	select {
	case <-ctx.Done():
		return
	default:
	}

	if depth > maxDepth {
		return
	}

	mutex.Lock()
	if visitedURLs[urlStr] {
		mutex.Unlock()
		return
	}
	visitedURLs[urlStr] = true
	atomic.AddInt64(&urlCount, 1)
	mutex.Unlock()

	client := &http.Client{
		Timeout: 30 * time.Second,
		Transport: &http.Transport{
			DisableKeepAlives: true,
		},
	}

	req, err := http.NewRequestWithContext(ctx, "GET", urlStr, nil)
	if err != nil {
		return
	}
	req.Header.Set("User-Agent", userAgent)

	resp, err := client.Do(req)
	if err != nil {
		return
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return
	}

	shortURLs, shortenerTypes := detectShortURLs(string(body))
	if len(shortURLs) > 0 {
		mutex.Lock()
		results = append(results, Result{
			URL:            urlStr,
			ShortURLs:      shortURLs,
			ShortenerTypes: shortenerTypes,
		})
		mutex.Unlock()
	}

	links := extractLinks(body)
	for _, link := range links {
		select {
		case <-ctx.Done():
			return
		default:
		}

		absoluteURL := toAbsoluteURL(urlStr, link)
		if absoluteURL != "" && isSameDomain(absoluteURL) {
			mutex.Lock()
			if !visitedURLs[absoluteURL] {
				atomic.AddInt64(&totalURLs, 1)
				wg.Add(1)
				go crawl(ctx, absoluteURL, depth+1)
			}
			mutex.Unlock()
		}
	}
}

func isSameDomain(urlStr string) bool {
	parsedURL, err := url.Parse(urlStr)
	if err != nil {
		return false
	}

	domain, err := publicsuffix.EffectiveTLDPlusOne(parsedURL.Hostname())
	if err != nil {
		return false
	}

	return domain == baseDomain
}

func detectShortURLs(content string) ([]string, []string) {
	patterns := map[string]*regexp.Regexp{
		"goo.gl":  regexp.MustCompile(`https?://goo\.gl/\w+`),
		"bit.ly":  regexp.MustCompile(`https?://bit\.ly/\w+`),
		"t.co":    regexp.MustCompile(`https?://t\.co/\w+`),
		"tinyurl": regexp.MustCompile(`https?://tinyurl\.com/\w+`),
		"ow.ly":   regexp.MustCompile(`https?://ow\.ly/\w+`),
	}

	var shortURLs []string
	var shortenerTypes []string

	for shortener, pattern := range patterns {
		matches := pattern.FindAllString(content, -1)
		shortURLs = append(shortURLs, matches...)
		for range matches {
			shortenerTypes = append(shortenerTypes, shortener)
		}
	}

	return shortURLs, shortenerTypes
}

func extractLinks(body []byte) []string {
	var links []string
	doc, err := html.Parse(strings.NewReader(string(body)))
	if err != nil {
		return links
	}

	var f func(*html.Node)
	f = func(n *html.Node) {
		if n.Type == html.ElementNode && n.Data == "a" {
			for _, a := range n.Attr {
				if a.Key == "href" {
					links = append(links, a.Val)
					break
				}
			}
		}
		for c := n.FirstChild; c != nil; c = c.NextSibling {
			f(c)
		}
	}
	f(doc)
	return links
}

func toAbsoluteURL(base, href string) string {
	uri, err := url.Parse(href)
	if err != nil {
		return ""
	}
	baseURL, err := url.Parse(base)
	if err != nil {
		return ""
	}
	uri = baseURL.ResolveReference(uri)
	return uri.String()
}

func saveResults() {
	if outputFormat == "json" {
		saveJSON()
	} else if outputFormat == "csv" {
		saveCSV()
	} else {
		saveJSON()
	}
}

func saveJSON() {
	file, err := os.Create(outputFile + ".json")
	if err != nil {
		fmt.Printf("Error creating output file: %v\n", err)
		return
	}
	defer file.Close()

	encoder := json.NewEncoder(file)
	encoder.SetIndent("", "  ")
	encoder.SetEscapeHTML(false)

	if err := encoder.Encode(results); err != nil {
		fmt.Printf("Error encoding results to JSON: %v\n", err)
	}
}

func saveCSV() {
	file, err := os.Create(outputFile + ".csv")
	if err != nil {
		fmt.Printf("Error creating output file: %v\n", err)
		return
	}
	defer file.Close()

	writer := csv.NewWriter(file)
	defer writer.Flush()

	headers := []string{"URL", "Short URLs", "Shortener Types"}
	writer.Write(headers)

	for _, result := range results {
		row := []string{
			result.URL,
			strings.Join(result.ShortURLs, "|"),
			strings.Join(result.ShortenerTypes, "|"),
		}
		writer.Write(row)
	}
}
