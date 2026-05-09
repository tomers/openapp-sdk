# ApartmentFloorSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Floor** | **interface{}** |  |
**FloorNumber** | Pointer to **NullableInt64** |  | [optional]
**Key** | **string** |  |

## Methods

### NewApartmentFloorSummary

`func NewApartmentFloorSummary(floor interface{}, key string, ) *ApartmentFloorSummary`

NewApartmentFloorSummary instantiates a new ApartmentFloorSummary object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApartmentFloorSummaryWithDefaults

`func NewApartmentFloorSummaryWithDefaults() *ApartmentFloorSummary`

NewApartmentFloorSummaryWithDefaults instantiates a new ApartmentFloorSummary object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetFloor

`func (o *ApartmentFloorSummary) GetFloor() interface{}`

GetFloor returns the Floor field if non-nil, zero value otherwise.

### GetFloorOk

`func (o *ApartmentFloorSummary) GetFloorOk() (*interface{}, bool)`

GetFloorOk returns a tuple with the Floor field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFloor

`func (o *ApartmentFloorSummary) SetFloor(v interface{})`

SetFloor sets Floor field to given value.


### SetFloorNil

`func (o *ApartmentFloorSummary) SetFloorNil(b bool)`

 SetFloorNil sets the value for Floor to be an explicit nil

### UnsetFloor
`func (o *ApartmentFloorSummary) UnsetFloor()`

UnsetFloor ensures that no value is present for Floor, not even an explicit nil
### GetFloorNumber

`func (o *ApartmentFloorSummary) GetFloorNumber() int64`

GetFloorNumber returns the FloorNumber field if non-nil, zero value otherwise.

### GetFloorNumberOk

`func (o *ApartmentFloorSummary) GetFloorNumberOk() (*int64, bool)`

GetFloorNumberOk returns a tuple with the FloorNumber field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFloorNumber

`func (o *ApartmentFloorSummary) SetFloorNumber(v int64)`

SetFloorNumber sets FloorNumber field to given value.

### HasFloorNumber

`func (o *ApartmentFloorSummary) HasFloorNumber() bool`

HasFloorNumber returns a boolean if a field has been set.

### SetFloorNumberNil

`func (o *ApartmentFloorSummary) SetFloorNumberNil(b bool)`

 SetFloorNumberNil sets the value for FloorNumber to be an explicit nil

### UnsetFloorNumber
`func (o *ApartmentFloorSummary) UnsetFloorNumber()`

UnsetFloorNumber ensures that no value is present for FloorNumber, not even an explicit nil
### GetKey

`func (o *ApartmentFloorSummary) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *ApartmentFloorSummary) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *ApartmentFloorSummary) SetKey(v string)`

SetKey sets Key field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
