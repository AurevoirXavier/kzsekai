package parser

import (
    "github.com/PuerkitoBio/goquery"
    "sexy/engine"
)

type Post struct {
    Address string
    Title   string

    Category []string
    Tags     []string
}

func ParsePost(doc *goquery.Document) engine.ParseResult {
    var post = Post{}

    var postUrl, _ = doc.Find(`head > link:nth-child(9)`).Attr("href")
    post.Address = postUrl
    post.Title = doc.Find(`#video h1`).Text()

    doc.Find(`.cat:nth-child(3) .ent a`).Each(func(_ int, s *goquery.Selection) {
        post.Category = append(post.Category, s.Text())
    })
    doc.Find(`.cat:nth-child(4) .ent a`).Each(func(_ int, s *goquery.Selection) {
        post.Tags = append(post.Tags, s.Text())
    })

    return engine.ParseResult{Item: post}
}
