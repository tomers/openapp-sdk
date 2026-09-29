# WebhookTestResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Delivered** | **bool** |  |
**Error** | Pointer to **NullableString** |  | [optional]
**StatusCode** | Pointer to **NullableInt32** |  | [optional]

## Methods

### NewWebhookTestResponse

`func NewWebhookTestResponse(delivered bool, ) *WebhookTestResponse`

NewWebhookTestResponse instantiates a new WebhookTestResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewWebhookTestResponseWithDefaults

`func NewWebhookTestResponseWithDefaults() *WebhookTestResponse`

NewWebhookTestResponseWithDefaults instantiates a new WebhookTestResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDelivered

`func (o *WebhookTestResponse) GetDelivered() bool`

GetDelivered returns the Delivered field if non-nil, zero value otherwise.

### GetDeliveredOk

`func (o *WebhookTestResponse) GetDeliveredOk() (*bool, bool)`

GetDeliveredOk returns a tuple with the Delivered field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDelivered

`func (o *WebhookTestResponse) SetDelivered(v bool)`

SetDelivered sets Delivered field to given value.


### GetError

`func (o *WebhookTestResponse) GetError() string`

GetError returns the Error field if non-nil, zero value otherwise.

### GetErrorOk

`func (o *WebhookTestResponse) GetErrorOk() (*string, bool)`

GetErrorOk returns a tuple with the Error field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetError

`func (o *WebhookTestResponse) SetError(v string)`

SetError sets Error field to given value.

### HasError

`func (o *WebhookTestResponse) HasError() bool`

HasError returns a boolean if a field has been set.

### SetErrorNil

`func (o *WebhookTestResponse) SetErrorNil(b bool)`

 SetErrorNil sets the value for Error to be an explicit nil

### UnsetError
`func (o *WebhookTestResponse) UnsetError()`

UnsetError ensures that no value is present for Error, not even an explicit nil
### GetStatusCode

`func (o *WebhookTestResponse) GetStatusCode() int32`

GetStatusCode returns the StatusCode field if non-nil, zero value otherwise.

### GetStatusCodeOk

`func (o *WebhookTestResponse) GetStatusCodeOk() (*int32, bool)`

GetStatusCodeOk returns a tuple with the StatusCode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatusCode

`func (o *WebhookTestResponse) SetStatusCode(v int32)`

SetStatusCode sets StatusCode field to given value.

### HasStatusCode

`func (o *WebhookTestResponse) HasStatusCode() bool`

HasStatusCode returns a boolean if a field has been set.

### SetStatusCodeNil

`func (o *WebhookTestResponse) SetStatusCodeNil(b bool)`

 SetStatusCodeNil sets the value for StatusCode to be an explicit nil

### UnsetStatusCode
`func (o *WebhookTestResponse) UnsetStatusCode()`

UnsetStatusCode ensures that no value is present for StatusCode, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
