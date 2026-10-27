# PaginatedResponseZoneResponseItemsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **time.Time** |  | [optional]
**DeletedAt** | Pointer to **time.Time** |  | [optional]
**HardDeleteAt** | Pointer to **time.Time** |  | [optional]
**PurgeAt** | Pointer to **time.Time** |  | [optional]
**UpdatedAt** | Pointer to **time.Time** |  | [optional]
**Id** | **string** | Unique identifier (ULID). |
**Name** | [**LocalizedString**](LocalizedString.md) | Zone name. |
**OrgId** | **string** | Organization that owns this zone. |
**ParentZoneId** | Pointer to **string** | Parent zone (same org) for hierarchy. | [optional]
**ProviderLinks** | [**[]ZoneProviderLinkResponse**](ZoneProviderLinkResponse.md) |  |
**SiteIds** | **[]string** | Access sites (&#x60;virtual_access&#x60; integrations) this zone is a member of, sorted. Empty means unbound (Resources-only until bound; never offered on Access pickers). |

## Methods

### NewPaginatedResponseZoneResponseItemsInner

`func NewPaginatedResponseZoneResponseItemsInner(id string, name LocalizedString, orgId string, providerLinks []ZoneProviderLinkResponse, siteIds []string, ) *PaginatedResponseZoneResponseItemsInner`

NewPaginatedResponseZoneResponseItemsInner instantiates a new PaginatedResponseZoneResponseItemsInner object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseZoneResponseItemsInnerWithDefaults

`func NewPaginatedResponseZoneResponseItemsInnerWithDefaults() *PaginatedResponseZoneResponseItemsInner`

NewPaginatedResponseZoneResponseItemsInnerWithDefaults instantiates a new PaginatedResponseZoneResponseItemsInner object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *PaginatedResponseZoneResponseItemsInner) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *PaginatedResponseZoneResponseItemsInner) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *PaginatedResponseZoneResponseItemsInner) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *PaginatedResponseZoneResponseItemsInner) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetDeletedAt

`func (o *PaginatedResponseZoneResponseItemsInner) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *PaginatedResponseZoneResponseItemsInner) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *PaginatedResponseZoneResponseItemsInner) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *PaginatedResponseZoneResponseItemsInner) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetHardDeleteAt

`func (o *PaginatedResponseZoneResponseItemsInner) GetHardDeleteAt() time.Time`

GetHardDeleteAt returns the HardDeleteAt field if non-nil, zero value otherwise.

### GetHardDeleteAtOk

`func (o *PaginatedResponseZoneResponseItemsInner) GetHardDeleteAtOk() (*time.Time, bool)`

GetHardDeleteAtOk returns a tuple with the HardDeleteAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHardDeleteAt

`func (o *PaginatedResponseZoneResponseItemsInner) SetHardDeleteAt(v time.Time)`

SetHardDeleteAt sets HardDeleteAt field to given value.

### HasHardDeleteAt

`func (o *PaginatedResponseZoneResponseItemsInner) HasHardDeleteAt() bool`

HasHardDeleteAt returns a boolean if a field has been set.

### GetPurgeAt

`func (o *PaginatedResponseZoneResponseItemsInner) GetPurgeAt() time.Time`

GetPurgeAt returns the PurgeAt field if non-nil, zero value otherwise.

### GetPurgeAtOk

`func (o *PaginatedResponseZoneResponseItemsInner) GetPurgeAtOk() (*time.Time, bool)`

GetPurgeAtOk returns a tuple with the PurgeAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurgeAt

`func (o *PaginatedResponseZoneResponseItemsInner) SetPurgeAt(v time.Time)`

SetPurgeAt sets PurgeAt field to given value.

### HasPurgeAt

`func (o *PaginatedResponseZoneResponseItemsInner) HasPurgeAt() bool`

HasPurgeAt returns a boolean if a field has been set.

### GetUpdatedAt

`func (o *PaginatedResponseZoneResponseItemsInner) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *PaginatedResponseZoneResponseItemsInner) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *PaginatedResponseZoneResponseItemsInner) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *PaginatedResponseZoneResponseItemsInner) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### GetId

`func (o *PaginatedResponseZoneResponseItemsInner) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PaginatedResponseZoneResponseItemsInner) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PaginatedResponseZoneResponseItemsInner) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *PaginatedResponseZoneResponseItemsInner) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PaginatedResponseZoneResponseItemsInner) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PaginatedResponseZoneResponseItemsInner) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetOrgId

`func (o *PaginatedResponseZoneResponseItemsInner) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *PaginatedResponseZoneResponseItemsInner) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *PaginatedResponseZoneResponseItemsInner) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetParentZoneId

`func (o *PaginatedResponseZoneResponseItemsInner) GetParentZoneId() string`

GetParentZoneId returns the ParentZoneId field if non-nil, zero value otherwise.

### GetParentZoneIdOk

`func (o *PaginatedResponseZoneResponseItemsInner) GetParentZoneIdOk() (*string, bool)`

GetParentZoneIdOk returns a tuple with the ParentZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetParentZoneId

`func (o *PaginatedResponseZoneResponseItemsInner) SetParentZoneId(v string)`

SetParentZoneId sets ParentZoneId field to given value.

### HasParentZoneId

`func (o *PaginatedResponseZoneResponseItemsInner) HasParentZoneId() bool`

HasParentZoneId returns a boolean if a field has been set.

### GetProviderLinks

`func (o *PaginatedResponseZoneResponseItemsInner) GetProviderLinks() []ZoneProviderLinkResponse`

GetProviderLinks returns the ProviderLinks field if non-nil, zero value otherwise.

### GetProviderLinksOk

`func (o *PaginatedResponseZoneResponseItemsInner) GetProviderLinksOk() (*[]ZoneProviderLinkResponse, bool)`

GetProviderLinksOk returns a tuple with the ProviderLinks field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderLinks

`func (o *PaginatedResponseZoneResponseItemsInner) SetProviderLinks(v []ZoneProviderLinkResponse)`

SetProviderLinks sets ProviderLinks field to given value.


### GetSiteIds

`func (o *PaginatedResponseZoneResponseItemsInner) GetSiteIds() []string`

GetSiteIds returns the SiteIds field if non-nil, zero value otherwise.

### GetSiteIdsOk

`func (o *PaginatedResponseZoneResponseItemsInner) GetSiteIdsOk() (*[]string, bool)`

GetSiteIdsOk returns a tuple with the SiteIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSiteIds

`func (o *PaginatedResponseZoneResponseItemsInner) SetSiteIds(v []string)`

SetSiteIds sets SiteIds field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
