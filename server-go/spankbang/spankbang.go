package spankbang

import (
    "fmt"
    "log"
    "net/http"
    "sexy/engine"
    "sexy/fetcher"
    "sexy/scheduler"
    "sexy/spankbang/config"
    "sexy/spankbang/parser"
    "strconv"
)

type SpankBang struct {
    LastPage uint16

    Fetcher *fetcher.Fetcher
}

func NewSpankBang() *SpankBang {
    var fc = fetcher.Fetcher{
        Client:    http.DefaultClient,
        UserAgent: "Mozilla/5.0",
    }

    return &SpankBang{
        LastPage: 0,
        Fetcher:  &fc,
    }
}

func (spankBang *SpankBang) GetLastPage() {
    var firstPageUrl = fmt.Sprintf("%s/tag/cosplay", config.Host)

    log.Println("getting last page from,", firstPageUrl)

    var req, _ = http.NewRequest("GET", firstPageUrl, nil)
    req.Header.Set("User-Agent", spankBang.Fetcher.UserAgent)

    var (
        doc, _       = spankBang.Fetcher.FetchDoc(req)
        lastPageATag = doc.Find(`.pagination ul > li:nth-child(9) > a`)
        lastPage, _  = strconv.Atoi(lastPageATag.Text())
    )
    spankBang.LastPage = uint16(lastPage)
}

func (spankBang *SpankBang) Scrape() {
    spankBang.GetLastPage()

    var tasks []engine.Task

    for pageNum := uint16(1); pageNum < spankBang.LastPage; pageNum += 1 {
        //for pageNum := uint16(1); pageNum < 2; pageNum += 1 {
        var (
            pageUrl = fmt.Sprintf("%s/tag/cosplay/%d", config.Host, pageNum)
            req, _  = http.NewRequest("GET", pageUrl, nil)
            task    = engine.Task{Request: req, ParserFunc: parser.ParsePage}
        )
        tasks = append(tasks, task)
    }

    //var basicEngine = engine.BasicEngine{}
    //basicEngine.Run(spankBang.Fetcher, tasks)
    var advancedEngine = engine.AdvancedEngine{
        WorkerNum: config.WorkerNum,
        Scheduler: &scheduler.AdvancedScheduler{},
    }
    advancedEngine.Run(spankBang.Fetcher, tasks)
}
