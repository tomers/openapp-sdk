# CreateInventoryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IsActive** | Pointer to **NullableBool** |  | [optional]
**PriceCents** | **int64** |  |
**ProductId** | **string** |  |
**StockQuantity** | Pointer to **NullableInt32** |  | [optional]

## Methods

### NewCreateInventoryRequest

`func NewCreateInventoryRequest(priceCents int64, productId string, ) *CreateInventoryRequest`

NewCreateInventoryRequest instantiates a new CreateInventoryRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateInventoryRequestWithDefaults

`func NewCreateInventoryRequestWithDefaults() *CreateInventoryRequest`

NewCreateInventoryRequestWithDefaults instantiates a new CreateInventoryRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIsActive

`func (o *CreateInventoryRequest) GetIsActive() bool`

GetIsActive returns the IsActive field if non-nil, zero value otherwise.

### GetIsActiveOk

`func (o *CreateInventoryRequest) GetIsActiveOk() (*bool, bool)`

GetIsActiveOk returns a tuple with the IsActive field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsActive

`func (o *CreateInventoryRequest) SetIsActive(v bool)`

SetIsActive sets IsActive field to given value.

### HasIsActive

`func (o *CreateInventoryRequest) HasIsActive() bool`

HasIsActive returns a boolean if a field has been set.

### SetIsActiveNil

`func (o *CreateInventoryRequest) SetIsActiveNil(b bool)`

 SetIsActiveNil sets the value for IsActive to be an explicit nil

### UnsetIsActive
`func (o *CreateInventoryRequest) UnsetIsActive()`

UnsetIsActive ensures that no value is present for IsActive, not even an explicit nil
### GetPriceCents

`func (o *CreateInventoryRequest) GetPriceCents() int64`

GetPriceCents returns the PriceCents field if non-nil, zero value otherwise.

### GetPriceCentsOk

`func (o *CreateInventoryRequest) GetPriceCentsOk() (*int64, bool)`

GetPriceCentsOk returns a tuple with the PriceCents field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPriceCents

`func (o *CreateInventoryRequest) SetPriceCents(v int64)`

SetPriceCents sets PriceCents field to given value.


### GetProductId

`func (o *CreateInventoryRequest) GetProductId() string`

GetProductId returns the ProductId field if non-nil, zero value otherwise.

### GetProductIdOk

`func (o *CreateInventoryRequest) GetProductIdOk() (*string, bool)`

GetProductIdOk returns a tuple with the ProductId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProductId

`func (o *CreateInventoryRequest) SetProductId(v string)`

SetProductId sets ProductId field to given value.


### GetStockQuantity

`func (o *CreateInventoryRequest) GetStockQuantity() int32`

GetStockQuantity returns the StockQuantity field if non-nil, zero value otherwise.

### GetStockQuantityOk

`func (o *CreateInventoryRequest) GetStockQuantityOk() (*int32, bool)`

GetStockQuantityOk returns a tuple with the StockQuantity field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStockQuantity

`func (o *CreateInventoryRequest) SetStockQuantity(v int32)`

SetStockQuantity sets StockQuantity field to given value.

### HasStockQuantity

`func (o *CreateInventoryRequest) HasStockQuantity() bool`

HasStockQuantity returns a boolean if a field has been set.

### SetStockQuantityNil

`func (o *CreateInventoryRequest) SetStockQuantityNil(b bool)`

 SetStockQuantityNil sets the value for StockQuantity to be an explicit nil

### UnsetStockQuantity
`func (o *CreateInventoryRequest) UnsetStockQuantity()`

UnsetStockQuantity ensures that no value is present for StockQuantity, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
