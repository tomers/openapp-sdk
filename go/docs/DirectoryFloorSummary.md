# DirectoryFloorSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Floor** | **interface{}** |  |
**FloorNumber** | Pointer to **NullableInt64** |  | [optional]
**Key** | **string** |  |

## Methods

### NewDirectoryFloorSummary

`func NewDirectoryFloorSummary(floor interface{}, key string, ) *DirectoryFloorSummary`

NewDirectoryFloorSummary instantiates a new DirectoryFloorSummary object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDirectoryFloorSummaryWithDefaults

`func NewDirectoryFloorSummaryWithDefaults() *DirectoryFloorSummary`

NewDirectoryFloorSummaryWithDefaults instantiates a new DirectoryFloorSummary object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetFloor

`func (o *DirectoryFloorSummary) GetFloor() interface{}`

GetFloor returns the Floor field if non-nil, zero value otherwise.

### GetFloorOk

`func (o *DirectoryFloorSummary) GetFloorOk() (*interface{}, bool)`

GetFloorOk returns a tuple with the Floor field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFloor

`func (o *DirectoryFloorSummary) SetFloor(v interface{})`

SetFloor sets Floor field to given value.


### SetFloorNil

`func (o *DirectoryFloorSummary) SetFloorNil(b bool)`

 SetFloorNil sets the value for Floor to be an explicit nil

### UnsetFloor
`func (o *DirectoryFloorSummary) UnsetFloor()`

UnsetFloor ensures that no value is present for Floor, not even an explicit nil
### GetFloorNumber

`func (o *DirectoryFloorSummary) GetFloorNumber() int64`

GetFloorNumber returns the FloorNumber field if non-nil, zero value otherwise.

### GetFloorNumberOk

`func (o *DirectoryFloorSummary) GetFloorNumberOk() (*int64, bool)`

GetFloorNumberOk returns a tuple with the FloorNumber field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFloorNumber

`func (o *DirectoryFloorSummary) SetFloorNumber(v int64)`

SetFloorNumber sets FloorNumber field to given value.

### HasFloorNumber

`func (o *DirectoryFloorSummary) HasFloorNumber() bool`

HasFloorNumber returns a boolean if a field has been set.

### SetFloorNumberNil

`func (o *DirectoryFloorSummary) SetFloorNumberNil(b bool)`

 SetFloorNumberNil sets the value for FloorNumber to be an explicit nil

### UnsetFloorNumber
`func (o *DirectoryFloorSummary) UnsetFloorNumber()`

UnsetFloorNumber ensures that no value is present for FloorNumber, not even an explicit nil
### GetKey

`func (o *DirectoryFloorSummary) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *DirectoryFloorSummary) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *DirectoryFloorSummary) SetKey(v string)`

SetKey sets Key field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
