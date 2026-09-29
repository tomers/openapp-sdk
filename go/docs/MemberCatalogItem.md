# MemberCatalogItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExternalUrl** | Pointer to **NullableString** |  | [optional]
**ImageMediaAssetId** | Pointer to **NullableString** |  | [optional]
**InventoryId** | **NullableString** |  |
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**PriceCents** | **int64** |  |
**ProductId** | **NullableString** |  |
**StockQuantity** | Pointer to **NullableInt32** |  | [optional]

## Methods

### NewMemberCatalogItem

`func NewMemberCatalogItem(inventoryId NullableString, name LocalizedString, priceCents int64, productId NullableString, ) *MemberCatalogItem`

NewMemberCatalogItem instantiates a new MemberCatalogItem object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMemberCatalogItemWithDefaults

`func NewMemberCatalogItemWithDefaults() *MemberCatalogItem`

NewMemberCatalogItemWithDefaults instantiates a new MemberCatalogItem object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExternalUrl

`func (o *MemberCatalogItem) GetExternalUrl() string`

GetExternalUrl returns the ExternalUrl field if non-nil, zero value otherwise.

### GetExternalUrlOk

`func (o *MemberCatalogItem) GetExternalUrlOk() (*string, bool)`

GetExternalUrlOk returns a tuple with the ExternalUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalUrl

`func (o *MemberCatalogItem) SetExternalUrl(v string)`

SetExternalUrl sets ExternalUrl field to given value.

### HasExternalUrl

`func (o *MemberCatalogItem) HasExternalUrl() bool`

HasExternalUrl returns a boolean if a field has been set.

### SetExternalUrlNil

`func (o *MemberCatalogItem) SetExternalUrlNil(b bool)`

 SetExternalUrlNil sets the value for ExternalUrl to be an explicit nil

### UnsetExternalUrl
`func (o *MemberCatalogItem) UnsetExternalUrl()`

UnsetExternalUrl ensures that no value is present for ExternalUrl, not even an explicit nil
### GetImageMediaAssetId

`func (o *MemberCatalogItem) GetImageMediaAssetId() string`

GetImageMediaAssetId returns the ImageMediaAssetId field if non-nil, zero value otherwise.

### GetImageMediaAssetIdOk

`func (o *MemberCatalogItem) GetImageMediaAssetIdOk() (*string, bool)`

GetImageMediaAssetIdOk returns a tuple with the ImageMediaAssetId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageMediaAssetId

`func (o *MemberCatalogItem) SetImageMediaAssetId(v string)`

SetImageMediaAssetId sets ImageMediaAssetId field to given value.

### HasImageMediaAssetId

`func (o *MemberCatalogItem) HasImageMediaAssetId() bool`

HasImageMediaAssetId returns a boolean if a field has been set.

### SetImageMediaAssetIdNil

`func (o *MemberCatalogItem) SetImageMediaAssetIdNil(b bool)`

 SetImageMediaAssetIdNil sets the value for ImageMediaAssetId to be an explicit nil

### UnsetImageMediaAssetId
`func (o *MemberCatalogItem) UnsetImageMediaAssetId()`

UnsetImageMediaAssetId ensures that no value is present for ImageMediaAssetId, not even an explicit nil
### GetInventoryId

`func (o *MemberCatalogItem) GetInventoryId() string`

GetInventoryId returns the InventoryId field if non-nil, zero value otherwise.

### GetInventoryIdOk

`func (o *MemberCatalogItem) GetInventoryIdOk() (*string, bool)`

GetInventoryIdOk returns a tuple with the InventoryId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInventoryId

`func (o *MemberCatalogItem) SetInventoryId(v string)`

SetInventoryId sets InventoryId field to given value.


### SetInventoryIdNil

`func (o *MemberCatalogItem) SetInventoryIdNil(b bool)`

 SetInventoryIdNil sets the value for InventoryId to be an explicit nil

### UnsetInventoryId
`func (o *MemberCatalogItem) UnsetInventoryId()`

UnsetInventoryId ensures that no value is present for InventoryId, not even an explicit nil
### GetName

`func (o *MemberCatalogItem) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *MemberCatalogItem) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *MemberCatalogItem) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetPriceCents

`func (o *MemberCatalogItem) GetPriceCents() int64`

GetPriceCents returns the PriceCents field if non-nil, zero value otherwise.

### GetPriceCentsOk

`func (o *MemberCatalogItem) GetPriceCentsOk() (*int64, bool)`

GetPriceCentsOk returns a tuple with the PriceCents field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPriceCents

`func (o *MemberCatalogItem) SetPriceCents(v int64)`

SetPriceCents sets PriceCents field to given value.


### GetProductId

`func (o *MemberCatalogItem) GetProductId() string`

GetProductId returns the ProductId field if non-nil, zero value otherwise.

### GetProductIdOk

`func (o *MemberCatalogItem) GetProductIdOk() (*string, bool)`

GetProductIdOk returns a tuple with the ProductId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProductId

`func (o *MemberCatalogItem) SetProductId(v string)`

SetProductId sets ProductId field to given value.


### SetProductIdNil

`func (o *MemberCatalogItem) SetProductIdNil(b bool)`

 SetProductIdNil sets the value for ProductId to be an explicit nil

### UnsetProductId
`func (o *MemberCatalogItem) UnsetProductId()`

UnsetProductId ensures that no value is present for ProductId, not even an explicit nil
### GetStockQuantity

`func (o *MemberCatalogItem) GetStockQuantity() int32`

GetStockQuantity returns the StockQuantity field if non-nil, zero value otherwise.

### GetStockQuantityOk

`func (o *MemberCatalogItem) GetStockQuantityOk() (*int32, bool)`

GetStockQuantityOk returns a tuple with the StockQuantity field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStockQuantity

`func (o *MemberCatalogItem) SetStockQuantity(v int32)`

SetStockQuantity sets StockQuantity field to given value.

### HasStockQuantity

`func (o *MemberCatalogItem) HasStockQuantity() bool`

HasStockQuantity returns a boolean if a field has been set.

### SetStockQuantityNil

`func (o *MemberCatalogItem) SetStockQuantityNil(b bool)`

 SetStockQuantityNil sets the value for StockQuantity to be an explicit nil

### UnsetStockQuantity
`func (o *MemberCatalogItem) UnsetStockQuantity()`

UnsetStockQuantity ensures that no value is present for StockQuantity, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
