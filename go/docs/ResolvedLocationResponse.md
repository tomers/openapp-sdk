# ResolvedLocationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Binding** | [**LocationBindingResponse**](LocationBindingResponse.md) |  |
**Resolved** | Pointer to [**NullableLocationResponse**](LocationResponse.md) |  | [optional]

## Methods

### NewResolvedLocationResponse

`func NewResolvedLocationResponse(binding LocationBindingResponse, ) *ResolvedLocationResponse`

NewResolvedLocationResponse instantiates a new ResolvedLocationResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResolvedLocationResponseWithDefaults

`func NewResolvedLocationResponseWithDefaults() *ResolvedLocationResponse`

NewResolvedLocationResponseWithDefaults instantiates a new ResolvedLocationResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBinding

`func (o *ResolvedLocationResponse) GetBinding() LocationBindingResponse`

GetBinding returns the Binding field if non-nil, zero value otherwise.

### GetBindingOk

`func (o *ResolvedLocationResponse) GetBindingOk() (*LocationBindingResponse, bool)`

GetBindingOk returns a tuple with the Binding field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBinding

`func (o *ResolvedLocationResponse) SetBinding(v LocationBindingResponse)`

SetBinding sets Binding field to given value.


### GetResolved

`func (o *ResolvedLocationResponse) GetResolved() LocationResponse`

GetResolved returns the Resolved field if non-nil, zero value otherwise.

### GetResolvedOk

`func (o *ResolvedLocationResponse) GetResolvedOk() (*LocationResponse, bool)`

GetResolvedOk returns a tuple with the Resolved field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResolved

`func (o *ResolvedLocationResponse) SetResolved(v LocationResponse)`

SetResolved sets Resolved field to given value.

### HasResolved

`func (o *ResolvedLocationResponse) HasResolved() bool`

HasResolved returns a boolean if a field has been set.

### SetResolvedNil

`func (o *ResolvedLocationResponse) SetResolvedNil(b bool)`

 SetResolvedNil sets the value for Resolved to be an explicit nil

### UnsetResolved
`func (o *ResolvedLocationResponse) UnsetResolved()`

UnsetResolved ensures that no value is present for Resolved, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
