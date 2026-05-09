# MultiResourceOutputOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Limit** | **int32** | Pagination: max items to return. Default from config when not specified. |
**Offset** | **int32** | Pagination: items to skip. Default 0 when not specified. |
**OnlyDeleted** | **bool** | If true, return *only* soft-deleted items.  Note: This implies &#x60;include_deleted&#x3D;true&#x60; at the API boundary, but repository/storage implementations should treat this as a separate filter. |
**Single** | [**SingleResourceOutputOptions**](SingleResourceOutputOptions.md) | Single-resource options (include_deleted, include_metadata). Accessed flat via &#x60;Deref&#x60;. |

## Methods

### NewMultiResourceOutputOptions

`func NewMultiResourceOutputOptions(limit int32, offset int32, onlyDeleted bool, single SingleResourceOutputOptions, ) *MultiResourceOutputOptions`

NewMultiResourceOutputOptions instantiates a new MultiResourceOutputOptions object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMultiResourceOutputOptionsWithDefaults

`func NewMultiResourceOutputOptionsWithDefaults() *MultiResourceOutputOptions`

NewMultiResourceOutputOptionsWithDefaults instantiates a new MultiResourceOutputOptions object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLimit

`func (o *MultiResourceOutputOptions) GetLimit() int32`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *MultiResourceOutputOptions) GetLimitOk() (*int32, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *MultiResourceOutputOptions) SetLimit(v int32)`

SetLimit sets Limit field to given value.


### GetOffset

`func (o *MultiResourceOutputOptions) GetOffset() int32`

GetOffset returns the Offset field if non-nil, zero value otherwise.

### GetOffsetOk

`func (o *MultiResourceOutputOptions) GetOffsetOk() (*int32, bool)`

GetOffsetOk returns a tuple with the Offset field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOffset

`func (o *MultiResourceOutputOptions) SetOffset(v int32)`

SetOffset sets Offset field to given value.


### GetOnlyDeleted

`func (o *MultiResourceOutputOptions) GetOnlyDeleted() bool`

GetOnlyDeleted returns the OnlyDeleted field if non-nil, zero value otherwise.

### GetOnlyDeletedOk

`func (o *MultiResourceOutputOptions) GetOnlyDeletedOk() (*bool, bool)`

GetOnlyDeletedOk returns a tuple with the OnlyDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOnlyDeleted

`func (o *MultiResourceOutputOptions) SetOnlyDeleted(v bool)`

SetOnlyDeleted sets OnlyDeleted field to given value.


### GetSingle

`func (o *MultiResourceOutputOptions) GetSingle() SingleResourceOutputOptions`

GetSingle returns the Single field if non-nil, zero value otherwise.

### GetSingleOk

`func (o *MultiResourceOutputOptions) GetSingleOk() (*SingleResourceOutputOptions, bool)`

GetSingleOk returns a tuple with the Single field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSingle

`func (o *MultiResourceOutputOptions) SetSingle(v SingleResourceOutputOptions)`

SetSingle sets Single field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
