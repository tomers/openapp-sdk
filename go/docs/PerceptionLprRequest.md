# PerceptionLprRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**EntityId** | **string** |  |
**Evidence** | Pointer to **interface{}** |  | [optional]
**Plate** | **string** |  |

## Methods

### NewPerceptionLprRequest

`func NewPerceptionLprRequest(entityId string, plate string, ) *PerceptionLprRequest`

NewPerceptionLprRequest instantiates a new PerceptionLprRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPerceptionLprRequestWithDefaults

`func NewPerceptionLprRequestWithDefaults() *PerceptionLprRequest`

NewPerceptionLprRequestWithDefaults instantiates a new PerceptionLprRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEntityId

`func (o *PerceptionLprRequest) GetEntityId() string`

GetEntityId returns the EntityId field if non-nil, zero value otherwise.

### GetEntityIdOk

`func (o *PerceptionLprRequest) GetEntityIdOk() (*string, bool)`

GetEntityIdOk returns a tuple with the EntityId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityId

`func (o *PerceptionLprRequest) SetEntityId(v string)`

SetEntityId sets EntityId field to given value.


### GetEvidence

`func (o *PerceptionLprRequest) GetEvidence() interface{}`

GetEvidence returns the Evidence field if non-nil, zero value otherwise.

### GetEvidenceOk

`func (o *PerceptionLprRequest) GetEvidenceOk() (*interface{}, bool)`

GetEvidenceOk returns a tuple with the Evidence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEvidence

`func (o *PerceptionLprRequest) SetEvidence(v interface{})`

SetEvidence sets Evidence field to given value.

### HasEvidence

`func (o *PerceptionLprRequest) HasEvidence() bool`

HasEvidence returns a boolean if a field has been set.

### SetEvidenceNil

`func (o *PerceptionLprRequest) SetEvidenceNil(b bool)`

 SetEvidenceNil sets the value for Evidence to be an explicit nil

### UnsetEvidence
`func (o *PerceptionLprRequest) UnsetEvidence()`

UnsetEvidence ensures that no value is present for Evidence, not even an explicit nil
### GetPlate

`func (o *PerceptionLprRequest) GetPlate() string`

GetPlate returns the Plate field if non-nil, zero value otherwise.

### GetPlateOk

`func (o *PerceptionLprRequest) GetPlateOk() (*string, bool)`

GetPlateOk returns a tuple with the Plate field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPlate

`func (o *PerceptionLprRequest) SetPlate(v string)`

SetPlate sets Plate field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
