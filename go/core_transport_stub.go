//go:build !cgo

package openapi

import "errors"

// NewAPIClient is unavailable without CGO. The module requires cgo and
// libopenapp_sdk_core_c_bridge for all HTTP; see packages/sdk/docker and
// packages/sdk/go/README.md.
func NewAPIClient(apiKey string) (*APIClient, error) {
	_ = apiKey
	return nil, errors.New("openapp-sdk: NewAPIClient requires CGO and openapp-sdk-core-c-bridge; this build has cgo disabled")
}
