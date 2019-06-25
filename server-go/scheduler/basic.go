package scheduler

import "sexy/engine"

type BasicScheduler struct {
    taskChannel chan engine.Task
}

func (scheduler *BasicScheduler) AddTask(task engine.Task) {
    go func() { scheduler.taskChannel <- task }()
}

func (scheduler *BasicScheduler) AddIdleWorker(chan engine.Task) {
}

func (scheduler *BasicScheduler) WorkerChannel() chan engine.Task {
    return scheduler.taskChannel
}

func (scheduler *BasicScheduler) Run() {
    scheduler.taskChannel = make(chan engine.Task)
}
