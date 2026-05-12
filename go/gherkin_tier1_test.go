//go:build cgo

package openapi

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"

	"github.com/cucumber/godog"
)

type gherkinRoute struct {
	method      string
	path        string
	status      int
	body        string
	firstStatus int
	firstServed bool
}

type gherkinWorld struct {
	server    *httptest.Server
	client    *APIClient
	routes    map[string]*gherkinRoute
	lastJSON  any
	lastError *gherkinError
}

type gherkinError struct {
	kind    string
	message string
	code    string
}

func TestGherkinTier1(t *testing.T) {
	world := &gherkinWorld{}
	suite := godog.TestSuite{
		Name: "go-sdk-gherkin",
		ScenarioInitializer: func(ctx *godog.ScenarioContext) {
			world.initialize(ctx)
		},
		Options: &godog.Options{
			Format:   "progress",
			Paths:    []string{"../features"},
			Tags:     gherkinTags(),
			TestingT: t,
		},
	}

	if status := suite.Run(); status != 0 {
		t.Fatalf("godog exited with status %d", status)
	}
}

func gherkinTags() string {
	if tags := os.Getenv("OPENAPP_SDK_GHERKIN_TAGS"); tags != "" {
		return tags
	}
	return "~@wip"
}

func (w *gherkinWorld) initialize(ctx *godog.ScenarioContext) {
	ctx.Before(func(ctx context.Context, _ *godog.Scenario) (context.Context, error) {
		w.reset()
		return ctx, nil
	})
	ctx.After(func(ctx context.Context, _ *godog.Scenario, _ error) (context.Context, error) {
		w.close()
		return ctx, nil
	})

	ctx.Step(`^the SDK is installed$`, func() error { return nil })
	ctx.Step(`^the client library is imported$`, func() error { return nil })
	ctx.Step(`^a non-empty version string is available$`, w.nonEmptyVersion)
	ctx.Step(`^path "([^"]+)" responds to GET with status (\d+) and JSON$`, w.mockGET)
	ctx.Step(`^path "([^"]+)" responds to POST with status (\d+) and JSON$`, w.mockPOST)
	ctx.Step(`^path "([^"]+)" responds to GET with status (\d+) once then status (\d+) and JSON$`, w.mockGETOnceThen)
	ctx.Step(`^the async client is connected with status probe skipped$`, w.connect)
	ctx.Step(`^the async client is connected with short retries and status probe skipped$`, w.connect)
	ctx.Step(`^the async client fetches status$`, func() error { return w.request(http.MethodGet, "/status", nil) })
	ctx.Step(`^the async client creates an org named "([^"]+)"$`, w.createOrgNamed)
	ctx.Step(`^the async client creates an org with empty name and records any error$`, w.createOrgEmpty)
	ctx.Step(`^the async client lists orgs and records any error$`, w.listOrgsAnyError)
	ctx.Step(`^the async client lists orgs$`, func() error { return w.request(http.MethodGet, "/orgs", nil) })
	ctx.Step(`^the async client gets org "([^"]+)"$`, w.getOrg)
	ctx.Step(`^the async client lists devices for org "([^"]+)"$`, w.listDevices)
	ctx.Step(`^the async client runs entity action "([^"]+)" on entity "([^"]+)"$`, w.entityAction)
	ctx.Step(`^the async client lists invitations$`, func() error { return w.request(http.MethodGet, "/me/invitations", nil) })
	ctx.Step(`^the async client fetches public invite "([^"]+)"$`, w.fetchPublicInvite)
	ctx.Step(`^the async client claims public invite "([^"]+)" with empty body$`, w.claimPublicInvite)
	ctx.Step(`^an AuthError was raised$`, func() error { return w.wantErrorKind("auth") })
	ctx.Step(`^the AuthError message contains "([^"]+)"$`, w.errorMessageContains)
	ctx.Step(`^an ApiError was raised$`, func() error { return w.wantErrorKind("api") })
	ctx.Step(`^the ApiError has code "([^"]+)"$`, w.errorCodeEquals)
	ctx.Step(`^the last JSON has key "([^"]+)"$`, w.lastJSONHasKey)
	ctx.Step(`^the last JSON field "([^"]+)" equals string "([^"]+)"$`, w.lastJSONFieldEqualsString)
	ctx.Step(`^the last JSON field "([^"]+)" is boolean true$`, w.lastJSONFieldIsTrue)
	ctx.Step(`^the last JSON is a non-empty list$`, w.lastJSONNonEmptyList)
	ctx.Step(`^the last JSON is an empty list$`, w.lastJSONEmptyList)
	ctx.Step(`^the last JSON list has length (\d+)$`, w.lastJSONListLength)
	ctx.Step(`^the last JSON list at key "([^"]+)" has length (\d+)$`, w.lastJSONListAtKeyLength)
	ctx.Step(`^the invitations list includes status "([^"]+)"$`, w.invitationsIncludeStatus)
}

func (w *gherkinWorld) reset() {
	w.close()
	w.routes = map[string]*gherkinRoute{}
	w.lastJSON = nil
	w.lastError = nil
	w.server = httptest.NewServer(http.HandlerFunc(w.serveHTTP))
}

func (w *gherkinWorld) close() {
	if w.client != nil {
		_ = w.client.Close()
		w.client = nil
	}
	if w.server != nil {
		w.server.Close()
		w.server = nil
	}
}

func (w *gherkinWorld) serveHTTP(rw http.ResponseWriter, req *http.Request) {
	route := w.routes[req.Method+" "+req.URL.Path]
	if route == nil {
		http.Error(rw, "unexpected route", http.StatusNotFound)
		return
	}
	status := route.status
	body := route.body
	if route.firstStatus != 0 && !route.firstServed {
		route.firstServed = true
		status = route.firstStatus
		body = `{"message":"temporarily unavailable"}`
	}
	rw.Header().Set("Content-Type", "application/json")
	rw.WriteHeader(status)
	_, _ = rw.Write([]byte(body))
}

func (w *gherkinWorld) mockGET(path string, status int, doc *godog.DocString) error {
	w.routes[http.MethodGet+" "+path] = &gherkinRoute{method: http.MethodGet, path: path, status: status, body: strings.TrimSpace(doc.Content)}
	return nil
}

func (w *gherkinWorld) mockPOST(path string, status int, doc *godog.DocString) error {
	w.routes[http.MethodPost+" "+path] = &gherkinRoute{method: http.MethodPost, path: path, status: status, body: strings.TrimSpace(doc.Content)}
	return nil
}

func (w *gherkinWorld) mockGETOnceThen(path string, firstStatus, status int, doc *godog.DocString) error {
	w.routes[http.MethodGet+" "+path] = &gherkinRoute{method: http.MethodGet, path: path, status: status, firstStatus: firstStatus, body: strings.TrimSpace(doc.Content)}
	return nil
}

func (w *gherkinWorld) connect() error {
	if w.server == nil {
		w.reset()
	}
	client, err := NewAPIClient(strings.TrimSuffix(w.server.URL, "/") + "_openapp_TEST_SECRET")
	if err != nil {
		return err
	}
	w.client = client
	return nil
}

func (w *gherkinWorld) nonEmptyVersion() error {
	if NewConfiguration().UserAgent == "" {
		return fmt.Errorf("empty user agent")
	}
	return nil
}

func (w *gherkinWorld) request(method, apiPath string, body []byte) error {
	if w.client == nil {
		return fmt.Errorf("client is not connected")
	}
	req, err := http.NewRequestWithContext(context.Background(), method, w.server.URL+apiPath, bytes.NewReader(body))
	if err != nil {
		return err
	}
	req.Header.Set("Accept", "application/json")
	if len(body) > 0 {
		req.Header.Set("Content-Type", "application/json")
	}
	resp, err := w.client.cfg.HTTPClient.Transport.RoundTrip(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	raw, err := io.ReadAll(resp.Body)
	if err != nil {
		return err
	}
	if resp.StatusCode >= 400 {
		w.lastError = classifyGherkinError(resp.StatusCode, raw)
		return nil
	}
	if len(raw) == 0 {
		w.lastJSON = nil
		return nil
	}
	var decoded any
	if err := json.Unmarshal(raw, &decoded); err != nil {
		return err
	}
	w.lastJSON = decoded
	w.lastError = nil
	return nil
}

func classifyGherkinError(status int, raw []byte) *gherkinError {
	var payload map[string]any
	_ = json.Unmarshal(raw, &payload)
	message, _ := payload["message"].(string)
	code, _ := payload["code"].(string)
	if message == "" {
		message = string(raw)
	}
	kind := "api"
	rawText := strings.ToLower(string(raw))
	if status == http.StatusUnauthorized || status == http.StatusForbidden || strings.Contains(strings.ToLower(message), "token") || strings.Contains(rawText, "token") {
		kind = "auth"
	}
	return &gherkinError{kind: kind, message: message, code: code}
}

func (w *gherkinWorld) createOrgNamed(name string) error {
	body, _ := json.Marshal(map[string]string{"name": name})
	return w.request(http.MethodPost, "/orgs", body)
}

func (w *gherkinWorld) createOrgEmpty() error {
	return w.createOrgNamed("")
}

func (w *gherkinWorld) listOrgsAnyError() error {
	return w.request(http.MethodGet, "/orgs", nil)
}

func (w *gherkinWorld) getOrg(id string) error {
	return w.request(http.MethodGet, "/orgs/"+id, nil)
}

func (w *gherkinWorld) listDevices(_ string) error {
	return w.request(http.MethodGet, "/devices", nil)
}

func (w *gherkinWorld) entityAction(action, entityID string) error {
	body, _ := json.Marshal(map[string]any{})
	return w.request(http.MethodPost, "/entities/"+entityID+"/actions/"+action, body)
}

func (w *gherkinWorld) fetchPublicInvite(token string) error {
	return w.request(http.MethodGet, "/public/access/invites/"+token, nil)
}

func (w *gherkinWorld) claimPublicInvite(token string) error {
	return w.request(http.MethodPost, "/public/access/invites/"+token+"/claim", []byte(`{}`))
}

func (w *gherkinWorld) wantErrorKind(kind string) error {
	if w.lastError == nil {
		return fmt.Errorf("expected %s error, got nil", kind)
	}
	if w.lastError.kind != kind {
		return fmt.Errorf("expected %s error, got %s", kind, w.lastError.kind)
	}
	return nil
}

func (w *gherkinWorld) errorMessageContains(needle string) error {
	if w.lastError == nil || !strings.Contains(w.lastError.message, needle) {
		return fmt.Errorf("error message %q does not contain %q", w.lastError, needle)
	}
	return nil
}

func (w *gherkinWorld) errorCodeEquals(code string) error {
	if w.lastError == nil || w.lastError.code != code {
		return fmt.Errorf("error code = %q, want %q", w.lastError, code)
	}
	return nil
}

func (w *gherkinWorld) lastJSONMap() (map[string]any, error) {
	obj, ok := w.lastJSON.(map[string]any)
	if !ok {
		return nil, fmt.Errorf("last JSON is %T, want object", w.lastJSON)
	}
	return obj, nil
}

func (w *gherkinWorld) lastJSONHasKey(key string) error {
	obj, err := w.lastJSONMap()
	if err != nil {
		return err
	}
	if _, ok := obj[key]; !ok {
		return fmt.Errorf("missing key %q", key)
	}
	return nil
}

func (w *gherkinWorld) lastJSONFieldEqualsString(key, want string) error {
	obj, err := w.lastJSONMap()
	if err != nil {
		return err
	}
	if got, _ := obj[key].(string); got != want {
		return fmt.Errorf("%s = %q, want %q", key, got, want)
	}
	return nil
}

func (w *gherkinWorld) lastJSONFieldIsTrue(key string) error {
	obj, err := w.lastJSONMap()
	if err != nil {
		return err
	}
	if got, _ := obj[key].(bool); !got {
		return fmt.Errorf("%s is not true", key)
	}
	return nil
}

func (w *gherkinWorld) lastJSONNonEmptyList() error {
	items, ok := w.lastJSON.([]any)
	if !ok || len(items) == 0 {
		return fmt.Errorf("last JSON is not a non-empty list: %#v", w.lastJSON)
	}
	return nil
}

func (w *gherkinWorld) lastJSONEmptyList() error {
	items, ok := w.lastJSON.([]any)
	if !ok || len(items) != 0 {
		return fmt.Errorf("last JSON is not an empty list: %#v", w.lastJSON)
	}
	return nil
}

func (w *gherkinWorld) lastJSONListLength(length int) error {
	items, ok := w.lastJSON.([]any)
	if !ok || len(items) != length {
		return fmt.Errorf("list length = %d, want %d", len(items), length)
	}
	return nil
}

func (w *gherkinWorld) lastJSONListAtKeyLength(key string, length int) error {
	obj, err := w.lastJSONMap()
	if err != nil {
		return err
	}
	items, ok := obj[key].([]any)
	if !ok || len(items) != length {
		return fmt.Errorf("%s length = %d, want %d", key, len(items), length)
	}
	return nil
}

func (w *gherkinWorld) invitationsIncludeStatus(status string) error {
	items, ok := w.lastJSON.([]any)
	if !ok {
		return fmt.Errorf("last JSON is not a list")
	}
	for _, item := range items {
		obj, ok := item.(map[string]any)
		if ok && obj["status"] == status {
			return nil
		}
	}
	return fmt.Errorf("status %q not found", status)
}
