package utils

import (
	"testing"
	"time"
)

func TestPaginationFor(t *testing.T) {
	// Test case 1: No more results
	results1 := []struct {
		UpdatedAt time.Time
		Name      string
	}{}
	pagination1 := PaginationFor(false, results1, "UpdatedAt")
	if pagination1.HasMore || pagination1.Results != 0 || pagination1.NextOffset != "" {
		t.Errorf("Test case 1 failed: Expected no more results")
	}
	// Test case 2: With more results and "UpdatedAt" as the field
	results2 := []struct {
		UpdatedAt time.Time
		Name      string
	}{
		{time.Now(), "Item1"},
		{time.Now().Add(1 * time.Hour), "Item2"},
	}
	pagination2 := PaginationFor(true, results2, "UpdatedAt")
	if !pagination2.HasMore || pagination2.Results != 2 || pagination2.NextOffset == "" {
		t.Errorf("Test case 2 failed: Expected more results with valid NextOffset")
	}

	// Test case 3: With more results and "Name" as the field
	results3 := []struct {
		UpdatedAt time.Time
		Name      string
	}{
		{time.Now(), "Item1"},
		{time.Now().Add(1 * time.Hour), "Item2"},
	}
	pagination3 := PaginationFor(true, results3, "Name")
	if !pagination3.HasMore || pagination3.Results != 2 || pagination3.NextOffset == "" {
		t.Errorf("Test case 3 failed: Expected more results with valid NextOffset")
	}
}
