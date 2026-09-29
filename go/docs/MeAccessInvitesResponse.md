# MeAccessInvitesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Invites** | [**[]AccessInviteListItem**](AccessInviteListItem.md) |  |

## Methods

### NewMeAccessInvitesResponse

`func NewMeAccessInvitesResponse(invites []AccessInviteListItem, ) *MeAccessInvitesResponse`

NewMeAccessInvitesResponse instantiates a new MeAccessInvitesResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMeAccessInvitesResponseWithDefaults

`func NewMeAccessInvitesResponseWithDefaults() *MeAccessInvitesResponse`

NewMeAccessInvitesResponseWithDefaults instantiates a new MeAccessInvitesResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetInvites

`func (o *MeAccessInvitesResponse) GetInvites() []AccessInviteListItem`

GetInvites returns the Invites field if non-nil, zero value otherwise.

### GetInvitesOk

`func (o *MeAccessInvitesResponse) GetInvitesOk() (*[]AccessInviteListItem, bool)`

GetInvitesOk returns a tuple with the Invites field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvites

`func (o *MeAccessInvitesResponse) SetInvites(v []AccessInviteListItem)`

SetInvites sets Invites field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
