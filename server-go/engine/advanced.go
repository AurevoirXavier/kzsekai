package engine

import (
    "log"
    "sexy/fetcher"
)

type AdvancedEngine struct {
    WorkerNum int

    Scheduler Scheduler
}

func createWorker(fc *fetcher.Fetcher, scheduler Scheduler, in chan Task, out chan ParseResult) {
    go func() {
        for {
            scheduler.AddIdleWorker(in)
            if parseResult, e := work(fc, <-in); e == nil {
                out <- parseResult
            }
        }
    }()
}

func (engine *AdvancedEngine) Run(fc *fetcher.Fetcher, tasks []Task) {
    var out = make(chan ParseResult)

    engine.Scheduler.Run()

    for i := 0; i < engine.WorkerNum; i++ {
        createWorker(fc, engine.Scheduler, engine.Scheduler.WorkerChannel(), out)
    }

    for _, task := range tasks {
        engine.Scheduler.AddTask(task)
    }

    var parseResult ParseResult
    for {
        parseResult = <-out
        if parseResult.Item != nil {
            log.Printf("got item, %+v\n", parseResult.Item)
        } else {
            for _, task := range parseResult.Tasks {
                log.Println("got task,", task.Request.URL)
                engine.Scheduler.AddTask(task)
            }
        }
    }
}
