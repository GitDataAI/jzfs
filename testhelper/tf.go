package testhelper

import (
	"time"

	"github.com/google/go-cmp/cmp"
)

var DBTimeCmpOpt = cmp.Comparer(func(x, y time.Time) bool {
	return x.Unix() == y.Unix()
})
