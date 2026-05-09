# ListOrgUsersQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IncludeDeleted** | **bool** |  |
**IncludeMetadata** | **bool** |  |
**OnlyDeleted** | **bool** |  |
**Limit** | Pointer to **int32** | Number of items per page. Default from config, max 200. | [optional]
**Offset** | Pointer to **int32** | Number of items to skip. Default 0. | [optional]
**Recursive** | Pointer to **NullableBool** |  | [optional]

## Methods

### NewListOrgUsersQuery

`func NewListOrgUsersQuery(includeDeleted bool, includeMetadata bool, onlyDeleted bool, ) *ListOrgUsersQuery`

NewListOrgUsersQuery instantiates a new ListOrgUsersQuery object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListOrgUsersQueryWithDefaults

`func NewListOrgUsersQueryWithDefaults() *ListOrgUsersQuery`

NewListOrgUsersQueryWithDefaults instantiates a new ListOrgUsersQuery object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIncludeDeleted

`func (o *ListOrgUsersQuery) GetIncludeDeleted() bool`

GetIncludeDeleted returns the IncludeDeleted field if non-nil, zero value otherwise.

### GetIncludeDeletedOk

`func (o *ListOrgUsersQuery) GetIncludeDeletedOk() (*bool, bool)`

GetIncludeDeletedOk returns a tuple with the IncludeDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeDeleted

`func (o *ListOrgUsersQuery) SetIncludeDeleted(v bool)`

SetIncludeDeleted sets IncludeDeleted field to given value.


### GetIncludeMetadata

`func (o *ListOrgUsersQuery) GetIncludeMetadata() bool`

GetIncludeMetadata returns the IncludeMetadata field if non-nil, zero value otherwise.

### GetIncludeMetadataOk

`func (o *ListOrgUsersQuery) GetIncludeMetadataOk() (*bool, bool)`

GetIncludeMetadataOk returns a tuple with the IncludeMetadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeMetadata

`func (o *ListOrgUsersQuery) SetIncludeMetadata(v bool)`

SetIncludeMetadata sets IncludeMetadata field to given value.


### GetOnlyDeleted

`func (o *ListOrgUsersQuery) GetOnlyDeleted() bool`

GetOnlyDeleted returns the OnlyDeleted field if non-nil, zero value otherwise.

### GetOnlyDeletedOk

`func (o *ListOrgUsersQuery) GetOnlyDeletedOk() (*bool, bool)`

GetOnlyDeletedOk returns a tuple with the OnlyDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOnlyDeleted

`func (o *ListOrgUsersQuery) SetOnlyDeleted(v bool)`

SetOnlyDeleted sets OnlyDeleted field to given value.


### GetLimit

`func (o *ListOrgUsersQuery) GetLimit() int32`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *ListOrgUsersQuery) GetLimitOk() (*int32, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *ListOrgUsersQuery) SetLimit(v int32)`

SetLimit sets Limit field to given value.

### HasLimit

`func (o *ListOrgUsersQuery) HasLimit() bool`

HasLimit returns a boolean if a field has been set.

### GetOffset

`func (o *ListOrgUsersQuery) GetOffset() int32`

GetOffset returns the Offset field if non-nil, zero value otherwise.

### GetOffsetOk

`func (o *ListOrgUsersQuery) GetOffsetOk() (*int32, bool)`

GetOffsetOk returns a tuple with the Offset field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOffset

`func (o *ListOrgUsersQuery) SetOffset(v int32)`

SetOffset sets Offset field to given value.

### HasOffset

`func (o *ListOrgUsersQuery) HasOffset() bool`

HasOffset returns a boolean if a field has been set.

### GetRecursive

`func (o *ListOrgUsersQuery) GetRecursive() bool`

GetRecursive returns the Recursive field if non-nil, zero value otherwise.

### GetRecursiveOk

`func (o *ListOrgUsersQuery) GetRecursiveOk() (*bool, bool)`

GetRecursiveOk returns a tuple with the Recursive field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecursive

`func (o *ListOrgUsersQuery) SetRecursive(v bool)`

SetRecursive sets Recursive field to given value.

### HasRecursive

`func (o *ListOrgUsersQuery) HasRecursive() bool`

HasRecursive returns a boolean if a field has been set.

### SetRecursiveNil

`func (o *ListOrgUsersQuery) SetRecursiveNil(b bool)`

 SetRecursiveNil sets the value for Recursive to be an explicit nil

### UnsetRecursive
`func (o *ListOrgUsersQuery) UnsetRecursive()`

UnsetRecursive ensures that no value is present for Recursive, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
