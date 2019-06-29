package parser

import (
    "github.com/PuerkitoBio/goquery"
    "net/http"
    "sexy/engine"
    "sexy/fetcher"
)

func ParsePage(doc *goquery.Document, fc *fetcher.Fetcher) engine.ParseResult {
    var parseResult = engine.ParseResult{}
    doc.Find(`.entry-title a`).Each(func(_ int, s *goquery.Selection) {
        var (
           postUrl, _ = s.Attr("href")
           req, _     = http.NewRequest("GET", postUrl, nil)
        )
        parseResult.Tasks = append(parseResult.Tasks, engine.Task{Request: req, ParserFunc: func(doc *goquery.Document) engine.ParseResult { return ParsePost(doc, fc) }})
    })

    return parseResult
}
