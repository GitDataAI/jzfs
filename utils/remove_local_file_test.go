package utils

import (
	"os"
	"path/filepath"
	"testing"
)

func TestRemoveLocalFiles(t *testing.T) {
	// Create a temporary test file
	content := []byte("This is a test file.")
	testFilename := "testfile.txt"
	tempDir, err := os.MkdirTemp("", "testdir")
	if err != nil {
		t.Fatalf("Error creating temporary directory: %v", err)
	}
	defer os.RemoveAll(tempDir)

	testFilePath := filepath.Join(tempDir, testFilename)
	err = os.WriteFile(testFilePath, content, 0644)
	if err != nil {
		t.Fatalf("Error creating temporary file: %v", err)
	}

	// Test removing a file
	err = RemoveLocalFiles(tempDir, testFilename)
	if err != nil {
		t.Errorf("Error removing file: %v", err)
	}

	// Check if the file no longer exists after removal
	_, err = os.Stat(testFilePath)
	if !os.IsNotExist(err) {
		t.Errorf("File still exists after removal")
	}

	// Test removing a directory
	err = RemoveLocalFiles(tempDir, "")
	if err != nil {
		t.Errorf("Error removing directory: %v", err)
	}

	// Check if the directory no longer exists after removal
	_, err = os.Stat(tempDir)
	if !os.IsNotExist(err) {
		t.Errorf("Directory still exists after removal")
	}
}
