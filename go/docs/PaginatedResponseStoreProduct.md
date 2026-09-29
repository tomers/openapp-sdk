# PaginatedResponseStoreProduct

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Items** | [**[]PaginatedResponseStoreProductItemsInner**](PaginatedResponseStoreProductItemsInner.md) |  |
**Total** | **int64** |  |

## Methods

### NewPaginatedResponseStoreProduct

`func NewPaginatedResponseStoreProduct(items []PaginatedResponseStoreProductItemsInner, total int64, ) *PaginatedResponseStoreProduct`

NewPaginatedResponseStoreProduct instantiates a new PaginatedResponseStoreProduct object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseStoreProductWithDefaults

`func NewPaginatedResponseStoreProductWithDefaults() *PaginatedResponseStoreProduct`

NewPaginatedResponseStoreProductWithDefaults instantiates a new PaginatedResponseStoreProduct object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetItems

`func (o *PaginatedResponseStoreProduct) GetItems() []PaginatedResponseStoreProductItemsInner`

GetItems returns the Items field if non-nil, zero value otherwise.

### GetItemsOk

`func (o *PaginatedResponseStoreProduct) GetItemsOk() (*[]PaginatedResponseStoreProductItemsInner, bool)`

GetItemsOk returns a tuple with the Items field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetItems

`func (o *PaginatedResponseStoreProduct) SetItems(v []PaginatedResponseStoreProductItemsInner)`

SetItems sets Items field to given value.


### GetTotal

`func (o *PaginatedResponseStoreProduct) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *PaginatedResponseStoreProduct) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *PaginatedResponseStoreProduct) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
