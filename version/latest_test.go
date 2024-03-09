package version_test

import (
	"fmt"
	"testing"

	"github.com/GitDataAI/jiaozifs/version"
	"github.com/stretchr/testify/require"
)

type checkLatestVersionTestCase struct {
	CurrentVersion   string
	LatestVersion    string
	ShouldError      bool
	ExpectedOutdated bool
}

func TestCheckLatestVersion(t *testing.T) {
	cases := []checkLatestVersionTestCase{
		{
			CurrentVersion:   version.UserVersion(),
			LatestVersion:    "1.0.0",
			ExpectedOutdated: false,
		},
		{
			CurrentVersion:   "v0.0.1",
			LatestVersion:    "1.2.3",
			ExpectedOutdated: true,
		},
		{
			CurrentVersion: "v1.2.3",
			LatestVersion:  "1.2.3",
		},
		{
			CurrentVersion: "v1.2.3",
			LatestVersion:  "1.0.0",
		},
		{
			LatestVersion: "abc",
			ShouldError:   true,
		},
	}
	for idx, tc := range cases {
		t.Run(fmt.Sprintf("check_latest_version_%d", idx), func(t *testing.T) {
			version.BuildVersion = tc.CurrentVersion
			t.Logf("check_latest_version test case input %+v", tc)
			latest, err := version.CheckLatestVersion(tc.LatestVersion)

			// assert if should error and quit
			if tc.ShouldError {
				require.Error(t, err, "expected error when comparing latest versions")
				return
			}
			// success path
			require.NoError(t, err, "unexpected error when comparing latest versions")
			require.Equal(t, tc.ExpectedOutdated, latest.Outdated, "outdated value not as expected")
			require.Equal(t, tc.LatestVersion, latest.LatestVersion, "latest version not as expected")
			require.Equal(t, tc.CurrentVersion, latest.CurrentVersion, "current version not as expected")
		})
	}
}
