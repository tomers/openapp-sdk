# StoreProduct

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**DeletedAt** | Pointer to **NullableString** |  | [optional]
**ExternalUrl** | Pointer to **NullableString** |  | [optional]
**Id** | **NullableString** |  |
**ImageMediaAssetId** | Pointer to **NullableString** |  | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**OrgId** | **NullableString** |  |
**PurgeAt** | Pointer to **NullableString** |  | [optional]
**UpdatedAt** | Pointer to **NullableString** |  | [optional]

## Methods

### NewStoreProduct

`func NewStoreProduct(id NullableString, name LocalizedString, orgId NullableString, ) *StoreProduct`

NewStoreProduct instantiates a new StoreProduct object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewStoreProductWithDefaults

`func NewStoreProductWithDefaults() *StoreProduct`

NewStoreProductWithDefaults instantiates a new StoreProduct object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *StoreProduct) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *StoreProduct) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *StoreProduct) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *StoreProduct) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *StoreProduct) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *StoreProduct) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetDeletedAt

`func (o *StoreProduct) GetDeletedAt() string`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *StoreProduct) GetDeletedAtOk() (*string, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *StoreProduct) SetDeletedAt(v string)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *StoreProduct) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### SetDeletedAtNil

`func (o *StoreProduct) SetDeletedAtNil(b bool)`

 SetDeletedAtNil sets the value for DeletedAt to be an explicit nil

### UnsetDeletedAt
`func (o *StoreProduct) UnsetDeletedAt()`

UnsetDeletedAt ensures that no value is present for DeletedAt, not even an explicit nil
### GetExternalUrl

`func (o *StoreProduct) GetExternalUrl() string`

GetExternalUrl returns the ExternalUrl field if non-nil, zero value otherwise.

### GetExternalUrlOk

`func (o *StoreProduct) GetExternalUrlOk() (*string, bool)`

GetExternalUrlOk returns a tuple with the ExternalUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalUrl

`func (o *StoreProduct) SetExternalUrl(v string)`

SetExternalUrl sets ExternalUrl field to given value.

### HasExternalUrl

`func (o *StoreProduct) HasExternalUrl() bool`

HasExternalUrl returns a boolean if a field has been set.

### SetExternalUrlNil

`func (o *StoreProduct) SetExternalUrlNil(b bool)`

 SetExternalUrlNil sets the value for ExternalUrl to be an explicit nil

### UnsetExternalUrl
`func (o *StoreProduct) UnsetExternalUrl()`

UnsetExternalUrl ensures that no value is present for ExternalUrl, not even an explicit nil
### GetId

`func (o *StoreProduct) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *StoreProduct) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *StoreProduct) SetId(v string)`

SetId sets Id field to given value.


### SetIdNil

`func (o *StoreProduct) SetIdNil(b bool)`

 SetIdNil sets the value for Id to be an explicit nil

### UnsetId
`func (o *StoreProduct) UnsetId()`

UnsetId ensures that no value is present for Id, not even an explicit nil
### GetImageMediaAssetId

`func (o *StoreProduct) GetImageMediaAssetId() string`

GetImageMediaAssetId returns the ImageMediaAssetId field if non-nil, zero value otherwise.

### GetImageMediaAssetIdOk

`func (o *StoreProduct) GetImageMediaAssetIdOk() (*string, bool)`

GetImageMediaAssetIdOk returns a tuple with the ImageMediaAssetId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageMediaAssetId

`func (o *StoreProduct) SetImageMediaAssetId(v string)`

SetImageMediaAssetId sets ImageMediaAssetId field to given value.

### HasImageMediaAssetId

`func (o *StoreProduct) HasImageMediaAssetId() bool`

HasImageMediaAssetId returns a boolean if a field has been set.

### SetImageMediaAssetIdNil

`func (o *StoreProduct) SetImageMediaAssetIdNil(b bool)`

 SetImageMediaAssetIdNil sets the value for ImageMediaAssetId to be an explicit nil

### UnsetImageMediaAssetId
`func (o *StoreProduct) UnsetImageMediaAssetId()`

UnsetImageMediaAssetId ensures that no value is present for ImageMediaAssetId, not even an explicit nil
### GetName

`func (o *StoreProduct) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *StoreProduct) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *StoreProduct) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetOrgId

`func (o *StoreProduct) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *StoreProduct) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *StoreProduct) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### SetOrgIdNil

`func (o *StoreProduct) SetOrgIdNil(b bool)`

 SetOrgIdNil sets the value for OrgId to be an explicit nil

### UnsetOrgId
`func (o *StoreProduct) UnsetOrgId()`

UnsetOrgId ensures that no value is present for OrgId, not even an explicit nil
### GetPurgeAt

`func (o *StoreProduct) GetPurgeAt() string`

GetPurgeAt returns the PurgeAt field if non-nil, zero value otherwise.

### GetPurgeAtOk

`func (o *StoreProduct) GetPurgeAtOk() (*string, bool)`

GetPurgeAtOk returns a tuple with the PurgeAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurgeAt

`func (o *StoreProduct) SetPurgeAt(v string)`

SetPurgeAt sets PurgeAt field to given value.

### HasPurgeAt

`func (o *StoreProduct) HasPurgeAt() bool`

HasPurgeAt returns a boolean if a field has been set.

### SetPurgeAtNil

`func (o *StoreProduct) SetPurgeAtNil(b bool)`

 SetPurgeAtNil sets the value for PurgeAt to be an explicit nil

### UnsetPurgeAt
`func (o *StoreProduct) UnsetPurgeAt()`

UnsetPurgeAt ensures that no value is present for PurgeAt, not even an explicit nil
### GetUpdatedAt

`func (o *StoreProduct) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *StoreProduct) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *StoreProduct) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *StoreProduct) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *StoreProduct) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *StoreProduct) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
