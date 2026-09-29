# PaginatedResponseAccessPortalListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Items** | [**[]PaginatedResponseAccessPortalListItemItemsInner**](PaginatedResponseAccessPortalListItemItemsInner.md) |  |
**Total** | **int64** |  |

## Methods

### NewPaginatedResponseAccessPortalListItem

`func NewPaginatedResponseAccessPortalListItem(items []PaginatedResponseAccessPortalListItemItemsInner, total int64, ) *PaginatedResponseAccessPortalListItem`

NewPaginatedResponseAccessPortalListItem instantiates a new PaginatedResponseAccessPortalListItem object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseAccessPortalListItemWithDefaults

`func NewPaginatedResponseAccessPortalListItemWithDefaults() *PaginatedResponseAccessPortalListItem`

NewPaginatedResponseAccessPortalListItemWithDefaults instantiates a new PaginatedResponseAccessPortalListItem object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetItems

`func (o *PaginatedResponseAccessPortalListItem) GetItems() []PaginatedResponseAccessPortalListItemItemsInner`

GetItems returns the Items field if non-nil, zero value otherwise.

### GetItemsOk

`func (o *PaginatedResponseAccessPortalListItem) GetItemsOk() (*[]PaginatedResponseAccessPortalListItemItemsInner, bool)`

GetItemsOk returns a tuple with the Items field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetItems

`func (o *PaginatedResponseAccessPortalListItem) SetItems(v []PaginatedResponseAccessPortalListItemItemsInner)`

SetItems sets Items field to given value.


### GetTotal

`func (o *PaginatedResponseAccessPortalListItem) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *PaginatedResponseAccessPortalListItem) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *PaginatedResponseAccessPortalListItem) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
