# PaginatedResponseZoneResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Items** | [**[]Zone**](Zone.md) |  |
**Total** | **int64** |  |

## Methods

### NewPaginatedResponseZoneResponse

`func NewPaginatedResponseZoneResponse(items []Zone, total int64, ) *PaginatedResponseZoneResponse`

NewPaginatedResponseZoneResponse instantiates a new PaginatedResponseZoneResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseZoneResponseWithDefaults

`func NewPaginatedResponseZoneResponseWithDefaults() *PaginatedResponseZoneResponse`

NewPaginatedResponseZoneResponseWithDefaults instantiates a new PaginatedResponseZoneResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetItems

`func (o *PaginatedResponseZoneResponse) GetItems() []Zone`

GetItems returns the Items field if non-nil, zero value otherwise.

### GetItemsOk

`func (o *PaginatedResponseZoneResponse) GetItemsOk() (*[]Zone, bool)`

GetItemsOk returns a tuple with the Items field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetItems

`func (o *PaginatedResponseZoneResponse) SetItems(v []Zone)`

SetItems sets Items field to given value.


### GetTotal

`func (o *PaginatedResponseZoneResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *PaginatedResponseZoneResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *PaginatedResponseZoneResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
