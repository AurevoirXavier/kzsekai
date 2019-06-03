package scheduler

import "sexy/engine"

type BasicScheduler struct {
    taskChannel chan engine.Task
}

func (scheduler *BasicScheduler) SetTaskChannel(taskChannel chan engine.Task) {
    scheduler.taskChannel = taskChannel
}

func (scheduler *BasicScheduler) Add(task engine.Task) {
    go func() { scheduler.taskChannel <- task }()
}
