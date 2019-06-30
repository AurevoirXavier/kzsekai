package hpjav

import (
    "fmt"
    "log"
    "net/http"
    "sexy/engine"
    "sexy/fetcher"
    "sexy/hpjav/parser"
    "strconv"
)

type HpJav struct {
    LastPage uint16

    Fetcher *fetcher.Fetcher
}

const (
    Host      = "https://hpjav.tv"
    WorkerNum = 30
)

func NewHpJav() *HpJav {
    var fc = fetcher.Fetcher{
        Client:    http.DefaultClient,
        UserAgent: "Mozilla/5.0",
    }

    return &HpJav{
        LastPage: 0,
        Fetcher:  &fc,
    }
}

func (hpJav *HpJav) GetLastPage() {
    log.Println("getting last page from,", Host)

    var req, _ = http.NewRequest("GET", fmt.Sprintf("%s/tw/tag/cosplay", Host), nil)
    req.Header.Set("User-Agent", hpJav.Fetcher.UserAgent)

    var (
        doc, _       = hpJav.Fetcher.FetchDoc(req)
        lastPageATag = doc.Find(`.extend`)
        href, _      = lastPageATag.Attr("href")
        lastPage, _  = strconv.Atoi(href[37:])
    )
    hpJav.LastPage = uint16(lastPage)
}

func (hpJav *HpJav) Scrape() {
    hpJav.GetLastPage()

    var tasks []engine.Task

    //for pageNum := uint16(1); pageNum < hpJav.LastPage; pageNum += 1 {
    for pageNum := uint16(1); pageNum < 2; pageNum += 1 {
       var (
           pageUrl = fmt.Sprintf("%s/tw/tag/cosplay/page/%d", Host, pageNum)
           req, _  = http.NewRequest("GET", pageUrl, nil)
           task    = engine.Task{Request: req, ParserFunc: parser.ParsePage }
       )
       tasks = append(tasks, task)
    }

    var basicEngine = engine.BasicEngine{}
    basicEngine.Run(hpJav.Fetcher, tasks)
    //var advancedEngine = engine.AdvancedEngine{
    // WorkerNum: WorkerNum,
    // Scheduler: &scheduler.AdvancedScheduler{},
    //}
    //advancedEngine.Run(hpJav.Fetcher, tasks)
}
