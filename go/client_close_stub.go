//go:build !cgo

package openapi

// Close is a no-op when the module is built without cgo.
func (c *APIClient) Close() error {
	return nil
}
