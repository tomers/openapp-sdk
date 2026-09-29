# PaginatedResponseAgentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Items** | [**[]PaginatedResponseAgentResponseItemsInner**](PaginatedResponseAgentResponseItemsInner.md) |  |
**Total** | **int64** |  |

## Methods

### NewPaginatedResponseAgentResponse

`func NewPaginatedResponseAgentResponse(items []PaginatedResponseAgentResponseItemsInner, total int64, ) *PaginatedResponseAgentResponse`

NewPaginatedResponseAgentResponse instantiates a new PaginatedResponseAgentResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseAgentResponseWithDefaults

`func NewPaginatedResponseAgentResponseWithDefaults() *PaginatedResponseAgentResponse`

NewPaginatedResponseAgentResponseWithDefaults instantiates a new PaginatedResponseAgentResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetItems

`func (o *PaginatedResponseAgentResponse) GetItems() []PaginatedResponseAgentResponseItemsInner`

GetItems returns the Items field if non-nil, zero value otherwise.

### GetItemsOk

`func (o *PaginatedResponseAgentResponse) GetItemsOk() (*[]PaginatedResponseAgentResponseItemsInner, bool)`

GetItemsOk returns a tuple with the Items field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetItems

`func (o *PaginatedResponseAgentResponse) SetItems(v []PaginatedResponseAgentResponseItemsInner)`

SetItems sets Items field to given value.


### GetTotal

`func (o *PaginatedResponseAgentResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *PaginatedResponseAgentResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *PaginatedResponseAgentResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
