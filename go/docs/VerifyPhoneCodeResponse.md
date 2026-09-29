# VerifyPhoneCodeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Phone** | **string** |  |
**PhoneVerifiedAt** | **string** |  |

## Methods

### NewVerifyPhoneCodeResponse

`func NewVerifyPhoneCodeResponse(phone string, phoneVerifiedAt string, ) *VerifyPhoneCodeResponse`

NewVerifyPhoneCodeResponse instantiates a new VerifyPhoneCodeResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewVerifyPhoneCodeResponseWithDefaults

`func NewVerifyPhoneCodeResponseWithDefaults() *VerifyPhoneCodeResponse`

NewVerifyPhoneCodeResponseWithDefaults instantiates a new VerifyPhoneCodeResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPhone

`func (o *VerifyPhoneCodeResponse) GetPhone() string`

GetPhone returns the Phone field if non-nil, zero value otherwise.

### GetPhoneOk

`func (o *VerifyPhoneCodeResponse) GetPhoneOk() (*string, bool)`

GetPhoneOk returns a tuple with the Phone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhone

`func (o *VerifyPhoneCodeResponse) SetPhone(v string)`

SetPhone sets Phone field to given value.


### GetPhoneVerifiedAt

`func (o *VerifyPhoneCodeResponse) GetPhoneVerifiedAt() string`

GetPhoneVerifiedAt returns the PhoneVerifiedAt field if non-nil, zero value otherwise.

### GetPhoneVerifiedAtOk

`func (o *VerifyPhoneCodeResponse) GetPhoneVerifiedAtOk() (*string, bool)`

GetPhoneVerifiedAtOk returns a tuple with the PhoneVerifiedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhoneVerifiedAt

`func (o *VerifyPhoneCodeResponse) SetPhoneVerifiedAt(v string)`

SetPhoneVerifiedAt sets PhoneVerifiedAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
