package testhelper

import (
	"time"

	"github.com/google/go-cmp/cmp"
)

var DbTimeCmpOpt = cmp.Comparer(func(x, y time.Time) bool {
	return x.Unix() == y.Unix()
})
