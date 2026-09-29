# RegenerateAccessInviteTokenResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**InviteToken** | **string** |  |
**InviteUrl** | **string** | Guest-facing invite URL (shareable link for SMS/email). |
**LastSharedAt** | **string** |  |

## Methods

### NewRegenerateAccessInviteTokenResponse

`func NewRegenerateAccessInviteTokenResponse(inviteToken string, inviteUrl string, lastSharedAt string, ) *RegenerateAccessInviteTokenResponse`

NewRegenerateAccessInviteTokenResponse instantiates a new RegenerateAccessInviteTokenResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewRegenerateAccessInviteTokenResponseWithDefaults

`func NewRegenerateAccessInviteTokenResponseWithDefaults() *RegenerateAccessInviteTokenResponse`

NewRegenerateAccessInviteTokenResponseWithDefaults instantiates a new RegenerateAccessInviteTokenResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetInviteToken

`func (o *RegenerateAccessInviteTokenResponse) GetInviteToken() string`

GetInviteToken returns the InviteToken field if non-nil, zero value otherwise.

### GetInviteTokenOk

`func (o *RegenerateAccessInviteTokenResponse) GetInviteTokenOk() (*string, bool)`

GetInviteTokenOk returns a tuple with the InviteToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteToken

`func (o *RegenerateAccessInviteTokenResponse) SetInviteToken(v string)`

SetInviteToken sets InviteToken field to given value.


### GetInviteUrl

`func (o *RegenerateAccessInviteTokenResponse) GetInviteUrl() string`

GetInviteUrl returns the InviteUrl field if non-nil, zero value otherwise.

### GetInviteUrlOk

`func (o *RegenerateAccessInviteTokenResponse) GetInviteUrlOk() (*string, bool)`

GetInviteUrlOk returns a tuple with the InviteUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteUrl

`func (o *RegenerateAccessInviteTokenResponse) SetInviteUrl(v string)`

SetInviteUrl sets InviteUrl field to given value.


### GetLastSharedAt

`func (o *RegenerateAccessInviteTokenResponse) GetLastSharedAt() string`

GetLastSharedAt returns the LastSharedAt field if non-nil, zero value otherwise.

### GetLastSharedAtOk

`func (o *RegenerateAccessInviteTokenResponse) GetLastSharedAtOk() (*string, bool)`

GetLastSharedAtOk returns a tuple with the LastSharedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastSharedAt

`func (o *RegenerateAccessInviteTokenResponse) SetLastSharedAt(v string)`

SetLastSharedAt sets LastSharedAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
