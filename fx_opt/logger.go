package fx_opt //nolint

import (
	logging "github.com/ipfs/go-log/v2"
	"go.uber.org/fx"
)

var fxLog = logging.Logger("fx")
var _ fx.Printer = (*Logger)(nil)

// Logger log for debug fx message
type Logger struct{}

// Printf print fx log message to debug log
func (log Logger) Printf(msg string, arg ...interface{}) {
	fxLog.Debugf(msg, arg...)
}
