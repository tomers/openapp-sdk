# CreateInvitationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Invitation** | [**UserInvitationDto**](UserInvitationDto.md) |  |
**RedeemUrl** | **string** | Shareable redeem URL — surfaced once so the inviter can hand it off out-of-band (the PIN is never returned here). |

## Methods

### NewCreateInvitationResponse

`func NewCreateInvitationResponse(invitation UserInvitationDto, redeemUrl string, ) *CreateInvitationResponse`

NewCreateInvitationResponse instantiates a new CreateInvitationResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateInvitationResponseWithDefaults

`func NewCreateInvitationResponseWithDefaults() *CreateInvitationResponse`

NewCreateInvitationResponseWithDefaults instantiates a new CreateInvitationResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetInvitation

`func (o *CreateInvitationResponse) GetInvitation() UserInvitationDto`

GetInvitation returns the Invitation field if non-nil, zero value otherwise.

### GetInvitationOk

`func (o *CreateInvitationResponse) GetInvitationOk() (*UserInvitationDto, bool)`

GetInvitationOk returns a tuple with the Invitation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvitation

`func (o *CreateInvitationResponse) SetInvitation(v UserInvitationDto)`

SetInvitation sets Invitation field to given value.


### GetRedeemUrl

`func (o *CreateInvitationResponse) GetRedeemUrl() string`

GetRedeemUrl returns the RedeemUrl field if non-nil, zero value otherwise.

### GetRedeemUrlOk

`func (o *CreateInvitationResponse) GetRedeemUrlOk() (*string, bool)`

GetRedeemUrlOk returns a tuple with the RedeemUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRedeemUrl

`func (o *CreateInvitationResponse) SetRedeemUrl(v string)`

SetRedeemUrl sets RedeemUrl field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
