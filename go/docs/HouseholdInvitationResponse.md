# HouseholdInvitationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Channel** | **string** |  |
**CreatedAt** | **string** |  |
**ExpiresAt** | **string** |  |
**GroupId** | **string** |  |
**Id** | **string** |  |
**InvitedByUserId** | **string** |  |
**LastSentAt** | Pointer to **NullableString** |  | [optional]
**RecipientEmail** | Pointer to **NullableString** |  | [optional]
**RecipientPhone** | Pointer to **NullableString** |  | [optional]
**RespondedAt** | Pointer to **NullableString** |  | [optional]
**SentCount** | **int32** |  |
**Status** | **string** |  |

## Methods

### NewHouseholdInvitationResponse

`func NewHouseholdInvitationResponse(channel string, createdAt string, expiresAt string, groupId string, id string, invitedByUserId string, sentCount int32, status string, ) *HouseholdInvitationResponse`

NewHouseholdInvitationResponse instantiates a new HouseholdInvitationResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewHouseholdInvitationResponseWithDefaults

`func NewHouseholdInvitationResponseWithDefaults() *HouseholdInvitationResponse`

NewHouseholdInvitationResponseWithDefaults instantiates a new HouseholdInvitationResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetChannel

`func (o *HouseholdInvitationResponse) GetChannel() string`

GetChannel returns the Channel field if non-nil, zero value otherwise.

### GetChannelOk

`func (o *HouseholdInvitationResponse) GetChannelOk() (*string, bool)`

GetChannelOk returns a tuple with the Channel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChannel

`func (o *HouseholdInvitationResponse) SetChannel(v string)`

SetChannel sets Channel field to given value.


### GetCreatedAt

`func (o *HouseholdInvitationResponse) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *HouseholdInvitationResponse) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *HouseholdInvitationResponse) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.


### GetExpiresAt

`func (o *HouseholdInvitationResponse) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *HouseholdInvitationResponse) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *HouseholdInvitationResponse) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.


### GetGroupId

`func (o *HouseholdInvitationResponse) GetGroupId() string`

GetGroupId returns the GroupId field if non-nil, zero value otherwise.

### GetGroupIdOk

`func (o *HouseholdInvitationResponse) GetGroupIdOk() (*string, bool)`

GetGroupIdOk returns a tuple with the GroupId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGroupId

`func (o *HouseholdInvitationResponse) SetGroupId(v string)`

SetGroupId sets GroupId field to given value.


### GetId

`func (o *HouseholdInvitationResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *HouseholdInvitationResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *HouseholdInvitationResponse) SetId(v string)`

SetId sets Id field to given value.


### GetInvitedByUserId

`func (o *HouseholdInvitationResponse) GetInvitedByUserId() string`

GetInvitedByUserId returns the InvitedByUserId field if non-nil, zero value otherwise.

### GetInvitedByUserIdOk

`func (o *HouseholdInvitationResponse) GetInvitedByUserIdOk() (*string, bool)`

GetInvitedByUserIdOk returns a tuple with the InvitedByUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvitedByUserId

`func (o *HouseholdInvitationResponse) SetInvitedByUserId(v string)`

SetInvitedByUserId sets InvitedByUserId field to given value.


### GetLastSentAt

`func (o *HouseholdInvitationResponse) GetLastSentAt() string`

GetLastSentAt returns the LastSentAt field if non-nil, zero value otherwise.

### GetLastSentAtOk

`func (o *HouseholdInvitationResponse) GetLastSentAtOk() (*string, bool)`

GetLastSentAtOk returns a tuple with the LastSentAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastSentAt

`func (o *HouseholdInvitationResponse) SetLastSentAt(v string)`

SetLastSentAt sets LastSentAt field to given value.

### HasLastSentAt

`func (o *HouseholdInvitationResponse) HasLastSentAt() bool`

HasLastSentAt returns a boolean if a field has been set.

### SetLastSentAtNil

`func (o *HouseholdInvitationResponse) SetLastSentAtNil(b bool)`

 SetLastSentAtNil sets the value for LastSentAt to be an explicit nil

### UnsetLastSentAt
`func (o *HouseholdInvitationResponse) UnsetLastSentAt()`

UnsetLastSentAt ensures that no value is present for LastSentAt, not even an explicit nil
### GetRecipientEmail

`func (o *HouseholdInvitationResponse) GetRecipientEmail() string`

GetRecipientEmail returns the RecipientEmail field if non-nil, zero value otherwise.

### GetRecipientEmailOk

`func (o *HouseholdInvitationResponse) GetRecipientEmailOk() (*string, bool)`

GetRecipientEmailOk returns a tuple with the RecipientEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecipientEmail

`func (o *HouseholdInvitationResponse) SetRecipientEmail(v string)`

SetRecipientEmail sets RecipientEmail field to given value.

### HasRecipientEmail

`func (o *HouseholdInvitationResponse) HasRecipientEmail() bool`

HasRecipientEmail returns a boolean if a field has been set.

### SetRecipientEmailNil

`func (o *HouseholdInvitationResponse) SetRecipientEmailNil(b bool)`

 SetRecipientEmailNil sets the value for RecipientEmail to be an explicit nil

### UnsetRecipientEmail
`func (o *HouseholdInvitationResponse) UnsetRecipientEmail()`

UnsetRecipientEmail ensures that no value is present for RecipientEmail, not even an explicit nil
### GetRecipientPhone

`func (o *HouseholdInvitationResponse) GetRecipientPhone() string`

GetRecipientPhone returns the RecipientPhone field if non-nil, zero value otherwise.

### GetRecipientPhoneOk

`func (o *HouseholdInvitationResponse) GetRecipientPhoneOk() (*string, bool)`

GetRecipientPhoneOk returns a tuple with the RecipientPhone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecipientPhone

`func (o *HouseholdInvitationResponse) SetRecipientPhone(v string)`

SetRecipientPhone sets RecipientPhone field to given value.

### HasRecipientPhone

`func (o *HouseholdInvitationResponse) HasRecipientPhone() bool`

HasRecipientPhone returns a boolean if a field has been set.

### SetRecipientPhoneNil

`func (o *HouseholdInvitationResponse) SetRecipientPhoneNil(b bool)`

 SetRecipientPhoneNil sets the value for RecipientPhone to be an explicit nil

### UnsetRecipientPhone
`func (o *HouseholdInvitationResponse) UnsetRecipientPhone()`

UnsetRecipientPhone ensures that no value is present for RecipientPhone, not even an explicit nil
### GetRespondedAt

`func (o *HouseholdInvitationResponse) GetRespondedAt() string`

GetRespondedAt returns the RespondedAt field if non-nil, zero value otherwise.

### GetRespondedAtOk

`func (o *HouseholdInvitationResponse) GetRespondedAtOk() (*string, bool)`

GetRespondedAtOk returns a tuple with the RespondedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRespondedAt

`func (o *HouseholdInvitationResponse) SetRespondedAt(v string)`

SetRespondedAt sets RespondedAt field to given value.

### HasRespondedAt

`func (o *HouseholdInvitationResponse) HasRespondedAt() bool`

HasRespondedAt returns a boolean if a field has been set.

### SetRespondedAtNil

`func (o *HouseholdInvitationResponse) SetRespondedAtNil(b bool)`

 SetRespondedAtNil sets the value for RespondedAt to be an explicit nil

### UnsetRespondedAt
`func (o *HouseholdInvitationResponse) UnsetRespondedAt()`

UnsetRespondedAt ensures that no value is present for RespondedAt, not even an explicit nil
### GetSentCount

`func (o *HouseholdInvitationResponse) GetSentCount() int32`

GetSentCount returns the SentCount field if non-nil, zero value otherwise.

### GetSentCountOk

`func (o *HouseholdInvitationResponse) GetSentCountOk() (*int32, bool)`

GetSentCountOk returns a tuple with the SentCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSentCount

`func (o *HouseholdInvitationResponse) SetSentCount(v int32)`

SetSentCount sets SentCount field to given value.


### GetStatus

`func (o *HouseholdInvitationResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *HouseholdInvitationResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *HouseholdInvitationResponse) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
