//go:build cgo

package openapi

import (
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync"
	"testing"
)

type recordedRequest struct {
	method  string
	path    string
	rawPath string
	header  http.Header
}

// newRecordingServer serves body for every request and records what reached the wire.
func newRecordingServer(t *testing.T, body string) (*httptest.Server, func() []recordedRequest) {
	t.Helper()
	var mu sync.Mutex
	var seen []recordedRequest
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		mu.Lock()
		seen = append(seen, recordedRequest{
			method:  r.Method,
			path:    r.URL.Path,
			rawPath: r.URL.EscapedPath(),
			header:  r.Header.Clone(),
		})
		mu.Unlock()
		w.Header().Set("Content-Type", "application/json")
		_, _ = w.Write([]byte(body))
	}))
	t.Cleanup(srv.Close)
	return srv, func() []recordedRequest {
		mu.Lock()
		defer mu.Unlock()
		return append([]recordedRequest(nil), seen...)
	}
}

func newTestClient(t *testing.T, token string) *APIClient {
	t.Helper()
	cli, err := NewAPIClient(token)
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = cli.Close() })
	return cli
}

func onlyRequest(t *testing.T, seen []recordedRequest) recordedRequest {
	t.Helper()
	if len(seen) != 1 {
		t.Fatalf("server saw %d requests, want 1: %#v", len(seen), seen)
	}
	return seen[0]
}

func TestGeneratedCall_SendsAPIKeyHeaderAndResolvesAgainstAPIRoot(t *testing.T) {
	srv, seen := newRecordingServer(t, `{"environment":"test","version":"1.2.3"}`)
	token := strings.TrimSuffix(srv.URL, "/") + "_openapp_testsecret"
	cli := newTestClient(t, token)

	status, httpResp, err := cli.StatusAPI.GetBackendStatus(context.Background()).Execute()
	if err != nil {
		t.Fatalf("GetBackendStatus: %v", err)
	}
	if httpResp.StatusCode != http.StatusOK {
		t.Fatalf("status %d", httpResp.StatusCode)
	}
	if status.Version != "1.2.3" {
		t.Fatalf("version %q", status.Version)
	}

	got := onlyRequest(t, seen())
	if got.method != http.MethodGet || got.path != "/api/v1/status" {
		t.Fatalf("request %s %s, want GET /api/v1/status", got.method, got.path)
	}
	if v := got.header.Get("X-API-Key"); v != token {
		t.Fatalf("X-API-Key %q, want full token %q", v, token)
	}
	if v, ok := got.header["Authorization"]; ok {
		t.Fatalf("unexpected Authorization header %q", v)
	}
}

func TestGeneratedCall_PreservesEscapedPathParameters(t *testing.T) {
	srv, seen := newRecordingServer(t, "")
	cli := newTestClient(t, strings.TrimSuffix(srv.URL, "/")+"_openapp_testsecret")

	if _, err := cli.AgentsAPI.DeleteAlias(context.Background(), "org1", "a/b").Execute(); err != nil {
		t.Fatalf("DeleteAlias: %v", err)
	}

	got := onlyRequest(t, seen())
	if got.method != http.MethodDelete || got.rawPath != "/api/v1/orgs/org1/aliases/a%2Fb" {
		t.Fatalf("request %s %s, want DELETE /api/v1/orgs/org1/aliases/a%%2Fb", got.method, got.rawPath)
	}
}

func TestNewAPIClientWithBaseURL_OverridesDerivedRoot(t *testing.T) {
	srv, seen := newRecordingServer(t, `{"environment":"test","version":"1.2.3"}`)
	// The key embeds an unreachable in-container origin, as dev-env keys do; the explicit
	// root is where requests must go.
	token := "http://oathkeeper.invalid:4455_openapp_testsecret"
	cli, err := NewAPIClientWithBaseURL(token, strings.TrimSuffix(srv.URL, "/")+"/api/v1")
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = cli.Close() })

	if _, _, err := cli.StatusAPI.GetBackendStatus(context.Background()).Execute(); err != nil {
		t.Fatalf("GetBackendStatus: %v", err)
	}

	got := onlyRequest(t, seen())
	if got.path != "/api/v1/status" {
		t.Fatalf("path %q, want /api/v1/status", got.path)
	}
	if v := got.header.Get("X-API-Key"); v != token {
		t.Fatalf("X-API-Key %q, want %q", v, token)
	}
}

func TestNewAPIClientWithBaseURL_RejectsEmptyBaseURL(t *testing.T) {
	if _, err := NewAPIClientWithBaseURL("https://openapp.house_openapp_x", ""); err == nil {
		t.Fatal("expected an error for an empty base URL")
	}
}
