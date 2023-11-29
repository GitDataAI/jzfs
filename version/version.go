package version

// CurrentCommit current program commit
var CurrentCommit string

// BuildVersion program version
const BuildVersion = "0.0.1"

// UserVersion return build version and current commit
func UserVersion() string {
	return BuildVersion + CurrentCommit
}
