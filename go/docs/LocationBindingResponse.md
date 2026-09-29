# LocationBindingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**LocationId** | Pointer to **NullableString** |  | [optional]
**Mode** | [**LocationBindingMode**](LocationBindingMode.md) |  |

## Methods

### NewLocationBindingResponse

`func NewLocationBindingResponse(mode LocationBindingMode, ) *LocationBindingResponse`

NewLocationBindingResponse instantiates a new LocationBindingResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewLocationBindingResponseWithDefaults

`func NewLocationBindingResponseWithDefaults() *LocationBindingResponse`

NewLocationBindingResponseWithDefaults instantiates a new LocationBindingResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLocationId

`func (o *LocationBindingResponse) GetLocationId() string`

GetLocationId returns the LocationId field if non-nil, zero value otherwise.

### GetLocationIdOk

`func (o *LocationBindingResponse) GetLocationIdOk() (*string, bool)`

GetLocationIdOk returns a tuple with the LocationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocationId

`func (o *LocationBindingResponse) SetLocationId(v string)`

SetLocationId sets LocationId field to given value.

### HasLocationId

`func (o *LocationBindingResponse) HasLocationId() bool`

HasLocationId returns a boolean if a field has been set.

### SetLocationIdNil

`func (o *LocationBindingResponse) SetLocationIdNil(b bool)`

 SetLocationIdNil sets the value for LocationId to be an explicit nil

### UnsetLocationId
`func (o *LocationBindingResponse) UnsetLocationId()`

UnsetLocationId ensures that no value is present for LocationId, not even an explicit nil
### GetMode

`func (o *LocationBindingResponse) GetMode() LocationBindingMode`

GetMode returns the Mode field if non-nil, zero value otherwise.

### GetModeOk

`func (o *LocationBindingResponse) GetModeOk() (*LocationBindingMode, bool)`

GetModeOk returns a tuple with the Mode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMode

`func (o *LocationBindingResponse) SetMode(v LocationBindingMode)`

SetMode sets Mode field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
