package parser

import (
    "github.com/PuerkitoBio/goquery"
    "log"
    "net/http"
    "sexy/engine"
    "sexy/fetcher"
    "strings"
    "sync"
)

type Post struct {
    Actress            string
    AlternativeTitle   string
    AnimeGameSeries    string
    CharacterCosplay   string
    Company            string
    Id                 string
    Info               string
    InPremiumSectionTo string
    Title              string

    Parts map[string][]string
}

func ParsePost(doc *goquery.Document, fc *fetcher.Fetcher) engine.ParseResult {
    var (
        guard sync.Mutex

        tasks []engine.Task
        post  = Post{}
    )

    doc.Find(`.post-description tr`).Each(func(_ int, s *goquery.Selection) {
        switch k, v := strings.ToUpper(s.Find(`td:nth-child(1)`).Text()), strings.TrimSpace(s.Find(`td:nth-child(2)`).Text()); {
        case k == "" || v == "":
            return
        case strings.Contains(k, "ID"):
            post.Id = v
        case strings.Contains(k, "TITLE"):
            post.Title = v
        case strings.Contains(k, "ALTERNATIVE"):
            post.AlternativeTitle = v
        case strings.Contains(k, "COMPANY"):
            post.Company = v
        case strings.Contains(k, "RESS"):
            post.Actress = v
        case strings.Contains(k, "ANIME"):
            post.AnimeGameSeries = v
        case strings.Contains(k, "CHARACTER"):
            post.CharacterCosplay = v
        case strings.Contains(k, "IN"):
            post.Info = v
        case strings.Contains(k, "PREMIUM"):
            post.InPremiumSectionTo = v
        default:
            log.Fatalf("unhandled property, k, `%s, v, %s`", k, v)
        }
    })

    post.Parts = make(map[string][]string)
    doc.Find(`.post-description .btn.btn-primary`).Each(func(_ int, s *goquery.Selection) {
        var link, _ = s.Attr("href")
        if strings.HasPrefix(link, "//") {
            post.Parts["cosplayjav"] = append(post.Parts["cosplayjav"], "http:"+link)
        } else {
            var req, _ = http.NewRequest("GET", link, nil)
            tasks = append(tasks, engine.Task{Request: req, ParserFunc: func(doc *goquery.Document) engine.ParseResult { return ParseLink(doc, &guard, &post) }})
        }
    })

    var basicEngine = engine.BasicEngine{}
    basicEngine.Run(fc, tasks)

    return engine.ParseResult{Item: post}
}
