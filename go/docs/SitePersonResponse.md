# SitePersonResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Groups** | [**[]SitePersonGroupResponse**](SitePersonGroupResponse.md) |  |
**ListingNames** | [**[]SitePersonListingResponse**](SitePersonListingResponse.md) |  |
**Listings** | **[]string** |  |
**Role** | **string** | Site role (&#x60;admin&#x60;, &#x60;resident&#x60;, &#x60;technician&#x60;) or, for a via-group row, the group role. |
**Source** | **string** | &#x60;org_member&#x60; or &#x60;via_group&#x60;. Via-group rows have no site role to edit. |
**UserEmail** | Pointer to **NullableString** |  | [optional]
**UserId** | **string** |  |
**UserName** | Pointer to **interface{}** |  | [optional]

## Methods

### NewSitePersonResponse

`func NewSitePersonResponse(groups []SitePersonGroupResponse, listingNames []SitePersonListingResponse, listings []string, role string, source string, userId string, ) *SitePersonResponse`

NewSitePersonResponse instantiates a new SitePersonResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSitePersonResponseWithDefaults

`func NewSitePersonResponseWithDefaults() *SitePersonResponse`

NewSitePersonResponseWithDefaults instantiates a new SitePersonResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetGroups

`func (o *SitePersonResponse) GetGroups() []SitePersonGroupResponse`

GetGroups returns the Groups field if non-nil, zero value otherwise.

### GetGroupsOk

`func (o *SitePersonResponse) GetGroupsOk() (*[]SitePersonGroupResponse, bool)`

GetGroupsOk returns a tuple with the Groups field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGroups

`func (o *SitePersonResponse) SetGroups(v []SitePersonGroupResponse)`

SetGroups sets Groups field to given value.


### GetListingNames

`func (o *SitePersonResponse) GetListingNames() []SitePersonListingResponse`

GetListingNames returns the ListingNames field if non-nil, zero value otherwise.

### GetListingNamesOk

`func (o *SitePersonResponse) GetListingNamesOk() (*[]SitePersonListingResponse, bool)`

GetListingNamesOk returns a tuple with the ListingNames field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListingNames

`func (o *SitePersonResponse) SetListingNames(v []SitePersonListingResponse)`

SetListingNames sets ListingNames field to given value.


### GetListings

`func (o *SitePersonResponse) GetListings() []string`

GetListings returns the Listings field if non-nil, zero value otherwise.

### GetListingsOk

`func (o *SitePersonResponse) GetListingsOk() (*[]string, bool)`

GetListingsOk returns a tuple with the Listings field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListings

`func (o *SitePersonResponse) SetListings(v []string)`

SetListings sets Listings field to given value.


### GetRole

`func (o *SitePersonResponse) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *SitePersonResponse) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *SitePersonResponse) SetRole(v string)`

SetRole sets Role field to given value.


### GetSource

`func (o *SitePersonResponse) GetSource() string`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *SitePersonResponse) GetSourceOk() (*string, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *SitePersonResponse) SetSource(v string)`

SetSource sets Source field to given value.


### GetUserEmail

`func (o *SitePersonResponse) GetUserEmail() string`

GetUserEmail returns the UserEmail field if non-nil, zero value otherwise.

### GetUserEmailOk

`func (o *SitePersonResponse) GetUserEmailOk() (*string, bool)`

GetUserEmailOk returns a tuple with the UserEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserEmail

`func (o *SitePersonResponse) SetUserEmail(v string)`

SetUserEmail sets UserEmail field to given value.

### HasUserEmail

`func (o *SitePersonResponse) HasUserEmail() bool`

HasUserEmail returns a boolean if a field has been set.

### SetUserEmailNil

`func (o *SitePersonResponse) SetUserEmailNil(b bool)`

 SetUserEmailNil sets the value for UserEmail to be an explicit nil

### UnsetUserEmail
`func (o *SitePersonResponse) UnsetUserEmail()`

UnsetUserEmail ensures that no value is present for UserEmail, not even an explicit nil
### GetUserId

`func (o *SitePersonResponse) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *SitePersonResponse) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *SitePersonResponse) SetUserId(v string)`

SetUserId sets UserId field to given value.


### GetUserName

`func (o *SitePersonResponse) GetUserName() interface{}`

GetUserName returns the UserName field if non-nil, zero value otherwise.

### GetUserNameOk

`func (o *SitePersonResponse) GetUserNameOk() (*interface{}, bool)`

GetUserNameOk returns a tuple with the UserName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserName

`func (o *SitePersonResponse) SetUserName(v interface{})`

SetUserName sets UserName field to given value.

### HasUserName

`func (o *SitePersonResponse) HasUserName() bool`

HasUserName returns a boolean if a field has been set.

### SetUserNameNil

`func (o *SitePersonResponse) SetUserNameNil(b bool)`

 SetUserNameNil sets the value for UserName to be an explicit nil

### UnsetUserName
`func (o *SitePersonResponse) UnsetUserName()`

UnsetUserName ensures that no value is present for UserName, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
