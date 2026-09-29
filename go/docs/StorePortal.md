# StorePortal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BrandingOverrides** | Pointer to **map[string]interface{}** |  | [optional]
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**DeletedAt** | Pointer to **NullableString** |  | [optional]
**Id** | **string** |  |
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**OrgId** | **string** |  |
**PublicId** | **string** |  |
**StoreId** | **string** |  |
**UpdatedAt** | Pointer to **NullableString** |  | [optional]

## Methods

### NewStorePortal

`func NewStorePortal(id string, name LocalizedString, orgId string, publicId string, storeId string, ) *StorePortal`

NewStorePortal instantiates a new StorePortal object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewStorePortalWithDefaults

`func NewStorePortalWithDefaults() *StorePortal`

NewStorePortalWithDefaults instantiates a new StorePortal object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBrandingOverrides

`func (o *StorePortal) GetBrandingOverrides() map[string]interface{}`

GetBrandingOverrides returns the BrandingOverrides field if non-nil, zero value otherwise.

### GetBrandingOverridesOk

`func (o *StorePortal) GetBrandingOverridesOk() (*map[string]interface{}, bool)`

GetBrandingOverridesOk returns a tuple with the BrandingOverrides field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBrandingOverrides

`func (o *StorePortal) SetBrandingOverrides(v map[string]interface{})`

SetBrandingOverrides sets BrandingOverrides field to given value.

### HasBrandingOverrides

`func (o *StorePortal) HasBrandingOverrides() bool`

HasBrandingOverrides returns a boolean if a field has been set.

### SetBrandingOverridesNil

`func (o *StorePortal) SetBrandingOverridesNil(b bool)`

 SetBrandingOverridesNil sets the value for BrandingOverrides to be an explicit nil

### UnsetBrandingOverrides
`func (o *StorePortal) UnsetBrandingOverrides()`

UnsetBrandingOverrides ensures that no value is present for BrandingOverrides, not even an explicit nil
### GetCreatedAt

`func (o *StorePortal) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *StorePortal) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *StorePortal) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *StorePortal) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *StorePortal) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *StorePortal) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetDeletedAt

`func (o *StorePortal) GetDeletedAt() string`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *StorePortal) GetDeletedAtOk() (*string, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *StorePortal) SetDeletedAt(v string)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *StorePortal) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### SetDeletedAtNil

`func (o *StorePortal) SetDeletedAtNil(b bool)`

 SetDeletedAtNil sets the value for DeletedAt to be an explicit nil

### UnsetDeletedAt
`func (o *StorePortal) UnsetDeletedAt()`

UnsetDeletedAt ensures that no value is present for DeletedAt, not even an explicit nil
### GetId

`func (o *StorePortal) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *StorePortal) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *StorePortal) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *StorePortal) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *StorePortal) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *StorePortal) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetOrgId

`func (o *StorePortal) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *StorePortal) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *StorePortal) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetPublicId

`func (o *StorePortal) GetPublicId() string`

GetPublicId returns the PublicId field if non-nil, zero value otherwise.

### GetPublicIdOk

`func (o *StorePortal) GetPublicIdOk() (*string, bool)`

GetPublicIdOk returns a tuple with the PublicId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicId

`func (o *StorePortal) SetPublicId(v string)`

SetPublicId sets PublicId field to given value.


### GetStoreId

`func (o *StorePortal) GetStoreId() string`

GetStoreId returns the StoreId field if non-nil, zero value otherwise.

### GetStoreIdOk

`func (o *StorePortal) GetStoreIdOk() (*string, bool)`

GetStoreIdOk returns a tuple with the StoreId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStoreId

`func (o *StorePortal) SetStoreId(v string)`

SetStoreId sets StoreId field to given value.


### GetUpdatedAt

`func (o *StorePortal) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *StorePortal) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *StorePortal) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *StorePortal) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *StorePortal) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *StorePortal) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
