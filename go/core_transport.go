//go:build cgo

package openapi

import (
	"fmt"
	"io"
	"net/http"
	"strings"
	"sync/atomic"

	"github.com/tomers/openapp-sdk/go/bridge"
)

// NewAPIClient builds a client whose outbound HTTP is implemented entirely by
// openapp-sdk-core (via cgo and openapp-sdk-core-c-bridge). apiKey must use OpenApp’s
// encoded form ({base_url}_openapp_{secret}) unless you rely on core defaults only.
//
// JSON responses use the bridge sync JSON path. Multipart uploads use the bridge raw-body
// path (same core transport as other HTTP calls). Byte streams (for example SSE) use the
// bridge streaming path when [http.Request.Header] “Accept” includes
// text/event-stream or application/octet-stream, or when [StreamResponseHeader] is “1”
// (for generated clients that only send Accept: application/json).
//
// Call [APIClient.Close] when done to release bridge runtime resources.
func NewAPIClient(apiKey string) (*APIClient, error) {
	cfg := NewConfiguration()
	rt, err := bridge.NewRuntime()
	if err != nil {
		return nil, fmt.Errorf("openapp sdk: runtime: %w", err)
	}
	bc, err := bridge.NewClient(rt, apiKey)
	if err != nil {
		rt.Close()
		return nil, fmt.Errorf("openapp sdk: client: %w", err)
	}

	tr := &coreRoundTripper{rt: rt, bc: bc}
	cfg.HTTPClient = &http.Client{Transport: tr}
	return newAPIClientFromConfig(cfg), nil
}

type coreRoundTripper struct {
	rt *bridge.Runtime
	bc *bridge.Client
}

func (t *coreRoundTripper) close() error {
	if t.bc != nil {
		t.bc.Close()
		t.bc = nil
	}
	if t.rt != nil {
		t.rt.Close()
		t.rt = nil
	}
	return nil
}

// StreamResponseHeader requests the core streaming path (C bridge
// openapp_sdk_client_request_stream_async) even when [http.Request.Header] "Accept"
// is only application/json — for example OpenAPI-generated clients that do not emit
// text/event-stream. Hand-built requests can set "Accept" to text/event-stream or
// application/octet-stream instead.
const StreamResponseHeader = "X-OpenApp-Stream-Response"

func (t *coreRoundTripper) RoundTrip(req *http.Request) (*http.Response, error) {
	if req == nil {
		return nil, fmt.Errorf("openapp sdk: nil http.Request")
	}

	path := req.URL.Path
	if req.URL.RawQuery != "" {
		path += "?" + req.URL.RawQuery
	}

	bodyBytes, err := readRequestBody(req)
	if err != nil {
		return nil, err
	}

	ct := req.Header.Get("Content-Type")
	if isMultipartContentType(ct) {
		return t.roundTripCoreRaw(req, path, ct, bodyBytes)
	}

	var bodyStr *string
	if len(bodyBytes) > 0 {
		s := string(bodyBytes)
		bodyStr = &s
	}

	if wantsCoreStreamTransport(req) {
		return t.roundTripCoreStream(req, path, bodyStr)
	}

	return t.roundTripCoreJSON(req, path, bodyStr)
}

func readRequestBody(req *http.Request) ([]byte, error) {
	if req.Body == nil {
		return nil, nil
	}
	b, err := io.ReadAll(req.Body)
	if err != nil {
		return nil, err
	}
	_ = req.Body.Close()
	return b, nil
}

func isMultipartContentType(ct string) bool {
	return ct != "" && strings.Contains(strings.ToLower(ct), "multipart/")
}

func wantsCoreStreamTransport(req *http.Request) bool {
	if req.Header.Get(StreamResponseHeader) == "1" {
		return true
	}
	accept := strings.ToLower(req.Header.Get("Accept"))
	if strings.Contains(accept, "text/event-stream") {
		return true
	}
	if strings.Contains(accept, "application/octet-stream") {
		return true
	}
	return false
}

func streamResponseContentType(req *http.Request) string {
	accept := strings.ToLower(req.Header.Get("Accept"))
	switch {
	case strings.Contains(accept, "text/event-stream"):
		return "text/event-stream"
	case strings.Contains(accept, "application/octet-stream"):
		return "application/octet-stream"
	default:
		return "application/octet-stream"
	}
}

type streamKickoff struct {
	code int
	err  error
}

func (t *coreRoundTripper) roundTripCoreStream(req *http.Request, path string, bodyStr *string) (*http.Response, error) {
	pr, pw := io.Pipe()

	kick := make(chan streamKickoff, 1)
	var opened int32

	go func() {
		var closeErr error
		defer func() {
			if closeErr != nil {
				_ = pw.CloseWithError(closeErr)
			} else {
				_ = pw.Close()
			}
		}()

		_, _, err := t.bc.StreamRequestWithHTTPCallback(req.Method, path, bodyStr, func(code int) {
			atomic.StoreInt32(&opened, 1)
			kick <- streamKickoff{code: code}
		}, func(p []byte) (stop bool) {
			if req.Context().Err() != nil {
				closeErr = req.Context().Err()
				return true
			}
			_, werr := pw.Write(p)
			if werr != nil {
				closeErr = werr
				return true
			}
			return false
		})

		if err != nil {
			if atomic.LoadInt32(&opened) == 0 {
				kick <- streamKickoff{err: err}
			} else {
				closeErr = err
			}
		}
	}()

	select {
	case k := <-kick:
		if k.err != nil {
			_ = pr.Close()
			return nil, fmt.Errorf("openapp sdk: stream transport: %w", k.err)
		}
		code := k.code
		if code == 0 {
			code = http.StatusInternalServerError
		}
		hdr := http.Header{}
		hdr.Set("Content-Type", streamResponseContentType(req))
		return &http.Response{
			StatusCode:    code,
			Status:        httpStatusLine(code),
			Proto:         "HTTP/1.1",
			ProtoMajor:    1,
			ProtoMinor:    1,
			Header:        hdr,
			Body:          pr,
			Request:       req,
			ContentLength: -1,
		}, nil
	case <-req.Context().Done():
		_ = pr.Close()
		return nil, req.Context().Err()
	}
}

func (t *coreRoundTripper) roundTripCoreRaw(req *http.Request, path, contentType string, body []byte) (*http.Response, error) {
	st, httpSt, bodyText, errBR := t.bc.JSONRequestRaw(req.Method, path, contentType, body)
	if errBR != nil && strings.Contains(errBR.Error(), "null body pointer") {
		return nil, errBR
	}

	code := httpSt
	if code == 0 {
		code = http.StatusInternalServerError
	}

	hdr := http.Header{}
	if bodyText != "" || code != http.StatusNoContent {
		hdr.Set("Content-Type", "application/json")
	}

	resp := &http.Response{
		StatusCode:    code,
		Status:        httpStatusLine(code),
		Proto:         "HTTP/1.1",
		ProtoMajor:    1,
		ProtoMinor:    1,
		Header:        hdr,
		Body:          io.NopCloser(strings.NewReader(bodyText)),
		Request:       req,
		ContentLength: int64(len(bodyText)),
	}

	_ = st

	return resp, nil
}

func (t *coreRoundTripper) roundTripCoreJSON(req *http.Request, path string, bodyStr *string) (*http.Response, error) {
	st, httpSt, body, errBR := t.bc.JSONRequest(req.Method, path, bodyStr)
	if errBR != nil && strings.Contains(errBR.Error(), "null body pointer") {
		return nil, errBR
	}

	code := httpSt
	if code == 0 {
		code = http.StatusInternalServerError
	}

	hdr := http.Header{}
	if body != "" || code != http.StatusNoContent {
		hdr.Set("Content-Type", "application/json")
	}

	resp := &http.Response{
		StatusCode:    code,
		Status:        httpStatusLine(code),
		Proto:         "HTTP/1.1",
		ProtoMajor:    1,
		ProtoMinor:    1,
		Header:        hdr,
		Body:          io.NopCloser(strings.NewReader(body)),
		Request:       req,
		ContentLength: int64(len(body)),
	}

	_ = st
	_ = errBR

	return resp, nil
}

func httpStatusLine(code int) string {
	if txt := http.StatusText(code); txt != "" {
		return fmt.Sprintf("%d %s", code, txt)
	}
	return fmt.Sprintf("%d", code)
}
