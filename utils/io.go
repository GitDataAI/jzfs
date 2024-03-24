package utils

import "io"

var _ io.ReadCloser = (*CloserWraper)(nil)

type CloserWraper struct {
	Reader io.Reader
}

func (c CloserWraper) Read(p []byte) (int, error) {
	return c.Reader.Read(p)
}

func (c CloserWraper) Close() error {
	return nil
}
