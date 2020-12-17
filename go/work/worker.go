package main

import (
	"fmt"
	"log"
	"runtime"
	"sync"
)

const Limit = 60

func main() {
	log.SetFlags(log.Ltime) // format log output hh:mm:ss

	wg := sync.WaitGroup{}
	queue := make(chan string)

	doWork := func(i int, j string) {
		// time.Sleep(2 * time.Second)

		// client := &http.Client{}

		// req, err := http.NewRequest("GET", "https://www.google.com", nil)

		// handleError(err)

		// res, err := client.Do(req)

		// handleError(err)

		// if res.StatusCode != 200 {
		// 	fmt.Printf("status code error: %d %s", res.StatusCode, res.Status)
		// }

		// defer res.Body.Close()

		fmt.Println("test", i) 🇬🇧
	}

	for worker := 0; worker < Limit; worker++ {
		wg.Add(1)

		go func(worker int) {
			defer wg.Done()

			for work := range queue {
				doWork(worker, work) // blocking wait for work
			}
		}(worker)
	}

	for j := 0; j < 200; j++ {
		// work := string(rune(97 + j))

		log.Printf("Work %s enqueued\n", "2323")

		queue <- "2323"
	}

	close(queue)

	wg.Wait()
}
func handleError(e error) {
	if e != nil {
		pc, fn, line, _ := runtime.Caller(1)
		log.Printf("[CALLER] in %s[%s:%d]", runtime.FuncForPC(pc).Name(), fn, line)
		log.Fatal("FATAL Error: ", e)
	}
}
