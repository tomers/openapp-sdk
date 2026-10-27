# NotifyContactResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ContactId** | **string** |  |
**Error** | Pointer to **NullableString** |  | [optional]
**Status** | **string** | &#x60;sent&#x60; or &#x60;failed&#x60;. |

## Methods

### NewNotifyContactResult

`func NewNotifyContactResult(contactId string, status string, ) *NotifyContactResult`

NewNotifyContactResult instantiates a new NotifyContactResult object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewNotifyContactResultWithDefaults

`func NewNotifyContactResultWithDefaults() *NotifyContactResult`

NewNotifyContactResultWithDefaults instantiates a new NotifyContactResult object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetContactId

`func (o *NotifyContactResult) GetContactId() string`

GetContactId returns the ContactId field if non-nil, zero value otherwise.

### GetContactIdOk

`func (o *NotifyContactResult) GetContactIdOk() (*string, bool)`

GetContactIdOk returns a tuple with the ContactId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetContactId

`func (o *NotifyContactResult) SetContactId(v string)`

SetContactId sets ContactId field to given value.


### GetError

`func (o *NotifyContactResult) GetError() string`

GetError returns the Error field if non-nil, zero value otherwise.

### GetErrorOk

`func (o *NotifyContactResult) GetErrorOk() (*string, bool)`

GetErrorOk returns a tuple with the Error field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetError

`func (o *NotifyContactResult) SetError(v string)`

SetError sets Error field to given value.

### HasError

`func (o *NotifyContactResult) HasError() bool`

HasError returns a boolean if a field has been set.

### SetErrorNil

`func (o *NotifyContactResult) SetErrorNil(b bool)`

 SetErrorNil sets the value for Error to be an explicit nil

### UnsetError
`func (o *NotifyContactResult) UnsetError()`

UnsetError ensures that no value is present for Error, not even an explicit nil
### GetStatus

`func (o *NotifyContactResult) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *NotifyContactResult) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *NotifyContactResult) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
