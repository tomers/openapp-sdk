# ListOrgsQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IncludeDeleted** | **bool** |  |
**IncludeMetadata** | **bool** |  |
**OnlyDeleted** | **bool** |  |
**Limit** | Pointer to **int32** | Number of items per page. Default from config, max 200. | [optional]
**Offset** | Pointer to **int32** | Number of items to skip. Default 0. | [optional]

## Methods

### NewListOrgsQuery

`func NewListOrgsQuery(includeDeleted bool, includeMetadata bool, onlyDeleted bool, ) *ListOrgsQuery`

NewListOrgsQuery instantiates a new ListOrgsQuery object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListOrgsQueryWithDefaults

`func NewListOrgsQueryWithDefaults() *ListOrgsQuery`

NewListOrgsQueryWithDefaults instantiates a new ListOrgsQuery object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIncludeDeleted

`func (o *ListOrgsQuery) GetIncludeDeleted() bool`

GetIncludeDeleted returns the IncludeDeleted field if non-nil, zero value otherwise.

### GetIncludeDeletedOk

`func (o *ListOrgsQuery) GetIncludeDeletedOk() (*bool, bool)`

GetIncludeDeletedOk returns a tuple with the IncludeDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeDeleted

`func (o *ListOrgsQuery) SetIncludeDeleted(v bool)`

SetIncludeDeleted sets IncludeDeleted field to given value.


### GetIncludeMetadata

`func (o *ListOrgsQuery) GetIncludeMetadata() bool`

GetIncludeMetadata returns the IncludeMetadata field if non-nil, zero value otherwise.

### GetIncludeMetadataOk

`func (o *ListOrgsQuery) GetIncludeMetadataOk() (*bool, bool)`

GetIncludeMetadataOk returns a tuple with the IncludeMetadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeMetadata

`func (o *ListOrgsQuery) SetIncludeMetadata(v bool)`

SetIncludeMetadata sets IncludeMetadata field to given value.


### GetOnlyDeleted

`func (o *ListOrgsQuery) GetOnlyDeleted() bool`

GetOnlyDeleted returns the OnlyDeleted field if non-nil, zero value otherwise.

### GetOnlyDeletedOk

`func (o *ListOrgsQuery) GetOnlyDeletedOk() (*bool, bool)`

GetOnlyDeletedOk returns a tuple with the OnlyDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOnlyDeleted

`func (o *ListOrgsQuery) SetOnlyDeleted(v bool)`

SetOnlyDeleted sets OnlyDeleted field to given value.


### GetLimit

`func (o *ListOrgsQuery) GetLimit() int32`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *ListOrgsQuery) GetLimitOk() (*int32, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *ListOrgsQuery) SetLimit(v int32)`

SetLimit sets Limit field to given value.

### HasLimit

`func (o *ListOrgsQuery) HasLimit() bool`

HasLimit returns a boolean if a field has been set.

### GetOffset

`func (o *ListOrgsQuery) GetOffset() int32`

GetOffset returns the Offset field if non-nil, zero value otherwise.

### GetOffsetOk

`func (o *ListOrgsQuery) GetOffsetOk() (*int32, bool)`

GetOffsetOk returns a tuple with the Offset field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOffset

`func (o *ListOrgsQuery) SetOffset(v int32)`

SetOffset sets Offset field to given value.

### HasOffset

`func (o *ListOrgsQuery) HasOffset() bool`

HasOffset returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
