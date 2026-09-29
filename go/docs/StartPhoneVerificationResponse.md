# StartPhoneVerificationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExpiresAt** | **string** |  |
**Phone** | **string** | Normalized E.164 phone the code was sent to (for display only). |

## Methods

### NewStartPhoneVerificationResponse

`func NewStartPhoneVerificationResponse(expiresAt string, phone string, ) *StartPhoneVerificationResponse`

NewStartPhoneVerificationResponse instantiates a new StartPhoneVerificationResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewStartPhoneVerificationResponseWithDefaults

`func NewStartPhoneVerificationResponseWithDefaults() *StartPhoneVerificationResponse`

NewStartPhoneVerificationResponseWithDefaults instantiates a new StartPhoneVerificationResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExpiresAt

`func (o *StartPhoneVerificationResponse) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *StartPhoneVerificationResponse) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *StartPhoneVerificationResponse) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.


### GetPhone

`func (o *StartPhoneVerificationResponse) GetPhone() string`

GetPhone returns the Phone field if non-nil, zero value otherwise.

### GetPhoneOk

`func (o *StartPhoneVerificationResponse) GetPhoneOk() (*string, bool)`

GetPhoneOk returns a tuple with the Phone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhone

`func (o *StartPhoneVerificationResponse) SetPhone(v string)`

SetPhone sets Phone field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
