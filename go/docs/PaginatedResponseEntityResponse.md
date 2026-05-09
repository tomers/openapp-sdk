# PaginatedResponseEntityResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Items** | [**[]PaginatedResponseEntityResponseItemsInner**](PaginatedResponseEntityResponseItemsInner.md) |  |
**Total** | **int64** |  |

## Methods

### NewPaginatedResponseEntityResponse

`func NewPaginatedResponseEntityResponse(items []PaginatedResponseEntityResponseItemsInner, total int64, ) *PaginatedResponseEntityResponse`

NewPaginatedResponseEntityResponse instantiates a new PaginatedResponseEntityResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseEntityResponseWithDefaults

`func NewPaginatedResponseEntityResponseWithDefaults() *PaginatedResponseEntityResponse`

NewPaginatedResponseEntityResponseWithDefaults instantiates a new PaginatedResponseEntityResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetItems

`func (o *PaginatedResponseEntityResponse) GetItems() []PaginatedResponseEntityResponseItemsInner`

GetItems returns the Items field if non-nil, zero value otherwise.

### GetItemsOk

`func (o *PaginatedResponseEntityResponse) GetItemsOk() (*[]PaginatedResponseEntityResponseItemsInner, bool)`

GetItemsOk returns a tuple with the Items field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetItems

`func (o *PaginatedResponseEntityResponse) SetItems(v []PaginatedResponseEntityResponseItemsInner)`

SetItems sets Items field to given value.


### GetTotal

`func (o *PaginatedResponseEntityResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *PaginatedResponseEntityResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *PaginatedResponseEntityResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
