package integrationtest

import (
	"context"
	"testing"

	"github.com/smartystreets/goconvey/convey"
)

func TestSpec(t *testing.T) {
	ctx := context.Background()
	urlStr, cancel := SetupDaemon(t, ctx)
	defer cancel()

	convey.Convey("user test", t, UserSpec(ctx, urlStr))
	convey.Convey("repo test", t, RepoSpec(ctx, urlStr))
	convey.Convey("branch test", t, BranchSpec(ctx, urlStr))
	convey.Convey("branch test", t, WipSpec(ctx, urlStr))
	convey.Convey("branch test", t, ObjectSpec(ctx, urlStr))
	convey.Convey("branch test", t, WipObjectSpec(ctx, urlStr))
}
