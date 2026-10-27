# ApiErrorResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CorrelationId** | Pointer to **NullableString** | Server-generated correlation id that ties this error to the request trace and the matching audit event (see the audit log). Present when the request carried one. | [optional]
**CurrentPhone** | Pointer to **NullableString** | Present when &#x60;error_code&#x60; is &#x60;invite_phone_choice&#x60;: the number already on the signed-in account. | [optional]
**ErrorCode** | Pointer to **NullableString** | Optional stable code for clients to branch on (e.g. \&quot;door_unreachable\&quot;). | [optional]
**InvitedPhone** | Pointer to **NullableString** | Present when &#x60;error_code&#x60; is &#x60;invite_phone_choice&#x60;: the number named by the invitation. | [optional]
**Message** | **string** |  |
**PreviewToken** | Pointer to **NullableString** | Present when &#x60;error_code&#x60; is &#x60;confirmation_required&#x60; (retry with &#x60;X-OpenApp-Confirm&#x60;). | [optional]

## Methods

### NewApiErrorResponse

`func NewApiErrorResponse(message string, ) *ApiErrorResponse`

NewApiErrorResponse instantiates a new ApiErrorResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiErrorResponseWithDefaults

`func NewApiErrorResponseWithDefaults() *ApiErrorResponse`

NewApiErrorResponseWithDefaults instantiates a new ApiErrorResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCorrelationId

`func (o *ApiErrorResponse) GetCorrelationId() string`

GetCorrelationId returns the CorrelationId field if non-nil, zero value otherwise.

### GetCorrelationIdOk

`func (o *ApiErrorResponse) GetCorrelationIdOk() (*string, bool)`

GetCorrelationIdOk returns a tuple with the CorrelationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCorrelationId

`func (o *ApiErrorResponse) SetCorrelationId(v string)`

SetCorrelationId sets CorrelationId field to given value.

### HasCorrelationId

`func (o *ApiErrorResponse) HasCorrelationId() bool`

HasCorrelationId returns a boolean if a field has been set.

### SetCorrelationIdNil

`func (o *ApiErrorResponse) SetCorrelationIdNil(b bool)`

 SetCorrelationIdNil sets the value for CorrelationId to be an explicit nil

### UnsetCorrelationId
`func (o *ApiErrorResponse) UnsetCorrelationId()`

UnsetCorrelationId ensures that no value is present for CorrelationId, not even an explicit nil
### GetCurrentPhone

`func (o *ApiErrorResponse) GetCurrentPhone() string`

GetCurrentPhone returns the CurrentPhone field if non-nil, zero value otherwise.

### GetCurrentPhoneOk

`func (o *ApiErrorResponse) GetCurrentPhoneOk() (*string, bool)`

GetCurrentPhoneOk returns a tuple with the CurrentPhone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrentPhone

`func (o *ApiErrorResponse) SetCurrentPhone(v string)`

SetCurrentPhone sets CurrentPhone field to given value.

### HasCurrentPhone

`func (o *ApiErrorResponse) HasCurrentPhone() bool`

HasCurrentPhone returns a boolean if a field has been set.

### SetCurrentPhoneNil

`func (o *ApiErrorResponse) SetCurrentPhoneNil(b bool)`

 SetCurrentPhoneNil sets the value for CurrentPhone to be an explicit nil

### UnsetCurrentPhone
`func (o *ApiErrorResponse) UnsetCurrentPhone()`

UnsetCurrentPhone ensures that no value is present for CurrentPhone, not even an explicit nil
### GetErrorCode

`func (o *ApiErrorResponse) GetErrorCode() string`

GetErrorCode returns the ErrorCode field if non-nil, zero value otherwise.

### GetErrorCodeOk

`func (o *ApiErrorResponse) GetErrorCodeOk() (*string, bool)`

GetErrorCodeOk returns a tuple with the ErrorCode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorCode

`func (o *ApiErrorResponse) SetErrorCode(v string)`

SetErrorCode sets ErrorCode field to given value.

### HasErrorCode

`func (o *ApiErrorResponse) HasErrorCode() bool`

HasErrorCode returns a boolean if a field has been set.

### SetErrorCodeNil

`func (o *ApiErrorResponse) SetErrorCodeNil(b bool)`

 SetErrorCodeNil sets the value for ErrorCode to be an explicit nil

### UnsetErrorCode
`func (o *ApiErrorResponse) UnsetErrorCode()`

UnsetErrorCode ensures that no value is present for ErrorCode, not even an explicit nil
### GetInvitedPhone

`func (o *ApiErrorResponse) GetInvitedPhone() string`

GetInvitedPhone returns the InvitedPhone field if non-nil, zero value otherwise.

### GetInvitedPhoneOk

`func (o *ApiErrorResponse) GetInvitedPhoneOk() (*string, bool)`

GetInvitedPhoneOk returns a tuple with the InvitedPhone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvitedPhone

`func (o *ApiErrorResponse) SetInvitedPhone(v string)`

SetInvitedPhone sets InvitedPhone field to given value.

### HasInvitedPhone

`func (o *ApiErrorResponse) HasInvitedPhone() bool`

HasInvitedPhone returns a boolean if a field has been set.

### SetInvitedPhoneNil

`func (o *ApiErrorResponse) SetInvitedPhoneNil(b bool)`

 SetInvitedPhoneNil sets the value for InvitedPhone to be an explicit nil

### UnsetInvitedPhone
`func (o *ApiErrorResponse) UnsetInvitedPhone()`

UnsetInvitedPhone ensures that no value is present for InvitedPhone, not even an explicit nil
### GetMessage

`func (o *ApiErrorResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *ApiErrorResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *ApiErrorResponse) SetMessage(v string)`

SetMessage sets Message field to given value.


### GetPreviewToken

`func (o *ApiErrorResponse) GetPreviewToken() string`

GetPreviewToken returns the PreviewToken field if non-nil, zero value otherwise.

### GetPreviewTokenOk

`func (o *ApiErrorResponse) GetPreviewTokenOk() (*string, bool)`

GetPreviewTokenOk returns a tuple with the PreviewToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPreviewToken

`func (o *ApiErrorResponse) SetPreviewToken(v string)`

SetPreviewToken sets PreviewToken field to given value.

### HasPreviewToken

`func (o *ApiErrorResponse) HasPreviewToken() bool`

HasPreviewToken returns a boolean if a field has been set.

### SetPreviewTokenNil

`func (o *ApiErrorResponse) SetPreviewTokenNil(b bool)`

 SetPreviewTokenNil sets the value for PreviewToken to be an explicit nil

### UnsetPreviewToken
`func (o *ApiErrorResponse) UnsetPreviewToken()`

UnsetPreviewToken ensures that no value is present for PreviewToken, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
