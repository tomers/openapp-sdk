# SearchUsersQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExcludeIds** | Pointer to **NullableString** | Comma-separated user IDs to exclude | [optional]
**Limit** | Pointer to **int32** |  | [optional]
**Offset** | Pointer to **int32** |  | [optional]
**OrgId** | Pointer to **NullableString** | Filter by org subtree (ignored when &#x60;scope&#x3D;all&#x60;). | [optional]
**Q** | Pointer to **NullableString** | Search text (ILIKE on name and email) | [optional]
**Scope** | Pointer to **NullableString** | &#x60;all&#x60;: search every user in the database (requires &#x60;users:list&#x60; on the root org). Otherwise org-scoped. | [optional]

## Methods

### NewSearchUsersQuery

`func NewSearchUsersQuery() *SearchUsersQuery`

NewSearchUsersQuery instantiates a new SearchUsersQuery object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSearchUsersQueryWithDefaults

`func NewSearchUsersQueryWithDefaults() *SearchUsersQuery`

NewSearchUsersQueryWithDefaults instantiates a new SearchUsersQuery object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExcludeIds

`func (o *SearchUsersQuery) GetExcludeIds() string`

GetExcludeIds returns the ExcludeIds field if non-nil, zero value otherwise.

### GetExcludeIdsOk

`func (o *SearchUsersQuery) GetExcludeIdsOk() (*string, bool)`

GetExcludeIdsOk returns a tuple with the ExcludeIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExcludeIds

`func (o *SearchUsersQuery) SetExcludeIds(v string)`

SetExcludeIds sets ExcludeIds field to given value.

### HasExcludeIds

`func (o *SearchUsersQuery) HasExcludeIds() bool`

HasExcludeIds returns a boolean if a field has been set.

### SetExcludeIdsNil

`func (o *SearchUsersQuery) SetExcludeIdsNil(b bool)`

 SetExcludeIdsNil sets the value for ExcludeIds to be an explicit nil

### UnsetExcludeIds
`func (o *SearchUsersQuery) UnsetExcludeIds()`

UnsetExcludeIds ensures that no value is present for ExcludeIds, not even an explicit nil
### GetLimit

`func (o *SearchUsersQuery) GetLimit() int32`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *SearchUsersQuery) GetLimitOk() (*int32, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *SearchUsersQuery) SetLimit(v int32)`

SetLimit sets Limit field to given value.

### HasLimit

`func (o *SearchUsersQuery) HasLimit() bool`

HasLimit returns a boolean if a field has been set.

### GetOffset

`func (o *SearchUsersQuery) GetOffset() int32`

GetOffset returns the Offset field if non-nil, zero value otherwise.

### GetOffsetOk

`func (o *SearchUsersQuery) GetOffsetOk() (*int32, bool)`

GetOffsetOk returns a tuple with the Offset field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOffset

`func (o *SearchUsersQuery) SetOffset(v int32)`

SetOffset sets Offset field to given value.

### HasOffset

`func (o *SearchUsersQuery) HasOffset() bool`

HasOffset returns a boolean if a field has been set.

### GetOrgId

`func (o *SearchUsersQuery) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *SearchUsersQuery) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *SearchUsersQuery) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.

### HasOrgId

`func (o *SearchUsersQuery) HasOrgId() bool`

HasOrgId returns a boolean if a field has been set.

### SetOrgIdNil

`func (o *SearchUsersQuery) SetOrgIdNil(b bool)`

 SetOrgIdNil sets the value for OrgId to be an explicit nil

### UnsetOrgId
`func (o *SearchUsersQuery) UnsetOrgId()`

UnsetOrgId ensures that no value is present for OrgId, not even an explicit nil
### GetQ

`func (o *SearchUsersQuery) GetQ() string`

GetQ returns the Q field if non-nil, zero value otherwise.

### GetQOk

`func (o *SearchUsersQuery) GetQOk() (*string, bool)`

GetQOk returns a tuple with the Q field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQ

`func (o *SearchUsersQuery) SetQ(v string)`

SetQ sets Q field to given value.

### HasQ

`func (o *SearchUsersQuery) HasQ() bool`

HasQ returns a boolean if a field has been set.

### SetQNil

`func (o *SearchUsersQuery) SetQNil(b bool)`

 SetQNil sets the value for Q to be an explicit nil

### UnsetQ
`func (o *SearchUsersQuery) UnsetQ()`

UnsetQ ensures that no value is present for Q, not even an explicit nil
### GetScope

`func (o *SearchUsersQuery) GetScope() string`

GetScope returns the Scope field if non-nil, zero value otherwise.

### GetScopeOk

`func (o *SearchUsersQuery) GetScopeOk() (*string, bool)`

GetScopeOk returns a tuple with the Scope field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScope

`func (o *SearchUsersQuery) SetScope(v string)`

SetScope sets Scope field to given value.

### HasScope

`func (o *SearchUsersQuery) HasScope() bool`

HasScope returns a boolean if a field has been set.

### SetScopeNil

`func (o *SearchUsersQuery) SetScopeNil(b bool)`

 SetScopeNil sets the value for Scope to be an explicit nil

### UnsetScope
`func (o *SearchUsersQuery) UnsetScope()`

UnsetScope ensures that no value is present for Scope, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
