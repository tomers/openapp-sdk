# InvitationPreviewSiteAccess

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Listing** | Pointer to [**NullableInvitationPreviewListing**](InvitationPreviewListing.md) |  | [optional]
**Role** | **string** | &#x60;admin&#x60;, &#x60;resident&#x60;, or &#x60;technician&#x60;. |
**SiteName** | Pointer to **interface{}** |  | [optional]

## Methods

### NewInvitationPreviewSiteAccess

`func NewInvitationPreviewSiteAccess(role string, ) *InvitationPreviewSiteAccess`

NewInvitationPreviewSiteAccess instantiates a new InvitationPreviewSiteAccess object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInvitationPreviewSiteAccessWithDefaults

`func NewInvitationPreviewSiteAccessWithDefaults() *InvitationPreviewSiteAccess`

NewInvitationPreviewSiteAccessWithDefaults instantiates a new InvitationPreviewSiteAccess object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetListing

`func (o *InvitationPreviewSiteAccess) GetListing() InvitationPreviewListing`

GetListing returns the Listing field if non-nil, zero value otherwise.

### GetListingOk

`func (o *InvitationPreviewSiteAccess) GetListingOk() (*InvitationPreviewListing, bool)`

GetListingOk returns a tuple with the Listing field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListing

`func (o *InvitationPreviewSiteAccess) SetListing(v InvitationPreviewListing)`

SetListing sets Listing field to given value.

### HasListing

`func (o *InvitationPreviewSiteAccess) HasListing() bool`

HasListing returns a boolean if a field has been set.

### SetListingNil

`func (o *InvitationPreviewSiteAccess) SetListingNil(b bool)`

 SetListingNil sets the value for Listing to be an explicit nil

### UnsetListing
`func (o *InvitationPreviewSiteAccess) UnsetListing()`

UnsetListing ensures that no value is present for Listing, not even an explicit nil
### GetRole

`func (o *InvitationPreviewSiteAccess) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *InvitationPreviewSiteAccess) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *InvitationPreviewSiteAccess) SetRole(v string)`

SetRole sets Role field to given value.


### GetSiteName

`func (o *InvitationPreviewSiteAccess) GetSiteName() interface{}`

GetSiteName returns the SiteName field if non-nil, zero value otherwise.

### GetSiteNameOk

`func (o *InvitationPreviewSiteAccess) GetSiteNameOk() (*interface{}, bool)`

GetSiteNameOk returns a tuple with the SiteName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSiteName

`func (o *InvitationPreviewSiteAccess) SetSiteName(v interface{})`

SetSiteName sets SiteName field to given value.

### HasSiteName

`func (o *InvitationPreviewSiteAccess) HasSiteName() bool`

HasSiteName returns a boolean if a field has been set.

### SetSiteNameNil

`func (o *InvitationPreviewSiteAccess) SetSiteNameNil(b bool)`

 SetSiteNameNil sets the value for SiteName to be an explicit nil

### UnsetSiteName
`func (o *InvitationPreviewSiteAccess) UnsetSiteName()`

UnsetSiteName ensures that no value is present for SiteName, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
