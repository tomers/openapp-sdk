# PublicCatalogItem

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

### NewPublicCatalogItem

`func NewPublicCatalogItem(inventoryId NullableString, name LocalizedString, priceCents int64, productId NullableString, ) *PublicCatalogItem`

NewPublicCatalogItem instantiates a new PublicCatalogItem object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicCatalogItemWithDefaults

`func NewPublicCatalogItemWithDefaults() *PublicCatalogItem`

NewPublicCatalogItemWithDefaults instantiates a new PublicCatalogItem object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExternalUrl

`func (o *PublicCatalogItem) GetExternalUrl() string`

GetExternalUrl returns the ExternalUrl field if non-nil, zero value otherwise.

### GetExternalUrlOk

`func (o *PublicCatalogItem) GetExternalUrlOk() (*string, bool)`

GetExternalUrlOk returns a tuple with the ExternalUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalUrl

`func (o *PublicCatalogItem) SetExternalUrl(v string)`

SetExternalUrl sets ExternalUrl field to given value.

### HasExternalUrl

`func (o *PublicCatalogItem) HasExternalUrl() bool`

HasExternalUrl returns a boolean if a field has been set.

### SetExternalUrlNil

`func (o *PublicCatalogItem) SetExternalUrlNil(b bool)`

 SetExternalUrlNil sets the value for ExternalUrl to be an explicit nil

### UnsetExternalUrl
`func (o *PublicCatalogItem) UnsetExternalUrl()`

UnsetExternalUrl ensures that no value is present for ExternalUrl, not even an explicit nil
### GetImageMediaAssetId

`func (o *PublicCatalogItem) GetImageMediaAssetId() string`

GetImageMediaAssetId returns the ImageMediaAssetId field if non-nil, zero value otherwise.

### GetImageMediaAssetIdOk

`func (o *PublicCatalogItem) GetImageMediaAssetIdOk() (*string, bool)`

GetImageMediaAssetIdOk returns a tuple with the ImageMediaAssetId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageMediaAssetId

`func (o *PublicCatalogItem) SetImageMediaAssetId(v string)`

SetImageMediaAssetId sets ImageMediaAssetId field to given value.

### HasImageMediaAssetId

`func (o *PublicCatalogItem) HasImageMediaAssetId() bool`

HasImageMediaAssetId returns a boolean if a field has been set.

### SetImageMediaAssetIdNil

`func (o *PublicCatalogItem) SetImageMediaAssetIdNil(b bool)`

 SetImageMediaAssetIdNil sets the value for ImageMediaAssetId to be an explicit nil

### UnsetImageMediaAssetId
`func (o *PublicCatalogItem) UnsetImageMediaAssetId()`

UnsetImageMediaAssetId ensures that no value is present for ImageMediaAssetId, not even an explicit nil
### GetInventoryId

`func (o *PublicCatalogItem) GetInventoryId() string`

GetInventoryId returns the InventoryId field if non-nil, zero value otherwise.

### GetInventoryIdOk

`func (o *PublicCatalogItem) GetInventoryIdOk() (*string, bool)`

GetInventoryIdOk returns a tuple with the InventoryId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInventoryId

`func (o *PublicCatalogItem) SetInventoryId(v string)`

SetInventoryId sets InventoryId field to given value.


### SetInventoryIdNil

`func (o *PublicCatalogItem) SetInventoryIdNil(b bool)`

 SetInventoryIdNil sets the value for InventoryId to be an explicit nil

### UnsetInventoryId
`func (o *PublicCatalogItem) UnsetInventoryId()`

UnsetInventoryId ensures that no value is present for InventoryId, not even an explicit nil
### GetName

`func (o *PublicCatalogItem) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PublicCatalogItem) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PublicCatalogItem) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetPriceCents

`func (o *PublicCatalogItem) GetPriceCents() int64`

GetPriceCents returns the PriceCents field if non-nil, zero value otherwise.

### GetPriceCentsOk

`func (o *PublicCatalogItem) GetPriceCentsOk() (*int64, bool)`

GetPriceCentsOk returns a tuple with the PriceCents field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPriceCents

`func (o *PublicCatalogItem) SetPriceCents(v int64)`

SetPriceCents sets PriceCents field to given value.


### GetProductId

`func (o *PublicCatalogItem) GetProductId() string`

GetProductId returns the ProductId field if non-nil, zero value otherwise.

### GetProductIdOk

`func (o *PublicCatalogItem) GetProductIdOk() (*string, bool)`

GetProductIdOk returns a tuple with the ProductId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProductId

`func (o *PublicCatalogItem) SetProductId(v string)`

SetProductId sets ProductId field to given value.


### SetProductIdNil

`func (o *PublicCatalogItem) SetProductIdNil(b bool)`

 SetProductIdNil sets the value for ProductId to be an explicit nil

### UnsetProductId
`func (o *PublicCatalogItem) UnsetProductId()`

UnsetProductId ensures that no value is present for ProductId, not even an explicit nil
### GetStockQuantity

`func (o *PublicCatalogItem) GetStockQuantity() int32`

GetStockQuantity returns the StockQuantity field if non-nil, zero value otherwise.

### GetStockQuantityOk

`func (o *PublicCatalogItem) GetStockQuantityOk() (*int32, bool)`

GetStockQuantityOk returns a tuple with the StockQuantity field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStockQuantity

`func (o *PublicCatalogItem) SetStockQuantity(v int32)`

SetStockQuantity sets StockQuantity field to given value.

### HasStockQuantity

`func (o *PublicCatalogItem) HasStockQuantity() bool`

HasStockQuantity returns a boolean if a field has been set.

### SetStockQuantityNil

`func (o *PublicCatalogItem) SetStockQuantityNil(b bool)`

 SetStockQuantityNil sets the value for StockQuantity to be an explicit nil

### UnsetStockQuantity
`func (o *PublicCatalogItem) UnsetStockQuantity()`

UnsetStockQuantity ensures that no value is present for StockQuantity, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
