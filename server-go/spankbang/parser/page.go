package parser

import (
    "github.com/PuerkitoBio/goquery"
    "net/http"
    "sexy/engine"
    "sexy/spankbang/config"
)

func ParsePage(doc *goquery.Document) engine.ParseResult {
    var parseResult = engine.ParseResult{}
    doc.Find(`.video-list-with-ads .video-item > a`).Each(func(_ int, s *goquery.Selection) {
        var (
            href, _ = s.Attr("href")
            postUrl = config.Host + href
            req, _  = http.NewRequest("GET", postUrl, nil)
        )
        parseResult.Tasks = append(parseResult.Tasks, engine.Task{Request: req, ParserFunc: ParsePost})
    })

    return parseResult
}
