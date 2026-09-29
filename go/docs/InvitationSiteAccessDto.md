# InvitationSiteAccessDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IntegrationId** | **NullableString** |  |
**Listing** | Pointer to [**NullableInvitationListingGrant**](InvitationListingGrant.md) |  | [optional]
**Role** | **NullableString** |  |

## Methods

### NewInvitationSiteAccessDto

`func NewInvitationSiteAccessDto(integrationId NullableString, role NullableString, ) *InvitationSiteAccessDto`

NewInvitationSiteAccessDto instantiates a new InvitationSiteAccessDto object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInvitationSiteAccessDtoWithDefaults

`func NewInvitationSiteAccessDtoWithDefaults() *InvitationSiteAccessDto`

NewInvitationSiteAccessDtoWithDefaults instantiates a new InvitationSiteAccessDto object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIntegrationId

`func (o *InvitationSiteAccessDto) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *InvitationSiteAccessDto) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *InvitationSiteAccessDto) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### SetIntegrationIdNil

`func (o *InvitationSiteAccessDto) SetIntegrationIdNil(b bool)`

 SetIntegrationIdNil sets the value for IntegrationId to be an explicit nil

### UnsetIntegrationId
`func (o *InvitationSiteAccessDto) UnsetIntegrationId()`

UnsetIntegrationId ensures that no value is present for IntegrationId, not even an explicit nil
### GetListing

`func (o *InvitationSiteAccessDto) GetListing() InvitationListingGrant`

GetListing returns the Listing field if non-nil, zero value otherwise.

### GetListingOk

`func (o *InvitationSiteAccessDto) GetListingOk() (*InvitationListingGrant, bool)`

GetListingOk returns a tuple with the Listing field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListing

`func (o *InvitationSiteAccessDto) SetListing(v InvitationListingGrant)`

SetListing sets Listing field to given value.

### HasListing

`func (o *InvitationSiteAccessDto) HasListing() bool`

HasListing returns a boolean if a field has been set.

### SetListingNil

`func (o *InvitationSiteAccessDto) SetListingNil(b bool)`

 SetListingNil sets the value for Listing to be an explicit nil

### UnsetListing
`func (o *InvitationSiteAccessDto) UnsetListing()`

UnsetListing ensures that no value is present for Listing, not even an explicit nil
### GetRole

`func (o *InvitationSiteAccessDto) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *InvitationSiteAccessDto) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *InvitationSiteAccessDto) SetRole(v string)`

SetRole sets Role field to given value.


### SetRoleNil

`func (o *InvitationSiteAccessDto) SetRoleNil(b bool)`

 SetRoleNil sets the value for Role to be an explicit nil

### UnsetRole
`func (o *InvitationSiteAccessDto) UnsetRole()`

UnsetRole ensures that no value is present for Role, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
