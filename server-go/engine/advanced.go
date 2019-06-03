package engine

import (
    "log"
    "sexy/fetcher"
)

type AdvancedEngine struct {
    WorkerNum int

    Scheduler Scheduler
}

func createWorker(fc *fetcher.Fetcher, in chan Task, out chan ParseResult) {
    go func() {
        var task Task
        for {
            task = <-in
            if parseResult, e := worker(fc, task); e == nil {
                out <- parseResult
            }
        }
    }()
}

func (engine *AdvancedEngine) Run(fc *fetcher.Fetcher, tasks []Task) {
    var (
        in  = make(chan Task)
        out = make(chan ParseResult)
    )

    engine.Scheduler.SetTaskChannel(in)

    for i := 0; i < engine.WorkerNum; i++ {
        createWorker(fc, in, out)
    }

    for _, task := range tasks {
        engine.Scheduler.Add(task)
    }

    var parseResult ParseResult
    for {
        parseResult = <-out
        if parseResult.Item != nil {
            log.Printf("got item, %+v\n", parseResult.Item)
        } else {
            for _, task := range parseResult.Tasks {
                log.Println("got task,", task.Request.URL)
                engine.Scheduler.Add(task)
            }
        }
    }
}
