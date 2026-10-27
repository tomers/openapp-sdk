# PaginatedResponseWifiNetworkResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Items** | [**[]PaginatedResponseWifiNetworkResponseItemsInner**](PaginatedResponseWifiNetworkResponseItemsInner.md) |  |
**Total** | **int64** |  |

## Methods

### NewPaginatedResponseWifiNetworkResponse

`func NewPaginatedResponseWifiNetworkResponse(items []PaginatedResponseWifiNetworkResponseItemsInner, total int64, ) *PaginatedResponseWifiNetworkResponse`

NewPaginatedResponseWifiNetworkResponse instantiates a new PaginatedResponseWifiNetworkResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseWifiNetworkResponseWithDefaults

`func NewPaginatedResponseWifiNetworkResponseWithDefaults() *PaginatedResponseWifiNetworkResponse`

NewPaginatedResponseWifiNetworkResponseWithDefaults instantiates a new PaginatedResponseWifiNetworkResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetItems

`func (o *PaginatedResponseWifiNetworkResponse) GetItems() []PaginatedResponseWifiNetworkResponseItemsInner`

GetItems returns the Items field if non-nil, zero value otherwise.

### GetItemsOk

`func (o *PaginatedResponseWifiNetworkResponse) GetItemsOk() (*[]PaginatedResponseWifiNetworkResponseItemsInner, bool)`

GetItemsOk returns a tuple with the Items field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetItems

`func (o *PaginatedResponseWifiNetworkResponse) SetItems(v []PaginatedResponseWifiNetworkResponseItemsInner)`

SetItems sets Items field to given value.


### GetTotal

`func (o *PaginatedResponseWifiNetworkResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *PaginatedResponseWifiNetworkResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *PaginatedResponseWifiNetworkResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
