package cosplayjav

import (
    "fmt"
    "github.com/PuerkitoBio/goquery"
    "log"
    "net/http"
    "sexy/cosplayjav/parser"
    "sexy/engine"
    "sexy/fetcher"
    "sexy/scheduler"
    "strconv"
)

type CosplayJav struct {
    LastPage uint16

    Fetcher *fetcher.Fetcher
}

const (
    Host      = "http://cosplayjav.pl"
    ProxyUrl  = "http://127.0.0.1:1087"
    WorkerNum = 30
)

func NewCosplayJav() *CosplayJav {
    var fc = fetcher.Fetcher{
        Client:    http.DefaultClient,
        UserAgent: "Mozilla/5.0",
    }

    fc.SetProxy(ProxyUrl)
    fc.Bypass(Host)

    return &CosplayJav{
        LastPage: 0,
        Fetcher:  &fc,
    }
}

func (cosplayJav *CosplayJav) GetLastPage() {
    log.Println("getting last page from,", Host)

    var req, _ = http.NewRequest("GET", Host, nil)
    req.Header.Set("User-Agent", cosplayJav.Fetcher.UserAgent)

    var (
        doc, _       = cosplayJav.Fetcher.FetchDoc(req)
        lastPageATag = doc.Find(`a.page-numbers:nth-child(10)`)
        lastPage, _  = strconv.Atoi(lastPageATag.Text())
    )
    cosplayJav.LastPage = uint16(lastPage)
}

func (cosplayJav *CosplayJav) Scrape() {
    cosplayJav.GetLastPage()

    var tasks []engine.Task

    for pageNum := uint16(1); pageNum < cosplayJav.LastPage; pageNum += 1 {
    //for pageNum := uint16(1); pageNum < 2; pageNum += 1 {
        var (
            pageUrl = fmt.Sprintf("%s/page/%d", Host, pageNum)
            req, _  = http.NewRequest("GET", pageUrl, nil)
            task    = engine.Task{Request: req, ParserFunc: func(doc *goquery.Document) engine.ParseResult { return parser.ParsePage(doc, cosplayJav.Fetcher) }}
        )
        tasks = append(tasks, task)
    }

    var advancedEngine = engine.AdvancedEngine{
        WorkerNum: WorkerNum,
        Scheduler: &scheduler.AdvancedScheduler{},
    }
    advancedEngine.Run(cosplayJav.Fetcher, tasks)
}
