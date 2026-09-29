# PutLocationBindingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**LocationId** | Pointer to **NullableString** |  | [optional]
**Mode** | [**LocationBindingMode**](LocationBindingMode.md) |  |

## Methods

### NewPutLocationBindingRequest

`func NewPutLocationBindingRequest(mode LocationBindingMode, ) *PutLocationBindingRequest`

NewPutLocationBindingRequest instantiates a new PutLocationBindingRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPutLocationBindingRequestWithDefaults

`func NewPutLocationBindingRequestWithDefaults() *PutLocationBindingRequest`

NewPutLocationBindingRequestWithDefaults instantiates a new PutLocationBindingRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLocationId

`func (o *PutLocationBindingRequest) GetLocationId() string`

GetLocationId returns the LocationId field if non-nil, zero value otherwise.

### GetLocationIdOk

`func (o *PutLocationBindingRequest) GetLocationIdOk() (*string, bool)`

GetLocationIdOk returns a tuple with the LocationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocationId

`func (o *PutLocationBindingRequest) SetLocationId(v string)`

SetLocationId sets LocationId field to given value.

### HasLocationId

`func (o *PutLocationBindingRequest) HasLocationId() bool`

HasLocationId returns a boolean if a field has been set.

### SetLocationIdNil

`func (o *PutLocationBindingRequest) SetLocationIdNil(b bool)`

 SetLocationIdNil sets the value for LocationId to be an explicit nil

### UnsetLocationId
`func (o *PutLocationBindingRequest) UnsetLocationId()`

UnsetLocationId ensures that no value is present for LocationId, not even an explicit nil
### GetMode

`func (o *PutLocationBindingRequest) GetMode() LocationBindingMode`

GetMode returns the Mode field if non-nil, zero value otherwise.

### GetModeOk

`func (o *PutLocationBindingRequest) GetModeOk() (*LocationBindingMode, bool)`

GetModeOk returns a tuple with the Mode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMode

`func (o *PutLocationBindingRequest) SetMode(v LocationBindingMode)`

SetMode sets Mode field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
