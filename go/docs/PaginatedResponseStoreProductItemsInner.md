# PaginatedResponseStoreProductItemsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**DeletedAt** | Pointer to **NullableString** |  | [optional]
**ExternalUrl** | Pointer to **NullableString** |  | [optional]
**Id** | **string** |  |
**ImageMediaAssetId** | Pointer to **NullableString** |  | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**OrgId** | **string** |  |
**PurgeAt** | Pointer to **NullableString** |  | [optional]
**UpdatedAt** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPaginatedResponseStoreProductItemsInner

`func NewPaginatedResponseStoreProductItemsInner(id string, name LocalizedString, orgId string, ) *PaginatedResponseStoreProductItemsInner`

NewPaginatedResponseStoreProductItemsInner instantiates a new PaginatedResponseStoreProductItemsInner object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseStoreProductItemsInnerWithDefaults

`func NewPaginatedResponseStoreProductItemsInnerWithDefaults() *PaginatedResponseStoreProductItemsInner`

NewPaginatedResponseStoreProductItemsInnerWithDefaults instantiates a new PaginatedResponseStoreProductItemsInner object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *PaginatedResponseStoreProductItemsInner) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *PaginatedResponseStoreProductItemsInner) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *PaginatedResponseStoreProductItemsInner) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *PaginatedResponseStoreProductItemsInner) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *PaginatedResponseStoreProductItemsInner) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *PaginatedResponseStoreProductItemsInner) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetDeletedAt

`func (o *PaginatedResponseStoreProductItemsInner) GetDeletedAt() string`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *PaginatedResponseStoreProductItemsInner) GetDeletedAtOk() (*string, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *PaginatedResponseStoreProductItemsInner) SetDeletedAt(v string)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *PaginatedResponseStoreProductItemsInner) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### SetDeletedAtNil

`func (o *PaginatedResponseStoreProductItemsInner) SetDeletedAtNil(b bool)`

 SetDeletedAtNil sets the value for DeletedAt to be an explicit nil

### UnsetDeletedAt
`func (o *PaginatedResponseStoreProductItemsInner) UnsetDeletedAt()`

UnsetDeletedAt ensures that no value is present for DeletedAt, not even an explicit nil
### GetExternalUrl

`func (o *PaginatedResponseStoreProductItemsInner) GetExternalUrl() string`

GetExternalUrl returns the ExternalUrl field if non-nil, zero value otherwise.

### GetExternalUrlOk

`func (o *PaginatedResponseStoreProductItemsInner) GetExternalUrlOk() (*string, bool)`

GetExternalUrlOk returns a tuple with the ExternalUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalUrl

`func (o *PaginatedResponseStoreProductItemsInner) SetExternalUrl(v string)`

SetExternalUrl sets ExternalUrl field to given value.

### HasExternalUrl

`func (o *PaginatedResponseStoreProductItemsInner) HasExternalUrl() bool`

HasExternalUrl returns a boolean if a field has been set.

### SetExternalUrlNil

`func (o *PaginatedResponseStoreProductItemsInner) SetExternalUrlNil(b bool)`

 SetExternalUrlNil sets the value for ExternalUrl to be an explicit nil

### UnsetExternalUrl
`func (o *PaginatedResponseStoreProductItemsInner) UnsetExternalUrl()`

UnsetExternalUrl ensures that no value is present for ExternalUrl, not even an explicit nil
### GetId

`func (o *PaginatedResponseStoreProductItemsInner) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PaginatedResponseStoreProductItemsInner) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PaginatedResponseStoreProductItemsInner) SetId(v string)`

SetId sets Id field to given value.


### GetImageMediaAssetId

`func (o *PaginatedResponseStoreProductItemsInner) GetImageMediaAssetId() string`

GetImageMediaAssetId returns the ImageMediaAssetId field if non-nil, zero value otherwise.

### GetImageMediaAssetIdOk

`func (o *PaginatedResponseStoreProductItemsInner) GetImageMediaAssetIdOk() (*string, bool)`

GetImageMediaAssetIdOk returns a tuple with the ImageMediaAssetId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageMediaAssetId

`func (o *PaginatedResponseStoreProductItemsInner) SetImageMediaAssetId(v string)`

SetImageMediaAssetId sets ImageMediaAssetId field to given value.

### HasImageMediaAssetId

`func (o *PaginatedResponseStoreProductItemsInner) HasImageMediaAssetId() bool`

HasImageMediaAssetId returns a boolean if a field has been set.

### SetImageMediaAssetIdNil

`func (o *PaginatedResponseStoreProductItemsInner) SetImageMediaAssetIdNil(b bool)`

 SetImageMediaAssetIdNil sets the value for ImageMediaAssetId to be an explicit nil

### UnsetImageMediaAssetId
`func (o *PaginatedResponseStoreProductItemsInner) UnsetImageMediaAssetId()`

UnsetImageMediaAssetId ensures that no value is present for ImageMediaAssetId, not even an explicit nil
### GetName

`func (o *PaginatedResponseStoreProductItemsInner) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PaginatedResponseStoreProductItemsInner) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PaginatedResponseStoreProductItemsInner) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetOrgId

`func (o *PaginatedResponseStoreProductItemsInner) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *PaginatedResponseStoreProductItemsInner) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *PaginatedResponseStoreProductItemsInner) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetPurgeAt

`func (o *PaginatedResponseStoreProductItemsInner) GetPurgeAt() string`

GetPurgeAt returns the PurgeAt field if non-nil, zero value otherwise.

### GetPurgeAtOk

`func (o *PaginatedResponseStoreProductItemsInner) GetPurgeAtOk() (*string, bool)`

GetPurgeAtOk returns a tuple with the PurgeAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurgeAt

`func (o *PaginatedResponseStoreProductItemsInner) SetPurgeAt(v string)`

SetPurgeAt sets PurgeAt field to given value.

### HasPurgeAt

`func (o *PaginatedResponseStoreProductItemsInner) HasPurgeAt() bool`

HasPurgeAt returns a boolean if a field has been set.

### SetPurgeAtNil

`func (o *PaginatedResponseStoreProductItemsInner) SetPurgeAtNil(b bool)`

 SetPurgeAtNil sets the value for PurgeAt to be an explicit nil

### UnsetPurgeAt
`func (o *PaginatedResponseStoreProductItemsInner) UnsetPurgeAt()`

UnsetPurgeAt ensures that no value is present for PurgeAt, not even an explicit nil
### GetUpdatedAt

`func (o *PaginatedResponseStoreProductItemsInner) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *PaginatedResponseStoreProductItemsInner) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *PaginatedResponseStoreProductItemsInner) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *PaginatedResponseStoreProductItemsInner) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *PaginatedResponseStoreProductItemsInner) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *PaginatedResponseStoreProductItemsInner) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
