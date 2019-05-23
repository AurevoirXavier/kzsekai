package engine

import (
    "github.com/PuerkitoBio/goquery"
    "net/http"
)

type ParseResult struct {
    Item interface{}
    Tasks []Task
}

type Task struct {
    Request    *http.Request
    ParserFunc func(document *goquery.Document) ParseResult
}
