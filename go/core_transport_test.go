//go:build cgo

package openapi

import (
	"bytes"
	"context"
	"io"
	"mime/multipart"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestCoreRoundTripper_JSONAcceptUsesJSONPath(t *testing.T) {
	mux := http.NewServeMux()
	mux.HandleFunc("/v1/x", func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		_, _ = w.Write([]byte(`{"ok":true}`))
	})
	srv := httptest.NewServer(mux)
	t.Cleanup(srv.Close)

	base := strings.TrimSuffix(srv.URL, "/")
	token := base + "_openapp_testsecret"

	cli, err := NewAPIClient(token)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = cli.Close() })

	req, err := http.NewRequestWithContext(context.Background(), http.MethodGet, srv.URL+"/v1/x", nil)
	if err != nil {
		t.Fatal(err)
	}
	req.Header.Set("Accept", "application/json")

	resp, err := cli.cfg.HTTPClient.Transport.RoundTrip(req)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = resp.Body.Close() })
	if resp.StatusCode != 200 {
		t.Fatalf("status %d", resp.StatusCode)
	}
	if ct := resp.Header.Get("Content-Type"); !strings.Contains(ct, "application/json") {
		t.Fatalf("Content-Type %q want json", ct)
	}
	b, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(string(b), `"ok"`) {
		t.Fatalf("body %q", b)
	}
}

func TestCoreRoundTripper_AcceptEventStreamUsesStreamPath(t *testing.T) {
	mux := http.NewServeMux()
	mux.HandleFunc("/sse", func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "text/event-stream")
		_, _ = w.Write([]byte("event: ping\ndata: hi\n\n"))
	})
	srv := httptest.NewServer(mux)
	t.Cleanup(srv.Close)

	base := strings.TrimSuffix(srv.URL, "/")
	token := base + "_openapp_testsecret"

	cli, err := NewAPIClient(token)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = cli.Close() })

	req, err := http.NewRequestWithContext(context.Background(), http.MethodGet, srv.URL+"/sse", nil)
	if err != nil {
		t.Fatal(err)
	}
	req.Header.Set("Accept", "text/event-stream")

	resp, err := cli.cfg.HTTPClient.Transport.RoundTrip(req)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = resp.Body.Close() })
	if resp.StatusCode != 200 {
		t.Fatalf("status %d", resp.StatusCode)
	}
	if ct := resp.Header.Get("Content-Type"); !strings.Contains(ct, "text/event-stream") {
		t.Fatalf("Content-Type %q want event-stream", ct)
	}
	b, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatal(err)
	}
	body := string(b)
	if !strings.Contains(body, "event: ping") || !strings.Contains(body, "data: hi") {
		t.Fatalf("body %q", body)
	}
}

func TestCoreRoundTripper_StreamOptInHeaderOverJSONAccept(t *testing.T) {
	mux := http.NewServeMux()
	mux.HandleFunc("/bin", func(w http.ResponseWriter, _ *http.Request) {
		_, _ = w.Write([]byte{0, 1, 2, 3})
	})
	srv := httptest.NewServer(mux)
	t.Cleanup(srv.Close)

	base := strings.TrimSuffix(srv.URL, "/")
	token := base + "_openapp_testsecret"

	cli, err := NewAPIClient(token)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = cli.Close() })

	req, err := http.NewRequestWithContext(context.Background(), http.MethodGet, srv.URL+"/bin", nil)
	if err != nil {
		t.Fatal(err)
	}
	req.Header.Set("Accept", "application/json")
	req.Header.Set(StreamResponseHeader, "1")

	resp, err := cli.cfg.HTTPClient.Transport.RoundTrip(req)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = resp.Body.Close() })
	if resp.StatusCode != 200 {
		t.Fatalf("status %d", resp.StatusCode)
	}
	b, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatal(err)
	}
	if len(b) != 4 {
		t.Fatalf("len %d", len(b))
	}
	if ct := resp.Header.Get("Content-Type"); ct != "application/octet-stream" {
		t.Fatalf("Content-Type %q want application/octet-stream", ct)
	}
}

func TestCoreRoundTripper_MultipartUsesRawPath(t *testing.T) {
	mux := http.NewServeMux()
	mux.HandleFunc("/upload", func(w http.ResponseWriter, r *http.Request) {
		ct := r.Header.Get("Content-Type")
		if !strings.Contains(strings.ToLower(ct), "multipart/") {
			http.Error(w, "want multipart", http.StatusBadRequest)
			return
		}
		w.Header().Set("Content-Type", "application/json")
		_, _ = w.Write([]byte(`{"ok":true}`))
	})
	srv := httptest.NewServer(mux)
	t.Cleanup(srv.Close)

	base := strings.TrimSuffix(srv.URL, "/")
	token := base + "_openapp_testsecret"

	cli, err := NewAPIClient(token)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = cli.Close() })

	var buf bytes.Buffer
	mw := multipart.NewWriter(&buf)
	if err := mw.WriteField("name", "test"); err != nil {
		t.Fatal(err)
	}
	if err := mw.Close(); err != nil {
		t.Fatal(err)
	}
	req, err := http.NewRequestWithContext(context.Background(), http.MethodPost, srv.URL+"/upload", &buf)
	if err != nil {
		t.Fatal(err)
	}
	req.Header.Set("Content-Type", mw.FormDataContentType())
	req.Header.Set("Accept", "application/json")

	resp, err := cli.cfg.HTTPClient.Transport.RoundTrip(req)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = resp.Body.Close() })
	if resp.StatusCode != 200 {
		t.Fatalf("status %d", resp.StatusCode)
	}
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(string(body), `"ok"`) {
		t.Fatalf("body %s", body)
	}
}

func TestCoreRoundTripper_NilRequestReturnsError(t *testing.T) {
	tr := &coreRoundTripper{}
	resp, err := tr.RoundTrip(nil)
	if err == nil {
		t.Fatal("expected error for nil request")
	}
	if resp != nil {
		t.Fatalf("expected nil response, got %#v", resp)
	}
	if !strings.Contains(err.Error(), "nil http.Request") {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestReadRequestBody_NilBodyReturnsNil(t *testing.T) {
	req, err := http.NewRequestWithContext(context.Background(), http.MethodPost, "https://example.test/x", nil)
	if err != nil {
		t.Fatal(err)
	}
	b, err := readRequestBody(req)
	if err != nil {
		t.Fatal(err)
	}
	if b != nil {
		t.Fatalf("body %v want nil", b)
	}
}

func TestWantsCoreStreamTransport_RecognizesHeaderAndAccept(t *testing.T) {
	req, err := http.NewRequestWithContext(context.Background(), http.MethodGet, "https://example.test/stream", nil)
	if err != nil {
		t.Fatal(err)
	}
	if wantsCoreStreamTransport(req) {
		t.Fatal("expected stream transport to be off by default")
	}

	req.Header.Set("Accept", "application/json, text/event-stream")
	if !wantsCoreStreamTransport(req) {
		t.Fatal("expected stream transport for event-stream accept")
	}

	req.Header.Set("Accept", "application/json")
	req.Header.Set(StreamResponseHeader, "1")
	if !wantsCoreStreamTransport(req) {
		t.Fatal("expected stream transport for explicit opt-in header")
	}
}
