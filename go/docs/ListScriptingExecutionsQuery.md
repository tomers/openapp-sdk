# ListScriptingExecutionsQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Limit** | Pointer to **NullableInt64** | Maximum number of executions to return (1-100, default 20). | [optional]

## Methods

### NewListScriptingExecutionsQuery

`func NewListScriptingExecutionsQuery() *ListScriptingExecutionsQuery`

NewListScriptingExecutionsQuery instantiates a new ListScriptingExecutionsQuery object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListScriptingExecutionsQueryWithDefaults

`func NewListScriptingExecutionsQueryWithDefaults() *ListScriptingExecutionsQuery`

NewListScriptingExecutionsQueryWithDefaults instantiates a new ListScriptingExecutionsQuery object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLimit

`func (o *ListScriptingExecutionsQuery) GetLimit() int64`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *ListScriptingExecutionsQuery) GetLimitOk() (*int64, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *ListScriptingExecutionsQuery) SetLimit(v int64)`

SetLimit sets Limit field to given value.

### HasLimit

`func (o *ListScriptingExecutionsQuery) HasLimit() bool`

HasLimit returns a boolean if a field has been set.

### SetLimitNil

`func (o *ListScriptingExecutionsQuery) SetLimitNil(b bool)`

 SetLimitNil sets the value for Limit to be an explicit nil

### UnsetLimit
`func (o *ListScriptingExecutionsQuery) UnsetLimit()`

UnsetLimit ensures that no value is present for Limit, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
