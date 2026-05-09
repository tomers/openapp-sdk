//go:build cgo

package bridge

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestBridgeGetRoundtrip(t *testing.T) {
	InitTelemetry()
	InitTelemetry()

	mux := http.NewServeMux()
	mux.HandleFunc("/ping", func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		_, _ = w.Write([]byte(`{"hello":"world"}`))
	})
	srv := httptest.NewServer(mux)
	t.Cleanup(srv.Close)

	base := strings.TrimSuffix(srv.URL, "/")
	token := base + "_openapp_testsecret"

	rt, err := NewRuntime()
	if err != nil {
		t.Fatalf("NewRuntime: %v", err)
	}
	t.Cleanup(func() { rt.Close() })

	client, err := NewClient(rt, token)
	if err != nil {
		t.Fatalf("NewClient: %v", err)
	}
	t.Cleanup(func() { client.Close() })

	st, httpSt, body, err := client.JSONRequest("GET", "/ping", nil)
	if err != nil {
		t.Fatalf("JSONRequest: %v", err)
	}
	if st != StatusOK {
		t.Fatalf("status: got %v want %v", st, StatusOK)
	}
	if httpSt != 200 {
		t.Fatalf("http: got %d want 200", httpSt)
	}
	var v struct {
		Hello string `json:"hello"`
	}
	if uerr := json.Unmarshal([]byte(body), &v); uerr != nil {
		t.Fatalf("json: %v", uerr)
	}
	if v.Hello != "world" {
		t.Fatalf("hello: got %q", v.Hello)
	}
}

func TestBridgeJSONRequestAsyncRoundtrip(t *testing.T) {
	mux := http.NewServeMux()
	mux.HandleFunc("/ping", func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		_, _ = w.Write([]byte(`{"hello":"async"}`))
	})
	srv := httptest.NewServer(mux)
	t.Cleanup(srv.Close)

	base := strings.TrimSuffix(srv.URL, "/")
	token := base + "_openapp_testsecret"

	rt, err := NewRuntime()
	if err != nil {
		t.Fatalf("NewRuntime: %v", err)
	}
	t.Cleanup(func() { rt.Close() })

	client, err := NewClient(rt, token)
	if err != nil {
		t.Fatalf("NewClient: %v", err)
	}
	t.Cleanup(func() { client.Close() })

	st, httpSt, body, err := client.JSONRequestAsync("GET", "/ping", nil)
	if err != nil {
		t.Fatalf("JSONRequestAsync: %v", err)
	}
	if st != StatusOK {
		t.Fatalf("status: got %v want %v", st, StatusOK)
	}
	if httpSt != 200 {
		t.Fatalf("http: got %d want 200", httpSt)
	}
	var v struct {
		Hello string `json:"hello"`
	}
	if uerr := json.Unmarshal([]byte(body), &v); uerr != nil {
		t.Fatalf("json: %v", uerr)
	}
	if v.Hello != "async" {
		t.Fatalf("hello: got %q", v.Hello)
	}
}

func TestBridgeStreamRequestDeliverChunks(t *testing.T) {
	mux := http.NewServeMux()
	mux.HandleFunc("/sse", func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "text/event-stream")
		_, _ = w.Write([]byte("event: ping\ndata: hi\n\n"))
	})
	srv := httptest.NewServer(mux)
	t.Cleanup(srv.Close)

	base := strings.TrimSuffix(srv.URL, "/")
	token := base + "_openapp_testsecret"

	rt, err := NewRuntime()
	if err != nil {
		t.Fatalf("NewRuntime: %v", err)
	}
	t.Cleanup(func() { rt.Close() })

	client, err := NewClient(rt, token)
	if err != nil {
		t.Fatalf("NewClient: %v", err)
	}
	t.Cleanup(func() { client.Close() })

	var buf strings.Builder
	st, httpSt, err := client.StreamRequest("GET", "/sse", nil, func(p []byte) (stop bool) {
		buf.Write(p)
		return false
	})
	if err != nil {
		t.Fatalf("StreamRequest: %v", err)
	}
	if st != StatusOK {
		t.Fatalf("status: got %v want %v", st, StatusOK)
	}
	if httpSt != 200 {
		t.Fatalf("http: got %d want 200", httpSt)
	}
	body := buf.String()
	if !strings.Contains(body, "event: ping") || !strings.Contains(body, "data: hi") {
		t.Fatalf("unexpected stream: %q", body)
	}
}
