package engine

import (
    "github.com/PuerkitoBio/goquery"
    "net/http"
    "sexy/fetcher"
)

type Engine interface {
    Run(*fetcher.Fetcher, []Task)
}

type ParseResult struct {
    Item  interface{}
    Tasks []Task
}

type Task struct {
    Request    *http.Request
    ParserFunc func(document *goquery.Document) ParseResult
}

type Scheduler interface {
    AddTask(Task)
    AddIdleWorker(chan Task)
    WorkerChannel() chan Task
    Run()
}
