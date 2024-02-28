package rbac_test

import (
	"testing"

	"github.com/jiaozifs/jiaozifs/auth/rbac"
)

func TestParseARN(t *testing.T) {
	cases := []struct {
		Input string
		Arn   rbac.Arn
		Error bool
	}{
		{Input: "", Error: true},
		{Input: "arn:jiaozifs:repo", Error: true},
		{Input: "arn:jiaozifs:repos:a:b:myrepo", Arn: rbac.Arn{
			Partition:  "jiaozifs",
			Service:    "repos",
			Region:     "a",
			AccountID:  "b",
			ResourceID: "myrepo"}},
		{Input: "arn:jiaozifs:repos:a::myrepo", Arn: rbac.Arn{
			Partition:  "jiaozifs",
			Service:    "repos",
			Region:     "a",
			AccountID:  "",
			ResourceID: "myrepo"}},
		{Input: "arn:jiaozifs:repos::b:myrepo", Arn: rbac.Arn{
			Partition:  "jiaozifs",
			Service:    "repos",
			Region:     "",
			AccountID:  "b",
			ResourceID: "myrepo"}},
		{Input: "arn:jiaozifs:repos:::myrepo", Arn: rbac.Arn{
			Partition:  "jiaozifs",
			Service:    "repos",
			Region:     "",
			AccountID:  "",
			ResourceID: "myrepo"}},
		{Input: "arn:jiaozifs:fs:::myrepo/branch/file:with:colon", Arn: rbac.Arn{
			Partition:  "jiaozifs",
			Service:    "fs",
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
		{"arn:jiaozifs:repos::b:myrepo", "arn:jiaozifs:repos::b:myrepo", true},
		{"arn:jiaozifs:repos::b:*", "arn:jiaozifs:repos::b:myrepo", true},
		{"arn:jiaozifs:repos::b:my*", "arn:jiaozifs:repos::b:myrepo", true},
		{"arn:jiaozifs:repos::b:my*po", "arn:jiaozifs:repos::b:myrepo", true},
		{"arn:jiaozifs:repos::b:our*", "arn:jiaozifs:repos::b:myrepo", false},
		{"arn:jiaozifs:repos::b:my*own", "arn:jiaozifs:repos::b:myrepo", false},
		{"arn:jiaozifs:repos::b:myrepo", "arn:jiaozifs:repos::b:*", false},
		{"arn:jiaozifs:repo:::*", "arn:jiaozifs:repo:::*", true},
		{"arn:jiaozifs:repo", "arn:jiaozifs:repo", false},
	}

	for _, c := range cases {
		got := rbac.ArnMatch(c.InputSource, c.InputDestination)
		if got != c.Match {
			t.Fatalf("expected match %v, got %v on source = %s, destination = %s", c.Match, got, c.InputSource, c.InputDestination)
		}
	}
}
