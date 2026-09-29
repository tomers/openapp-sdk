# ListOrgChildrenQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IncludeDeleted** | Pointer to **bool** |  | [optional]
**OnlyDeleted** | Pointer to **bool** |  | [optional]
**Limit** | Pointer to **int32** | Number of items per page. Default from config, max 200. | [optional]
**Offset** | Pointer to **int32** | Number of items to skip. Default 0. | [optional]
**ParentId** | Pointer to **NullableString** | Parent org id. When omitted, returns the roots of the caller&#39;s visible org forest. | [optional]

## Methods

### NewListOrgChildrenQuery

`func NewListOrgChildrenQuery() *ListOrgChildrenQuery`

NewListOrgChildrenQuery instantiates a new ListOrgChildrenQuery object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListOrgChildrenQueryWithDefaults

`func NewListOrgChildrenQueryWithDefaults() *ListOrgChildrenQuery`

NewListOrgChildrenQueryWithDefaults instantiates a new ListOrgChildrenQuery object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIncludeDeleted

`func (o *ListOrgChildrenQuery) GetIncludeDeleted() bool`

GetIncludeDeleted returns the IncludeDeleted field if non-nil, zero value otherwise.

### GetIncludeDeletedOk

`func (o *ListOrgChildrenQuery) GetIncludeDeletedOk() (*bool, bool)`

GetIncludeDeletedOk returns a tuple with the IncludeDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeDeleted

`func (o *ListOrgChildrenQuery) SetIncludeDeleted(v bool)`

SetIncludeDeleted sets IncludeDeleted field to given value.

### HasIncludeDeleted

`func (o *ListOrgChildrenQuery) HasIncludeDeleted() bool`

HasIncludeDeleted returns a boolean if a field has been set.

### GetOnlyDeleted

`func (o *ListOrgChildrenQuery) GetOnlyDeleted() bool`

GetOnlyDeleted returns the OnlyDeleted field if non-nil, zero value otherwise.

### GetOnlyDeletedOk

`func (o *ListOrgChildrenQuery) GetOnlyDeletedOk() (*bool, bool)`

GetOnlyDeletedOk returns a tuple with the OnlyDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOnlyDeleted

`func (o *ListOrgChildrenQuery) SetOnlyDeleted(v bool)`

SetOnlyDeleted sets OnlyDeleted field to given value.

### HasOnlyDeleted

`func (o *ListOrgChildrenQuery) HasOnlyDeleted() bool`

HasOnlyDeleted returns a boolean if a field has been set.

### GetLimit

`func (o *ListOrgChildrenQuery) GetLimit() int32`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *ListOrgChildrenQuery) GetLimitOk() (*int32, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *ListOrgChildrenQuery) SetLimit(v int32)`

SetLimit sets Limit field to given value.

### HasLimit

`func (o *ListOrgChildrenQuery) HasLimit() bool`

HasLimit returns a boolean if a field has been set.

### GetOffset

`func (o *ListOrgChildrenQuery) GetOffset() int32`

GetOffset returns the Offset field if non-nil, zero value otherwise.

### GetOffsetOk

`func (o *ListOrgChildrenQuery) GetOffsetOk() (*int32, bool)`

GetOffsetOk returns a tuple with the Offset field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOffset

`func (o *ListOrgChildrenQuery) SetOffset(v int32)`

SetOffset sets Offset field to given value.

### HasOffset

`func (o *ListOrgChildrenQuery) HasOffset() bool`

HasOffset returns a boolean if a field has been set.

### GetParentId

`func (o *ListOrgChildrenQuery) GetParentId() string`

GetParentId returns the ParentId field if non-nil, zero value otherwise.

### GetParentIdOk

`func (o *ListOrgChildrenQuery) GetParentIdOk() (*string, bool)`

GetParentIdOk returns a tuple with the ParentId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetParentId

`func (o *ListOrgChildrenQuery) SetParentId(v string)`

SetParentId sets ParentId field to given value.

### HasParentId

`func (o *ListOrgChildrenQuery) HasParentId() bool`

HasParentId returns a boolean if a field has been set.

### SetParentIdNil

`func (o *ListOrgChildrenQuery) SetParentIdNil(b bool)`

 SetParentIdNil sets the value for ParentId to be an explicit nil

### UnsetParentId
`func (o *ListOrgChildrenQuery) UnsetParentId()`

UnsetParentId ensures that no value is present for ParentId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
