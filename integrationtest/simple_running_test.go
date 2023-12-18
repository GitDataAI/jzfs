package integrationtest

import (
	"context"
	"testing"
	"time"
)

func TestRunning(t *testing.T) {
	_, cancel := SetupDaemon(t, context.Background())
	time.Sleep(time.Second * 5)
	cancel()
}
