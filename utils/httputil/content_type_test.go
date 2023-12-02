package httputil

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestExtensionsByType(t *testing.T) {

	t.Run("image type", func(t *testing.T) {
		mine := ExtensionsByType("m.jpg")
		assert.Equal(t, "image/jpeg", mine)
	})

	t.Run("image type", func(t *testing.T) {
		mine := ExtensionsByType("m.txt")
		assert.Equal(t, "text/plain", mine)
	})

	t.Run("custome type", func(t *testing.T) {
		mine := ExtensionsByType("m.aaaa")
		assert.Equal(t, "application/octet-stream", mine)
	})
}
