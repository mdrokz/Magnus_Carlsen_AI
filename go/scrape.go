package main

import (
	"bufio"
	"fmt"
	"log"
	"net/http"
	"os"
	"runtime"
	"sync"

	"github.com/PuerkitoBio/goquery"
)

func main() {

	log.SetFlags(log.LstdFlags | log.Lshortfile)

	var wg sync.WaitGroup

	f, err := os.Open("../html/t.txt")

	handleError(err)

	defer func() {
		handleError(f.Close())
	}()

	scanner := bufio.NewScanner(f)

	// for scanner.Scan() {
	// 	wg.Add(1)
	// 	go writeTitle(&wg, scanner.Text())
	// }

	for i := 0; i < 200; i++ {
		wg.Add(1)
		go writeTitle(&wg, scanner.Text())
	}

	wg.Wait()

	fmt.Println("hello world")
}

func writeTitle(wg *sync.WaitGroup, url string) {
	client := &http.Client{}

	req, err := http.NewRequest("GET", "https://www.google.com", nil)

	req.Header.Set("Connection", "close")

	handleError(err)

	res, err := client.Do(req)

	handleError(err)

	defer wg.Done()

	if res.StatusCode != 200 {
		fmt.Printf("status code error: %d %s", res.StatusCode, res.Status)
	}

	doc, err := goquery.NewDocumentFromReader(res.Body)

	handleError(err)

	doc.Find("a").Each(func(i int, s *goquery.Selection) {
		title, _ := s.Attr("href")

		fmt.Println(i, title)
	})
	handleError(res.Body.Close())
}

func handleError(e error) {
	if e != nil {
		pc, fn, line, _ := runtime.Caller(1)
		log.Printf("[CALLER] in %s[%s:%d]", runtime.FuncForPC(pc).Name(), fn, line)
		log.Fatal("FATAL Error: ", e)
	}
}
