# ListEntitiesQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IncludeDeleted** | Pointer to **bool** |  | [optional]
**OnlyDeleted** | Pointer to **bool** |  | [optional]
**Limit** | Pointer to **int32** | Number of items per page. Default from config, max 200. | [optional]
**Offset** | Pointer to **int32** | Number of items to skip. Default 0. | [optional]
**Q** | Pointer to **NullableString** | Case-insensitive substring match on entity &#x60;name&#x60; (plain text). Applies to org-wide lists. | [optional]
**Sort** | Pointer to **NullableString** | Server-side ordering for org-wide lists: &#x60;name:asc&#x60;, &#x60;name:desc&#x60;, &#x60;created_at:asc&#x60;, &#x60;created_at:desc&#x60;. Ignored when &#x60;zone_id&#x60; is set. Defaults to insertion order when omitted. | [optional]
**ZoneId** | Pointer to **NullableString** | Optional filter: only entities in this zone. | [optional]

## Methods

### NewListEntitiesQuery

`func NewListEntitiesQuery() *ListEntitiesQuery`

NewListEntitiesQuery instantiates a new ListEntitiesQuery object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListEntitiesQueryWithDefaults

`func NewListEntitiesQueryWithDefaults() *ListEntitiesQuery`

NewListEntitiesQueryWithDefaults instantiates a new ListEntitiesQuery object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIncludeDeleted

`func (o *ListEntitiesQuery) GetIncludeDeleted() bool`

GetIncludeDeleted returns the IncludeDeleted field if non-nil, zero value otherwise.

### GetIncludeDeletedOk

`func (o *ListEntitiesQuery) GetIncludeDeletedOk() (*bool, bool)`

GetIncludeDeletedOk returns a tuple with the IncludeDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeDeleted

`func (o *ListEntitiesQuery) SetIncludeDeleted(v bool)`

SetIncludeDeleted sets IncludeDeleted field to given value.

### HasIncludeDeleted

`func (o *ListEntitiesQuery) HasIncludeDeleted() bool`

HasIncludeDeleted returns a boolean if a field has been set.

### GetOnlyDeleted

`func (o *ListEntitiesQuery) GetOnlyDeleted() bool`

GetOnlyDeleted returns the OnlyDeleted field if non-nil, zero value otherwise.

### GetOnlyDeletedOk

`func (o *ListEntitiesQuery) GetOnlyDeletedOk() (*bool, bool)`

GetOnlyDeletedOk returns a tuple with the OnlyDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOnlyDeleted

`func (o *ListEntitiesQuery) SetOnlyDeleted(v bool)`

SetOnlyDeleted sets OnlyDeleted field to given value.

### HasOnlyDeleted

`func (o *ListEntitiesQuery) HasOnlyDeleted() bool`

HasOnlyDeleted returns a boolean if a field has been set.

### GetLimit

`func (o *ListEntitiesQuery) GetLimit() int32`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *ListEntitiesQuery) GetLimitOk() (*int32, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *ListEntitiesQuery) SetLimit(v int32)`

SetLimit sets Limit field to given value.

### HasLimit

`func (o *ListEntitiesQuery) HasLimit() bool`

HasLimit returns a boolean if a field has been set.

### GetOffset

`func (o *ListEntitiesQuery) GetOffset() int32`

GetOffset returns the Offset field if non-nil, zero value otherwise.

### GetOffsetOk

`func (o *ListEntitiesQuery) GetOffsetOk() (*int32, bool)`

GetOffsetOk returns a tuple with the Offset field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOffset

`func (o *ListEntitiesQuery) SetOffset(v int32)`

SetOffset sets Offset field to given value.

### HasOffset

`func (o *ListEntitiesQuery) HasOffset() bool`

HasOffset returns a boolean if a field has been set.

### GetQ

`func (o *ListEntitiesQuery) GetQ() string`

GetQ returns the Q field if non-nil, zero value otherwise.

### GetQOk

`func (o *ListEntitiesQuery) GetQOk() (*string, bool)`

GetQOk returns a tuple with the Q field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQ

`func (o *ListEntitiesQuery) SetQ(v string)`

SetQ sets Q field to given value.

### HasQ

`func (o *ListEntitiesQuery) HasQ() bool`

HasQ returns a boolean if a field has been set.

### SetQNil

`func (o *ListEntitiesQuery) SetQNil(b bool)`

 SetQNil sets the value for Q to be an explicit nil

### UnsetQ
`func (o *ListEntitiesQuery) UnsetQ()`

UnsetQ ensures that no value is present for Q, not even an explicit nil
### GetSort

`func (o *ListEntitiesQuery) GetSort() string`

GetSort returns the Sort field if non-nil, zero value otherwise.

### GetSortOk

`func (o *ListEntitiesQuery) GetSortOk() (*string, bool)`

GetSortOk returns a tuple with the Sort field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSort

`func (o *ListEntitiesQuery) SetSort(v string)`

SetSort sets Sort field to given value.

### HasSort

`func (o *ListEntitiesQuery) HasSort() bool`

HasSort returns a boolean if a field has been set.

### SetSortNil

`func (o *ListEntitiesQuery) SetSortNil(b bool)`

 SetSortNil sets the value for Sort to be an explicit nil

### UnsetSort
`func (o *ListEntitiesQuery) UnsetSort()`

UnsetSort ensures that no value is present for Sort, not even an explicit nil
### GetZoneId

`func (o *ListEntitiesQuery) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *ListEntitiesQuery) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *ListEntitiesQuery) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.

### HasZoneId

`func (o *ListEntitiesQuery) HasZoneId() bool`

HasZoneId returns a boolean if a field has been set.

### SetZoneIdNil

`func (o *ListEntitiesQuery) SetZoneIdNil(b bool)`

 SetZoneIdNil sets the value for ZoneId to be an explicit nil

### UnsetZoneId
`func (o *ListEntitiesQuery) UnsetZoneId()`

UnsetZoneId ensures that no value is present for ZoneId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
