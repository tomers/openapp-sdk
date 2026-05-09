//go:build cgo

package bridge

/*
#cgo LDFLAGS: -L${SRCDIR}/../../core/target/release/deps -L${SRCDIR}/../../core/target/release -L${SRCDIR}/../../node/releases/x86_64-unknown-linux-gnu -L/usr/local/lib -lopenapp_sdk_core_c_bridge

typedef struct BridgeRuntime BridgeRuntime;
typedef struct BridgeClient BridgeClient;

BridgeRuntime* openapp_sdk_runtime_new(void);
void openapp_sdk_runtime_free(BridgeRuntime* ptr);
void openapp_sdk_telemetry_init(void);

BridgeClient* openapp_sdk_client_new(BridgeRuntime* runtime, const char* api_key, char** out_err);
void openapp_sdk_client_free(BridgeClient* ptr);

int openapp_sdk_client_request(
	BridgeClient* client,
	const char* method_utf8,
	const char* path_utf8,
	const char* body_json_utf8,
	char** out_body,
	int* out_http_status);
void openapp_sdk_string_free(char* ptr);

#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

static inline void* sdk_go_bridge_handle_as_ptr(uintptr_t h) {
	return (void*)h;
}

typedef void (*OpenAppSdkRequestCompleteFn)(int bridge_status, int http_status, char* body, void* user_data);
int openapp_sdk_client_request_async(
	BridgeClient* client,
	const char* method_utf8,
	const char* path_utf8,
	const char* body_json_utf8,
	OpenAppSdkRequestCompleteFn complete,
	void* user_data);

typedef int (*OpenAppSdkStreamChunkFn)(unsigned char* data, size_t len, void* user_data);
typedef void (*OpenAppSdkStreamCompleteFn)(int bridge_status, int http_status, char* error_message, void* user_data);
typedef void (*OpenAppSdkStreamResponseHeadersFn)(int http_status, void* user_data);
int openapp_sdk_client_request_stream_async_with_headers(
	BridgeClient* client,
	const char* method_utf8,
	const char* path_utf8,
	const char* body_json_utf8,
	OpenAppSdkStreamChunkFn chunk,
	OpenAppSdkStreamCompleteFn complete,
	OpenAppSdkStreamResponseHeadersFn response_headers,
	void* user_data);

int openapp_sdk_client_request_raw(
	BridgeClient* client,
	const char* method_utf8,
	const char* path_utf8,
	const char* content_type_utf8,
	const uint8_t* body,
	size_t body_len,
	char** out_body,
	int* out_http_status);

extern void bridgeJSONAsyncComplete(int bridge_status, int http_status, char* body, void* user_data);
extern int bridgeStreamChunk(unsigned char* data, size_t len, void* user_data);
extern void bridgeStreamComplete(int bridge_status, int http_status, char* error_message, void* user_data);
extern void bridgeStreamResponseHeaders(int http_status, void* user_data);

static inline int go_bridge_request_async(
	BridgeClient* client,
	const char* method_utf8,
	const char* path_utf8,
	const char* body_json_utf8,
	void* user_data)
{
	return openapp_sdk_client_request_async(
		client, method_utf8, path_utf8, body_json_utf8,
		(OpenAppSdkRequestCompleteFn)bridgeJSONAsyncComplete,
		user_data);
}

static inline int go_bridge_request_stream_async_with_headers_maybe(
	BridgeClient* client,
	const char* method_utf8,
	const char* path_utf8,
	const char* body_json_utf8,
	void* user_data,
	int use_response_headers_fn)
{
	return openapp_sdk_client_request_stream_async_with_headers(
		client, method_utf8, path_utf8, body_json_utf8,
		(OpenAppSdkStreamChunkFn)bridgeStreamChunk,
		(OpenAppSdkStreamCompleteFn)bridgeStreamComplete,
		use_response_headers_fn ? (OpenAppSdkStreamResponseHeadersFn)bridgeStreamResponseHeaders : (OpenAppSdkStreamResponseHeadersFn)0,
		user_data);
}
*/
import "C"
import (
	"errors"
	"fmt"
	"runtime/cgo"
	"sync"
	"unsafe"
)

// Status is a [BridgeStatus] value from the C bridge (see core-c-bridge).
type Status int32

// Known bridge status codes; others may appear as new errors are mapped.
const (
	StatusOK              Status = 0
	StatusInvalidArgument Status = 1
	StatusConfigError     Status = 2
	StatusTransportError  Status = 3
	StatusAuthError       Status = 4
	StatusApiError        Status = 5
	StatusInternalError   Status = 99
)

func (s Status) String() string {
	switch s {
	case StatusOK:
		return "ok"
	case StatusInvalidArgument:
		return "invalid_argument"
	case StatusConfigError:
		return "config_error"
	case StatusTransportError:
		return "transport_error"
	case StatusAuthError:
		return "auth_error"
	case StatusApiError:
		return "api_error"
	case StatusInternalError:
		return "internal_error"
	default:
		return fmt.Sprintf("bridge_status(%d)", s)
	}
}

// Runtime manages the Tokio runtime owned by the C bridge.
type Runtime struct {
	ptr *C.BridgeRuntime
}

var telemetryInitOnce sync.Once

// InitTelemetry installs the core tracing subscriber used by openapp-sdk-core.
// It is safe to call multiple times; initialization runs once process-wide.
func InitTelemetry() {
	telemetryInitOnce.Do(func() {
		C.openapp_sdk_telemetry_init()
	})
}

// NewRuntime allocates a bridge runtime. Call [Runtime.Close] when finished.
func NewRuntime() (*Runtime, error) {
	InitTelemetry()
	p := C.openapp_sdk_runtime_new()
	if p == nil {
		return nil, errors.New("openapp_sdk_runtime_new returned null")
	}
	return &Runtime{ptr: p}, nil
}

// Close releases the runtime handle.
func (r *Runtime) Close() {
	if r == nil || r.ptr == nil {
		return
	}
	C.openapp_sdk_runtime_free(r.ptr)
	r.ptr = nil
}

// Client is an OpenApp HTTP client backed by openapp-sdk-core.
type Client struct {
	rt     *Runtime
	client *C.BridgeClient
}

// NewClient builds a client from an API key string in OpenApp form
// ({base_url}_openapp_{secret}). The runtime must outlive the client.
func NewClient(rt *Runtime, apiKey string) (*Client, error) {
	if rt == nil || rt.ptr == nil {
		return nil, errors.New("nil runtime")
	}
	key := C.CString(apiKey)
	defer C.free(unsafe.Pointer(key))
	var errOut *C.char
	c := C.openapp_sdk_client_new(rt.ptr, key, &errOut)
	if c == nil {
		msg := "openapp_sdk_client_new failed"
		if errOut != nil {
			msg = C.GoString(errOut)
			C.openapp_sdk_string_free(errOut)
		}
		return nil, errors.New(msg)
	}
	return &Client{rt: rt, client: c}, nil
}

// Close releases the client handle.
func (c *Client) Close() {
	if c == nil || c.client == nil {
		return
	}
	C.openapp_sdk_client_free(c.client)
	c.client = nil
}

// JSONRequest performs a JSON request using core transport. On success, status is
// [StatusOK], httpStatus is the HTTP status line code, and body is JSON text or an
// error message from the bridge on failure.
func (c *Client) JSONRequest(method, path string, bodyJSON *string) (st Status, httpStatus int, body string, err error) {
	if c == nil || c.client == nil {
		return StatusInvalidArgument, 0, "", errors.New("nil client")
	}
	m := C.CString(method)
	defer C.free(unsafe.Pointer(m))
	p := C.CString(path)
	defer C.free(unsafe.Pointer(p))
	var bodyIn *C.char
	if bodyJSON != nil {
		bodyIn = C.CString(*bodyJSON)
		defer C.free(unsafe.Pointer(bodyIn))
	}
	var outBody *C.char
	var httpSt C.int
	code := C.openapp_sdk_client_request(c.client, m, p, bodyIn, &outBody, &httpSt)
	st = Status(code)
	httpStatus = int(httpSt)
	if outBody == nil {
		return st, httpStatus, "", fmt.Errorf("bridge: null body pointer (status=%s)", st)
	}
	defer C.openapp_sdk_string_free(outBody)
	body = C.GoString(outBody)
	if st != StatusOK {
		err = fmt.Errorf("%s: %s", st, body)
	}
	return st, httpStatus, body, err
}

// JSONRequestRaw sends a pre-encoded body (e.g. multipart) with an explicit Content-Type
// and returns a JSON response body from core (same decode path as [Client.JSONRequest]).
func (c *Client) JSONRequestRaw(method, path, contentType string, body []byte) (st Status, httpStatus int, bodyOut string, err error) {
	if c == nil || c.client == nil {
		return StatusInvalidArgument, 0, "", errors.New("nil client")
	}
	if len(body) == 0 {
		return StatusInvalidArgument, 0, "", errors.New("empty body")
	}
	if contentType == "" {
		return StatusInvalidArgument, 0, "", errors.New("empty content type")
	}
	m := C.CString(method)
	defer C.free(unsafe.Pointer(m))
	p := C.CString(path)
	defer C.free(unsafe.Pointer(p))
	ct := C.CString(contentType)
	defer C.free(unsafe.Pointer(ct))
	var outBody *C.char
	var httpSt C.int
	code := C.openapp_sdk_client_request_raw(
		c.client,
		m,
		p,
		ct,
		(*C.uint8_t)(unsafe.Pointer(&body[0])),
		C.size_t(len(body)),
		&outBody,
		&httpSt,
	)
	st = Status(code)
	httpStatus = int(httpSt)
	if outBody == nil {
		return st, httpStatus, "", fmt.Errorf("bridge: null body pointer (status=%s)", st)
	}
	defer C.openapp_sdk_string_free(outBody)
	bodyOut = C.GoString(outBody)
	if st != StatusOK {
		err = fmt.Errorf("%s: %s", st, bodyOut)
	}
	return st, httpStatus, bodyOut, err
}

type jsonAsyncResult struct {
	st         Status
	httpStatus int
	body       string
	err        error
}

type jsonAsyncCtx struct {
	ch chan jsonAsyncResult
}

//export bridgeJSONAsyncComplete
func bridgeJSONAsyncComplete(bridgeStatus C.int, httpStatus C.int, body *C.char, userData unsafe.Pointer) {
	h := cgo.Handle(uintptr(userData))
	ctx := h.Value().(*jsonAsyncCtx)
	var bodyStr string
	if body != nil {
		bodyStr = C.GoString(body)
		C.openapp_sdk_string_free(body)
	}
	st := Status(bridgeStatus)
	var err error
	if st != StatusOK {
		err = fmt.Errorf("%s: %s", st, bodyStr)
	}
	ctx.ch <- jsonAsyncResult{st: st, httpStatus: int(httpStatus), body: bodyStr, err: err}
}

// JSONRequestAsync schedules the same JSON request path as [Client.JSONRequest] on the
// bridge runtime and blocks until the completion callback runs. [Client] and [Runtime]
// must remain valid until this method returns.
func (c *Client) JSONRequestAsync(method, path string, bodyJSON *string) (st Status, httpStatus int, body string, err error) {
	if c == nil || c.client == nil {
		return StatusInvalidArgument, 0, "", errors.New("nil client")
	}
	m := C.CString(method)
	defer C.free(unsafe.Pointer(m))
	p := C.CString(path)
	defer C.free(unsafe.Pointer(p))
	var bodyIn *C.char
	if bodyJSON != nil {
		bodyIn = C.CString(*bodyJSON)
		defer C.free(unsafe.Pointer(bodyIn))
	}

	ctx := &jsonAsyncCtx{ch: make(chan jsonAsyncResult, 1)}
	handle := cgo.NewHandle(ctx)
	code := C.go_bridge_request_async(
		c.client, m, p, bodyIn,
		C.sdk_go_bridge_handle_as_ptr(C.uintptr_t(uintptr(handle))),
	)
	queue := Status(code)
	if queue != StatusOK {
		handle.Delete()
		return queue, 0, "", fmt.Errorf("bridge: async request not queued: %s", queue)
	}

	res := <-ctx.ch
	handle.Delete()
	return res.st, res.httpStatus, res.body, res.err
}

type streamCompleteMsg struct {
	st         Status
	httpStatus int
	err        error
}

type streamUserData struct {
	onResponseHTTP func(int)
	fn             func([]byte) (stop bool)
	done           chan streamCompleteMsg
}

//export bridgeStreamResponseHeaders
func bridgeStreamResponseHeaders(httpStatus C.int, userData unsafe.Pointer) {
	if userData == nil {
		return
	}
	h := cgo.Handle(uintptr(userData))
	ud := h.Value().(*streamUserData)
	if ud.onResponseHTTP != nil {
		ud.onResponseHTTP(int(httpStatus))
	}
}

//export bridgeStreamChunk
func bridgeStreamChunk(data *C.uchar, length C.size_t, userData unsafe.Pointer) C.int {
	if userData == nil {
		return 1
	}
	h := cgo.Handle(uintptr(userData))
	ud := h.Value().(*streamUserData)
	n := int(length)
	if n == 0 || data == nil {
		return 0
	}
	sl := unsafe.Slice((*byte)(unsafe.Pointer(data)), n)
	if ud.fn == nil {
		return 0
	}
	stop := ud.fn(sl)
	if stop {
		return 1
	}
	return 0
}

//export bridgeStreamComplete
func bridgeStreamComplete(bridgeStatus C.int, httpStatus C.int, errorMessage *C.char, userData unsafe.Pointer) {
	if userData == nil {
		return
	}
	h := cgo.Handle(uintptr(userData))
	ud := h.Value().(*streamUserData)
	var err error
	if errorMessage != nil {
		msg := C.GoString(errorMessage)
		C.openapp_sdk_string_free(errorMessage)
		err = errors.New(msg)
	}
	st := Status(bridgeStatus)
	if err == nil && st != StatusOK {
		err = fmt.Errorf("%s", st)
	}
	ud.done <- streamCompleteMsg{st: st, httpStatus: int(httpStatus), err: err}
}

// StreamRequest performs a streaming HTTP request (same routing as [Client.JSONRequestAsync]);
// response bytes are delivered to chunk. Return false from chunk to continue, true to stop
// early. On success, err is nil and httpStatus is the HTTP status code.
func (c *Client) StreamRequest(method, path string, bodyJSON *string, chunk func([]byte) (stop bool)) (st Status, httpStatus int, err error) {
	return c.streamRequest(method, path, bodyJSON, nil, chunk)
}

// StreamRequestWithHTTPCallback is like [Client.StreamRequest] but invokes onResponseHTTP
// once with the HTTP status when the response line is known, before any chunk.
func (c *Client) StreamRequestWithHTTPCallback(method, path string, bodyJSON *string, onResponseHTTP func(int), chunk func([]byte) (stop bool)) (st Status, httpStatus int, err error) {
	return c.streamRequest(method, path, bodyJSON, onResponseHTTP, chunk)
}

func (c *Client) streamRequest(method, path string, bodyJSON *string, onResponseHTTP func(int), chunk func([]byte) (stop bool)) (st Status, httpStatus int, err error) {
	if c == nil || c.client == nil {
		return StatusInvalidArgument, 0, errors.New("nil client")
	}
	if chunk == nil {
		return StatusInvalidArgument, 0, errors.New("nil chunk callback")
	}
	m := C.CString(method)
	defer C.free(unsafe.Pointer(m))
	p := C.CString(path)
	defer C.free(unsafe.Pointer(p))
	var bodyIn *C.char
	if bodyJSON != nil {
		bodyIn = C.CString(*bodyJSON)
		defer C.free(unsafe.Pointer(bodyIn))
	}

	useHdr := C.int(0)
	if onResponseHTTP != nil {
		useHdr = C.int(1)
	}
	ud := &streamUserData{onResponseHTTP: onResponseHTTP, fn: chunk, done: make(chan streamCompleteMsg, 1)}
	handle := cgo.NewHandle(ud)
	code := C.go_bridge_request_stream_async_with_headers_maybe(
		c.client, m, p, bodyIn,
		C.sdk_go_bridge_handle_as_ptr(C.uintptr_t(uintptr(handle))),
		useHdr,
	)
	queue := Status(code)
	if queue != StatusOK {
		handle.Delete()
		return queue, 0, fmt.Errorf("bridge: stream request not queued: %s", queue)
	}

	msg := <-ud.done
	handle.Delete()
	return msg.st, msg.httpStatus, msg.err
}
