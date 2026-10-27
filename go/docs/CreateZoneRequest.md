# CreateZoneRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**ParentZoneId** | Pointer to **NullableString** |  | [optional]
**SiteIds** | Pointer to **[]string** | Access sites this zone belongs to. When omitted: inherit the parent&#39;s memberships if a parent is set, otherwise unbound (empty). | [optional]

## Methods

### NewCreateZoneRequest

`func NewCreateZoneRequest(name LocalizedString, ) *CreateZoneRequest`

NewCreateZoneRequest instantiates a new CreateZoneRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateZoneRequestWithDefaults

`func NewCreateZoneRequestWithDefaults() *CreateZoneRequest`

NewCreateZoneRequestWithDefaults instantiates a new CreateZoneRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetName

`func (o *CreateZoneRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateZoneRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateZoneRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetParentZoneId

`func (o *CreateZoneRequest) GetParentZoneId() string`

GetParentZoneId returns the ParentZoneId field if non-nil, zero value otherwise.

### GetParentZoneIdOk

`func (o *CreateZoneRequest) GetParentZoneIdOk() (*string, bool)`

GetParentZoneIdOk returns a tuple with the ParentZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetParentZoneId

`func (o *CreateZoneRequest) SetParentZoneId(v string)`

SetParentZoneId sets ParentZoneId field to given value.

### HasParentZoneId

`func (o *CreateZoneRequest) HasParentZoneId() bool`

HasParentZoneId returns a boolean if a field has been set.

### SetParentZoneIdNil

`func (o *CreateZoneRequest) SetParentZoneIdNil(b bool)`

 SetParentZoneIdNil sets the value for ParentZoneId to be an explicit nil

### UnsetParentZoneId
`func (o *CreateZoneRequest) UnsetParentZoneId()`

UnsetParentZoneId ensures that no value is present for ParentZoneId, not even an explicit nil
### GetSiteIds

`func (o *CreateZoneRequest) GetSiteIds() []string`

GetSiteIds returns the SiteIds field if non-nil, zero value otherwise.

### GetSiteIdsOk

`func (o *CreateZoneRequest) GetSiteIdsOk() (*[]string, bool)`

GetSiteIdsOk returns a tuple with the SiteIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSiteIds

`func (o *CreateZoneRequest) SetSiteIds(v []string)`

SetSiteIds sets SiteIds field to given value.

### HasSiteIds

`func (o *CreateZoneRequest) HasSiteIds() bool`

HasSiteIds returns a boolean if a field has been set.

### SetSiteIdsNil

`func (o *CreateZoneRequest) SetSiteIdsNil(b bool)`

 SetSiteIdsNil sets the value for SiteIds to be an explicit nil

### UnsetSiteIds
`func (o *CreateZoneRequest) UnsetSiteIds()`

UnsetSiteIds ensures that no value is present for SiteIds, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
