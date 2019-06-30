package parser

import (
    "github.com/PuerkitoBio/goquery"
    "sexy/engine"
    "strings"
    "sync"
)

func ParseLink(doc *goquery.Document, guard *sync.Mutex, post *Post) engine.ParseResult {
    var link, _ = doc.Find(`.btn.btn-primary.btn-download`).Attr("href")
    guard.Lock()
    if strings.Contains(link, "openload") {
        post.Parts["openload"] = append(post.Parts["openload"], link)
    } else if strings.Contains(link, "mega") {
        post.Parts["mega"] = append(post.Parts["mega"], link)
    } else {
        post.Parts["unknown"] = append(post.Parts["unknown"], link)
    }
    guard.Unlock()

    return engine.ParseResult{}
}
