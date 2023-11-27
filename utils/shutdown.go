package utils

import (
	"context"
	"os"
	"os/signal"
	"syscall"
)

// Shutdown  chan for catch signal to shutdown program
type Shutdown chan struct{}

// CatchSig wait for sigquit sigterm sigint sighup sigsegv to stop program
func CatchSig(ctx context.Context, done Shutdown) {
	c := make(chan os.Signal, 1)
	signal.Notify(c, syscall.SIGHUP, syscall.SIGQUIT, syscall.SIGTERM, syscall.SIGINT, syscall.SIGSEGV)
LOOP:
	for {
		select {
		case <-ctx.Done():
			break LOOP
		case s := <-c:
			switch s {
			case syscall.SIGQUIT, syscall.SIGTERM, syscall.SIGINT:
				break LOOP
			case syscall.SIGHUP:
			case syscall.SIGSEGV:
			default:
				break LOOP
			}
		}
	}
	done <- struct{}{}
}
