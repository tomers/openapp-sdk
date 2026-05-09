//go:build cgo

package openapi

// Close releases openapp-sdk-core resources when this client was created with
// [NewAPIClient]. It is a no-op for other configurations.
func (c *APIClient) Close() error {
	if c == nil || c.cfg == nil || c.cfg.HTTPClient == nil {
		return nil
	}
	t, ok := c.cfg.HTTPClient.Transport.(*coreRoundTripper)
	if !ok {
		return nil
	}
	return t.close()
}
