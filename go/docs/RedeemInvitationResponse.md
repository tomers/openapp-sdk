# RedeemInvitationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DefaultRoles** | **[]string** |  |
**Merge** | Pointer to [**NullableMergeOfferDto**](MergeOfferDto.md) |  | [optional]
**OrgId** | **string** |  |
**UserId** | **string** |  |

## Methods

### NewRedeemInvitationResponse

`func NewRedeemInvitationResponse(defaultRoles []string, orgId string, userId string, ) *RedeemInvitationResponse`

NewRedeemInvitationResponse instantiates a new RedeemInvitationResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewRedeemInvitationResponseWithDefaults

`func NewRedeemInvitationResponseWithDefaults() *RedeemInvitationResponse`

NewRedeemInvitationResponseWithDefaults instantiates a new RedeemInvitationResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDefaultRoles

`func (o *RedeemInvitationResponse) GetDefaultRoles() []string`

GetDefaultRoles returns the DefaultRoles field if non-nil, zero value otherwise.

### GetDefaultRolesOk

`func (o *RedeemInvitationResponse) GetDefaultRolesOk() (*[]string, bool)`

GetDefaultRolesOk returns a tuple with the DefaultRoles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultRoles

`func (o *RedeemInvitationResponse) SetDefaultRoles(v []string)`

SetDefaultRoles sets DefaultRoles field to given value.


### GetMerge

`func (o *RedeemInvitationResponse) GetMerge() MergeOfferDto`

GetMerge returns the Merge field if non-nil, zero value otherwise.

### GetMergeOk

`func (o *RedeemInvitationResponse) GetMergeOk() (*MergeOfferDto, bool)`

GetMergeOk returns a tuple with the Merge field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMerge

`func (o *RedeemInvitationResponse) SetMerge(v MergeOfferDto)`

SetMerge sets Merge field to given value.

### HasMerge

`func (o *RedeemInvitationResponse) HasMerge() bool`

HasMerge returns a boolean if a field has been set.

### SetMergeNil

`func (o *RedeemInvitationResponse) SetMergeNil(b bool)`

 SetMergeNil sets the value for Merge to be an explicit nil

### UnsetMerge
`func (o *RedeemInvitationResponse) UnsetMerge()`

UnsetMerge ensures that no value is present for Merge, not even an explicit nil
### GetOrgId

`func (o *RedeemInvitationResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *RedeemInvitationResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *RedeemInvitationResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetUserId

`func (o *RedeemInvitationResponse) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *RedeemInvitationResponse) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *RedeemInvitationResponse) SetUserId(v string)`

SetUserId sets UserId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
