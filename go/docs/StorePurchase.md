# StorePurchase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BillingAccountId** | Pointer to **NullableString** |  | [optional]
**BuyerUserId** | Pointer to **NullableString** |  | [optional]
**CreatedAt** | **string** |  |
**Currency** | **string** |  |
**Id** | **string** |  |
**InventoryId** | Pointer to **NullableString** |  | [optional]
**OrgId** | **string** |  |
**ProductId** | Pointer to **NullableString** |  | [optional]
**ProductNameSnapshot** | [**LocalizedString**](LocalizedString.md) | Localized product name captured at purchase time. |
**Quantity** | **int32** |  |
**StoreId** | **string** |  |
**TotalCents** | **int64** |  |
**UnitPriceCents** | **int64** |  |

## Methods

### NewStorePurchase

`func NewStorePurchase(createdAt string, currency string, id string, orgId string, productNameSnapshot LocalizedString, quantity int32, storeId string, totalCents int64, unitPriceCents int64, ) *StorePurchase`

NewStorePurchase instantiates a new StorePurchase object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewStorePurchaseWithDefaults

`func NewStorePurchaseWithDefaults() *StorePurchase`

NewStorePurchaseWithDefaults instantiates a new StorePurchase object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBillingAccountId

`func (o *StorePurchase) GetBillingAccountId() string`

GetBillingAccountId returns the BillingAccountId field if non-nil, zero value otherwise.

### GetBillingAccountIdOk

`func (o *StorePurchase) GetBillingAccountIdOk() (*string, bool)`

GetBillingAccountIdOk returns a tuple with the BillingAccountId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBillingAccountId

`func (o *StorePurchase) SetBillingAccountId(v string)`

SetBillingAccountId sets BillingAccountId field to given value.

### HasBillingAccountId

`func (o *StorePurchase) HasBillingAccountId() bool`

HasBillingAccountId returns a boolean if a field has been set.

### SetBillingAccountIdNil

`func (o *StorePurchase) SetBillingAccountIdNil(b bool)`

 SetBillingAccountIdNil sets the value for BillingAccountId to be an explicit nil

### UnsetBillingAccountId
`func (o *StorePurchase) UnsetBillingAccountId()`

UnsetBillingAccountId ensures that no value is present for BillingAccountId, not even an explicit nil
### GetBuyerUserId

`func (o *StorePurchase) GetBuyerUserId() string`

GetBuyerUserId returns the BuyerUserId field if non-nil, zero value otherwise.

### GetBuyerUserIdOk

`func (o *StorePurchase) GetBuyerUserIdOk() (*string, bool)`

GetBuyerUserIdOk returns a tuple with the BuyerUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBuyerUserId

`func (o *StorePurchase) SetBuyerUserId(v string)`

SetBuyerUserId sets BuyerUserId field to given value.

### HasBuyerUserId

`func (o *StorePurchase) HasBuyerUserId() bool`

HasBuyerUserId returns a boolean if a field has been set.

### SetBuyerUserIdNil

`func (o *StorePurchase) SetBuyerUserIdNil(b bool)`

 SetBuyerUserIdNil sets the value for BuyerUserId to be an explicit nil

### UnsetBuyerUserId
`func (o *StorePurchase) UnsetBuyerUserId()`

UnsetBuyerUserId ensures that no value is present for BuyerUserId, not even an explicit nil
### GetCreatedAt

`func (o *StorePurchase) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *StorePurchase) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *StorePurchase) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.


### GetCurrency

`func (o *StorePurchase) GetCurrency() string`

GetCurrency returns the Currency field if non-nil, zero value otherwise.

### GetCurrencyOk

`func (o *StorePurchase) GetCurrencyOk() (*string, bool)`

GetCurrencyOk returns a tuple with the Currency field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrency

`func (o *StorePurchase) SetCurrency(v string)`

SetCurrency sets Currency field to given value.


### GetId

`func (o *StorePurchase) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *StorePurchase) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *StorePurchase) SetId(v string)`

SetId sets Id field to given value.


### GetInventoryId

`func (o *StorePurchase) GetInventoryId() string`

GetInventoryId returns the InventoryId field if non-nil, zero value otherwise.

### GetInventoryIdOk

`func (o *StorePurchase) GetInventoryIdOk() (*string, bool)`

GetInventoryIdOk returns a tuple with the InventoryId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInventoryId

`func (o *StorePurchase) SetInventoryId(v string)`

SetInventoryId sets InventoryId field to given value.

### HasInventoryId

`func (o *StorePurchase) HasInventoryId() bool`

HasInventoryId returns a boolean if a field has been set.

### SetInventoryIdNil

`func (o *StorePurchase) SetInventoryIdNil(b bool)`

 SetInventoryIdNil sets the value for InventoryId to be an explicit nil

### UnsetInventoryId
`func (o *StorePurchase) UnsetInventoryId()`

UnsetInventoryId ensures that no value is present for InventoryId, not even an explicit nil
### GetOrgId

`func (o *StorePurchase) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *StorePurchase) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *StorePurchase) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetProductId

`func (o *StorePurchase) GetProductId() string`

GetProductId returns the ProductId field if non-nil, zero value otherwise.

### GetProductIdOk

`func (o *StorePurchase) GetProductIdOk() (*string, bool)`

GetProductIdOk returns a tuple with the ProductId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProductId

`func (o *StorePurchase) SetProductId(v string)`

SetProductId sets ProductId field to given value.

### HasProductId

`func (o *StorePurchase) HasProductId() bool`

HasProductId returns a boolean if a field has been set.

### SetProductIdNil

`func (o *StorePurchase) SetProductIdNil(b bool)`

 SetProductIdNil sets the value for ProductId to be an explicit nil

### UnsetProductId
`func (o *StorePurchase) UnsetProductId()`

UnsetProductId ensures that no value is present for ProductId, not even an explicit nil
### GetProductNameSnapshot

`func (o *StorePurchase) GetProductNameSnapshot() LocalizedString`

GetProductNameSnapshot returns the ProductNameSnapshot field if non-nil, zero value otherwise.

### GetProductNameSnapshotOk

`func (o *StorePurchase) GetProductNameSnapshotOk() (*LocalizedString, bool)`

GetProductNameSnapshotOk returns a tuple with the ProductNameSnapshot field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProductNameSnapshot

`func (o *StorePurchase) SetProductNameSnapshot(v LocalizedString)`

SetProductNameSnapshot sets ProductNameSnapshot field to given value.


### GetQuantity

`func (o *StorePurchase) GetQuantity() int32`

GetQuantity returns the Quantity field if non-nil, zero value otherwise.

### GetQuantityOk

`func (o *StorePurchase) GetQuantityOk() (*int32, bool)`

GetQuantityOk returns a tuple with the Quantity field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQuantity

`func (o *StorePurchase) SetQuantity(v int32)`

SetQuantity sets Quantity field to given value.


### GetStoreId

`func (o *StorePurchase) GetStoreId() string`

GetStoreId returns the StoreId field if non-nil, zero value otherwise.

### GetStoreIdOk

`func (o *StorePurchase) GetStoreIdOk() (*string, bool)`

GetStoreIdOk returns a tuple with the StoreId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStoreId

`func (o *StorePurchase) SetStoreId(v string)`

SetStoreId sets StoreId field to given value.


### GetTotalCents

`func (o *StorePurchase) GetTotalCents() int64`

GetTotalCents returns the TotalCents field if non-nil, zero value otherwise.

### GetTotalCentsOk

`func (o *StorePurchase) GetTotalCentsOk() (*int64, bool)`

GetTotalCentsOk returns a tuple with the TotalCents field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalCents

`func (o *StorePurchase) SetTotalCents(v int64)`

SetTotalCents sets TotalCents field to given value.


### GetUnitPriceCents

`func (o *StorePurchase) GetUnitPriceCents() int64`

GetUnitPriceCents returns the UnitPriceCents field if non-nil, zero value otherwise.

### GetUnitPriceCentsOk

`func (o *StorePurchase) GetUnitPriceCentsOk() (*int64, bool)`

GetUnitPriceCentsOk returns a tuple with the UnitPriceCents field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnitPriceCents

`func (o *StorePurchase) SetUnitPriceCents(v int64)`

SetUnitPriceCents sets UnitPriceCents field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
