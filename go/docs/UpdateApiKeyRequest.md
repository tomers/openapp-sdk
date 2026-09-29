# UpdateApiKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExpiresAt** | Pointer to **NullableString** | [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) absolute expiration timestamp (UTC). Mutually exclusive with &#x60;expires_in&#x60;. Optional; if omitted, expiry is unchanged. | [optional]
**ExpiresIn** | Pointer to **NullableString** | Duration per [RFC 5545 §3.3.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.6) (ISO 8601 &#x60;P1D&#x60;, &#x60;PT1H&#x60;, …; no months/years) or compact tokens &#x60;s&#x60;/&#x60;m&#x60;/&#x60;h&#x60;/&#x60;d&#x60;/&#x60;w&#x60; (&#x60;M&#x60; is minutes). Max 3650d (10 years). &#x60;1y&#x60; is rejected. Mutually exclusive with &#x60;expires_at&#x60;. Optional; if omitted, expiry is unchanged. | [optional]
**Name** | **string** |  |

## Methods

### NewUpdateApiKeyRequest

`func NewUpdateApiKeyRequest(name string, ) *UpdateApiKeyRequest`

NewUpdateApiKeyRequest instantiates a new UpdateApiKeyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateApiKeyRequestWithDefaults

`func NewUpdateApiKeyRequestWithDefaults() *UpdateApiKeyRequest`

NewUpdateApiKeyRequestWithDefaults instantiates a new UpdateApiKeyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExpiresAt

`func (o *UpdateApiKeyRequest) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *UpdateApiKeyRequest) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *UpdateApiKeyRequest) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *UpdateApiKeyRequest) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *UpdateApiKeyRequest) SetExpiresAtNil(b bool)`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *UpdateApiKeyRequest) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil
### GetExpiresIn

`func (o *UpdateApiKeyRequest) GetExpiresIn() string`

GetExpiresIn returns the ExpiresIn field if non-nil, zero value otherwise.

### GetExpiresInOk

`func (o *UpdateApiKeyRequest) GetExpiresInOk() (*string, bool)`

GetExpiresInOk returns a tuple with the ExpiresIn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresIn

`func (o *UpdateApiKeyRequest) SetExpiresIn(v string)`

SetExpiresIn sets ExpiresIn field to given value.

### HasExpiresIn

`func (o *UpdateApiKeyRequest) HasExpiresIn() bool`

HasExpiresIn returns a boolean if a field has been set.

### SetExpiresInNil

`func (o *UpdateApiKeyRequest) SetExpiresInNil(b bool)`

 SetExpiresInNil sets the value for ExpiresIn to be an explicit nil

### UnsetExpiresIn
`func (o *UpdateApiKeyRequest) UnsetExpiresIn()`

UnsetExpiresIn ensures that no value is present for ExpiresIn, not even an explicit nil
### GetName

`func (o *UpdateApiKeyRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UpdateApiKeyRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UpdateApiKeyRequest) SetName(v string)`

SetName sets Name field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
