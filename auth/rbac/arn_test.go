package rbac_test

import (
	"testing"

	"github.com/GitDataAI/jiaozifs/auth/rbac"
)

func TestParseARN(t *testing.T) {
	cases := []struct {
		Input string
		Arn   rbac.Arn
		Error bool
	}{
		{Input: "", Error: true},
		{Input: "arn:gitdata:jiaozifs", Error: true},
		{Input: "arn:gitdata:jiaozifs:a:b:myrepo", Arn: rbac.Arn{
			Partition:  "gitdata",
			Service:    "jiaozifs",
			Region:     "a",
			AccountID:  "b",
			ResourceID: "myrepo"}},
		{Input: "arn:gitdata:jiaozifs:a::myrepo", Arn: rbac.Arn{
			Partition:  "gitdata",
			Service:    "jiaozifs",
			Region:     "a",
			AccountID:  "",
			ResourceID: "myrepo"}},
		{Input: "arn:gitdata:jiaozifs::b:myrepo", Arn: rbac.Arn{
			Partition:  "gitdata",
			Service:    "jiaozifs",
			Region:     "",
			AccountID:  "b",
			ResourceID: "myrepo"}},
		{Input: "arn:gitdata:jiaozifs:::myrepo", Arn: rbac.Arn{
			Partition:  "gitdata",
			Service:    "jiaozifs",
			Region:     "",
			AccountID:  "",
			ResourceID: "myrepo"}},
		{Input: "arn:gitdata:jiaozifs:::myrepo/branch/file:with:colon", Arn: rbac.Arn{
			Partition:  "gitdata",
			Service:    "jiaozifs",
			Region:     "",
			AccountID:  "",
			ResourceID: "myrepo/branch/file:with:colon"}},
	}

	for _, c := range cases {
		got, err := rbac.ParseARN(c.Input)
		if err != nil && !c.Error {
			t.Fatalf("got unexpected error parsing arn: \"%s\": \"%s\"", c.Input, err)
		} else if err != nil {
			continue
		} else if c.Error {
			t.Fatalf("expected error parsing arn: \"%s\"", c.Input)
		}
		if got.AccountID != c.Arn.AccountID {
			t.Fatalf("got unexpected account ID parsing arn: \"%s\": \"%s\" (expected \"%s\")", c.Input, got.AccountID, c.Arn.AccountID)
		}
		if got.Region != c.Arn.Region {
			t.Fatalf("got unexpected region parsing arn: \"%s\": \"%s\" (expected \"%s\")", c.Input, got.Region, c.Arn.Region)
		}
		if got.Partition != c.Arn.Partition {
			t.Fatalf("got unexpected partition parsing arn: \"%s\": \"%s\" (expected \"%s\")", c.Input, got.Partition, c.Arn.Partition)
		}
		if got.Service != c.Arn.Service {
			t.Fatalf("got unexpected service parsing arn: \"%s\": \"%s\" (expected \"%s\")", c.Input, got.Service, c.Arn.Service)
		}
		if got.ResourceID != c.Arn.ResourceID {
			t.Fatalf("got unexpected resource ID parsing arn: \"%s\": \"%s\" (expected \"%s\")", c.Input, got.ResourceID, c.Arn.ResourceID)
		}
	}
}

func TestArnMatch(t *testing.T) {
	cases := []struct {
		InputSource      string
		InputDestination string
		Match            bool
	}{
		{"arn:gitdata:jiaozifs::b:myrepo", "arn:gitdata:jiaozifs::b:myrepo", true},
		{"arn:gitdata:jiaozifs::b:*", "arn:gitdata:jiaozifs::b:myrepo", true},
		{"arn:gitdata:jiaozifs::b:my*", "arn:gitdata:jiaozifs::b:myrepo", true},
		{"arn:gitdata:jiaozifs::b:my*po", "arn:gitdata:jiaozifs::b:myrepo", true},
		{"arn:gitdata:jiaozifs::b:our*", "arn:gitdata:jiaozifs::b:myrepo", false},
		{"arn:gitdata:jiaozifs::b:my*own", "arn:gitdata:jiaozifs::b:myrepo", false},
		{"arn:gitdata:jiaozifs::b:myrepo", "arn:gitdata:jiaozifs::b:*", false},
		{"arn:gitdata:jiaozifs:::*", "arn:gitdata:jiaozifs:::*", true},
		{"arn:gitdata:repo", "arn:gitdata:repo", false},
	}

	for _, c := range cases {
		got := rbac.ArnMatch(c.InputSource, c.InputDestination)
		if got != c.Match {
			t.Fatalf("expected match %v, got %v on source = %s, destination = %s", c.Match, got, c.InputSource, c.InputDestination)
		}
	}
}
