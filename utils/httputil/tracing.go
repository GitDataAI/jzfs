package httputil

import (
	"io"
	"net/http"
)

const (
	MaxBodyBytes                      = 750               // Log lines will be < 2KiB
	RequestTracingMaxRequestBodySize  = 1024 * 1024 * 50  // 50KB
	RequestTracingMaxResponseBodySize = 1024 * 1024 * 150 // 150KB
)

type CappedBuffer struct {
	SizeBytes int
	cursor    int
	Buffer    []byte
}

func (c *CappedBuffer) Write(p []byte) (n int, err error) {
	// pretend to write the whole thing, but only write SizeBytes
	if c.cursor >= c.SizeBytes {
		return len(p), nil
	}
	if c.Buffer == nil {
		c.Buffer = make([]byte, 0)
	}
	var written int
	if len(p) > (c.SizeBytes - c.cursor) {
		c.Buffer = append(c.Buffer, p[0:(c.SizeBytes-c.cursor)]...)
		written = c.SizeBytes - c.cursor
	} else {
		c.Buffer = append(c.Buffer, p...)
		written = len(p)
	}
	c.cursor += written
	return len(p), nil
}

type responseTracingWriter struct {
	StatusCode   int
	ResponseSize int64
	BodyRecorder *CappedBuffer

	Writer      http.ResponseWriter
	multiWriter io.Writer
}

func newResponseTracingWriter(w http.ResponseWriter, sizeInBytes int) *responseTracingWriter {
	buf := &CappedBuffer{
		SizeBytes: sizeInBytes,
	}
	mw := io.MultiWriter(w, buf)
	return &responseTracingWriter{
		StatusCode:   http.StatusOK,
		BodyRecorder: buf,
		Writer:       w,
		multiWriter:  mw,
	}
}

func (w *responseTracingWriter) Header() http.Header {
	return w.Writer.Header()
}

func (w *responseTracingWriter) Write(data []byte) (int, error) {
	return w.multiWriter.Write(data)
}

func (w *responseTracingWriter) WriteHeader(statusCode int) {
	w.StatusCode = statusCode
	w.Writer.WriteHeader(statusCode)
}

type requestBodyTracer struct {
	body         io.ReadCloser
	bodyRecorder *CappedBuffer
	tee          io.Reader
}

func newRequestBodyTracer(body io.ReadCloser, sizeInBytes int) *requestBodyTracer {
	w := &CappedBuffer{
		SizeBytes: sizeInBytes,
	}
	return &requestBodyTracer{
		body:         body,
		bodyRecorder: w,
		tee:          io.TeeReader(body, w),
	}
}

func (r *requestBodyTracer) Read(p []byte) (n int, err error) {
	return r.tee.Read(p)
}

func (r *requestBodyTracer) Close() error {
	return r.body.Close()
}

func presentBody(body []byte) string {
	if len(body) > MaxBodyBytes {
		body = body[:MaxBodyBytes]
	}
	return string(body)
}
