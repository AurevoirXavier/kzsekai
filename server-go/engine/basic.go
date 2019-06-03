package engine

import (
    "log"
    "sexy/fetcher"
)

type BasicEngine struct{}

func worker(fc *fetcher.Fetcher, task Task) (ParseResult, error) {
    log.Println("fetching, ", task.Request.URL)
    task.Request.Header.Set("User-Agent", fc.UserAgent)
    var doc, e = fc.FetchDoc(task.Request)
    if e != nil {
        log.Printf("error, fetching %s, %v\n", task.Request.URL, e)
        return ParseResult{}, e
    } else {
        return task.ParserFunc(doc), nil
    }
}

func (engine *BasicEngine) Run(fc *fetcher.Fetcher, tasks []Task) {
    for _, task := range tasks {
        log.Println("got task,", task.Request.URL)
    }

    for len(tasks) > 0 {
        var task = tasks[0]

        var parseResult, e = worker(fc, task)
        if e != nil {
            continue
        }

        tasks = tasks[1:]

        if parseResult.Item != nil {
            log.Printf("got item, %+v\n", parseResult.Item)
        } else {
            for _, task := range parseResult.Tasks {
                log.Println("got task,", task.Request.URL)
                tasks = append(tasks, task)
            }
        }
    }
}
