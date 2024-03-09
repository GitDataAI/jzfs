package version

import (
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"net/url"
	"time"

	goversion "github.com/hashicorp/go-version"
)

const (
	latestVersionTimeout = 10 * time.Second

	DefaultReleasesURL = "https://github.com/GitDataAI/jiaozifs/releases"
	githubBaseURL      = "https://api.github.com/"

	GithubRepoOwner = "jiaozifs"
	GithubRepoName  = "jiaozifs"
)

var ErrHTTPStatus = errors.New("unexpected HTTP status code")

type RepositoryRelease struct {
	TagName    string `json:"tag_name,omitempty"`
	Name       string `json:"name,omitempty"`
	Draft      bool   `json:"draft,omitempty"`
	Prerelease bool   `json:"prerelease,omitempty"`
	ID         int64  `json:"id,omitempty"`
	URL        string `json:"url,omitempty"`
}

type LatestVersionResponse struct {
	CheckTime      time.Time `json:"check_time"`
	Outdated       bool      `json:"outdated"`
	LatestVersion  string    `json:"latest_version"`
	CurrentVersion string    `json:"current_version"`
}

type Source interface {
	FetchLatestVersion() (string, error)
}

type CachedVersionSource struct {
	Source        Source
	lastCheck     time.Time
	cachePeriod   time.Duration
	fetchErr      error
	fetchResponse string
}

func NewDefaultVersionSource(cachePeriod time.Duration) Source {
	gh := NewGithubReleases(GithubRepoOwner, GithubRepoName)
	return NewCachedSource(gh, cachePeriod)
}

func NewCachedSource(src Source, cachePeriod time.Duration) *CachedVersionSource {
	return &CachedVersionSource{
		Source:      src,
		cachePeriod: cachePeriod,
	}
}

func (cs *CachedVersionSource) FetchLatestVersion() (string, error) {
	if time.Since(cs.lastCheck) > cs.cachePeriod {
		cs.fetchResponse, cs.fetchErr = cs.Source.FetchLatestVersion()
		cs.lastCheck = time.Now()
	}
	return cs.fetchResponse, cs.fetchErr
}

type GithubReleases struct {
	owner      string
	repository string
}

func NewGithubReleases(owner, repository string) *GithubReleases {
	return &GithubReleases{
		owner:      owner,
		repository: repository,
	}
}

func (gh *GithubReleases) FetchLatestVersion() (string, error) {
	u, err := url.JoinPath(githubBaseURL, "repos", gh.owner, gh.repository, "releases", "latest")
	if err != nil {
		return "", err
	}

	req, err := http.NewRequest(http.MethodGet, u, nil)
	if err != nil {
		return "", err
	}

	client := &http.Client{
		Timeout: latestVersionTimeout,
	}

	resp, err := client.Do(req)
	if err != nil {
		return "", err
	}
	defer func() { _ = resp.Body.Close() }()

	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("unexpected HTTP response %d: %w", resp.StatusCode, ErrHTTPStatus)
	}

	var release RepositoryRelease
	if err := json.NewDecoder(resp.Body).Decode(&release); err != nil {
		return "", err
	}

	return release.TagName, nil
}

func CheckLatestVersion(targetVersion string) (*LatestVersionResponse, error) {
	targetV, err := goversion.NewVersion(targetVersion)
	if err != nil {
		return nil, fmt.Errorf("tag parse %s: %w", targetVersion, err)
	}

	if IsVersionUnreleased() {
		return &LatestVersionResponse{
			Outdated:       false,
			LatestVersion:  targetV.String(),
			CurrentVersion: UserVersion(),
		}, nil
	}

	currentV, err := goversion.NewVersion(UserVersion())
	if err != nil {
		return nil, fmt.Errorf("version parse %s: %w", UserVersion(), err)
	}

	return &LatestVersionResponse{
		Outdated:       currentV.LessThan(targetV),
		LatestVersion:  targetV.String(),
		CurrentVersion: UserVersion(),
	}, nil
}

type IChecker interface {
	CheckLatestVersion() (*LatestVersionResponse, error)
}

type Checker struct {
	Client         http.Client
	Version        string
	latestReleases Source
}

func NewVersionChecker() *Checker {
	return &Checker{
		Client:         http.Client{},
		Version:        UserVersion(),
		latestReleases: NewDefaultVersionSource(time.Hour),
	}
}

// CheckLatestVersion will return the latest version of the current package compared to the current version
func (a *Checker) CheckLatestVersion() (*LatestVersionResponse, error) {
	if a == nil || a.latestReleases == nil {
		return &LatestVersionResponse{}, nil
	}

	latest, err := a.latestReleases.FetchLatestVersion()
	if err != nil {
		return nil, err
	}
	result, err := CheckLatestVersion(latest)
	if err != nil {
		return nil, err
	}
	return result, nil
}
