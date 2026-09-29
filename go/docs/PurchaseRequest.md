# PurchaseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BillingAccountId** | Pointer to **NullableString** | Optional billing account to charge (must be chargeable by the buyer). Defaults to the member&#39;s account. | [optional]
**InventoryId** | **string** |  |
**Quantity** | Pointer to **NullableInt32** |  | [optional]

## Methods

### NewPurchaseRequest

`func NewPurchaseRequest(inventoryId string, ) *PurchaseRequest`

NewPurchaseRequest instantiates a new PurchaseRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPurchaseRequestWithDefaults

`func NewPurchaseRequestWithDefaults() *PurchaseRequest`

NewPurchaseRequestWithDefaults instantiates a new PurchaseRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBillingAccountId

`func (o *PurchaseRequest) GetBillingAccountId() string`

GetBillingAccountId returns the BillingAccountId field if non-nil, zero value otherwise.

### GetBillingAccountIdOk

`func (o *PurchaseRequest) GetBillingAccountIdOk() (*string, bool)`

GetBillingAccountIdOk returns a tuple with the BillingAccountId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBillingAccountId

`func (o *PurchaseRequest) SetBillingAccountId(v string)`

SetBillingAccountId sets BillingAccountId field to given value.

### HasBillingAccountId

`func (o *PurchaseRequest) HasBillingAccountId() bool`

HasBillingAccountId returns a boolean if a field has been set.

### SetBillingAccountIdNil

`func (o *PurchaseRequest) SetBillingAccountIdNil(b bool)`

 SetBillingAccountIdNil sets the value for BillingAccountId to be an explicit nil

### UnsetBillingAccountId
`func (o *PurchaseRequest) UnsetBillingAccountId()`

UnsetBillingAccountId ensures that no value is present for BillingAccountId, not even an explicit nil
### GetInventoryId

`func (o *PurchaseRequest) GetInventoryId() string`

GetInventoryId returns the InventoryId field if non-nil, zero value otherwise.

### GetInventoryIdOk

`func (o *PurchaseRequest) GetInventoryIdOk() (*string, bool)`

GetInventoryIdOk returns a tuple with the InventoryId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInventoryId

`func (o *PurchaseRequest) SetInventoryId(v string)`

SetInventoryId sets InventoryId field to given value.


### GetQuantity

`func (o *PurchaseRequest) GetQuantity() int32`

GetQuantity returns the Quantity field if non-nil, zero value otherwise.

### GetQuantityOk

`func (o *PurchaseRequest) GetQuantityOk() (*int32, bool)`

GetQuantityOk returns a tuple with the Quantity field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQuantity

`func (o *PurchaseRequest) SetQuantity(v int32)`

SetQuantity sets Quantity field to given value.

### HasQuantity

`func (o *PurchaseRequest) HasQuantity() bool`

HasQuantity returns a boolean if a field has been set.

### SetQuantityNil

`func (o *PurchaseRequest) SetQuantityNil(b bool)`

 SetQuantityNil sets the value for Quantity to be an explicit nil

### UnsetQuantity
`func (o *PurchaseRequest) UnsetQuantity()`

UnsetQuantity ensures that no value is present for Quantity, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
