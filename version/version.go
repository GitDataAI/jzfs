package version

import (
	"regexp"
)

// CurrentCommit current program commit
var CurrentCommit string

// BuildVersion program version
var BuildVersion = "dev"

// UserVersion return build version and current commit
func UserVersion() string {
	return BuildVersion + CurrentCommit
}

var versionRegex, _ = regexp.Compile(`^v[0-9]+\.[0-9]+\.[0-9]?`)

func IsVersionUnreleased() bool {
	return !versionRegex.Match([]byte(UserVersion()))
}
