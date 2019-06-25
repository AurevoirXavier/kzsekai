package scheduler

import "sexy/engine"

type AdvancedScheduler struct {
    taskChannel   chan engine.Task
    workerChannel chan chan engine.Task
}

func (scheduler *AdvancedScheduler) AddTask(task engine.Task) {
    scheduler.taskChannel <- task
}

func (scheduler *AdvancedScheduler) AddIdleWorker(worker chan engine.Task) {
    scheduler.workerChannel <- worker
}

func (scheduler *AdvancedScheduler) WorkerChannel() chan engine.Task {
    return make(chan engine.Task)
}

func (scheduler *AdvancedScheduler) Run() {
    scheduler.taskChannel = make(chan engine.Task)
    scheduler.workerChannel = make(chan chan engine.Task)

    go func() {
        var (
            tasks   []engine.Task
            workers []chan engine.Task
        )

        for {
            var (
                task   engine.Task
                worker chan engine.Task
            )

            if len(tasks) > 0 && len(workers) > 0 {
                task = tasks[0]
                worker = workers[0]
            }

            select {
            case task := <-scheduler.taskChannel:
                tasks = append(tasks, task)
            case worker := <-scheduler.workerChannel:
                workers = append(workers, worker)
            // if nil skip this case
            case worker <- task:
                tasks = tasks[1:]
                workers = workers[1:]
            }
        }
    }()
}
