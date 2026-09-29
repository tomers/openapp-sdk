# PerceptionVisitorRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**EntityId** | **string** |  |
**Evidence** | Pointer to **interface{}** |  | [optional]
**PortalId** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPerceptionVisitorRequest

`func NewPerceptionVisitorRequest(entityId string, ) *PerceptionVisitorRequest`

NewPerceptionVisitorRequest instantiates a new PerceptionVisitorRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPerceptionVisitorRequestWithDefaults

`func NewPerceptionVisitorRequestWithDefaults() *PerceptionVisitorRequest`

NewPerceptionVisitorRequestWithDefaults instantiates a new PerceptionVisitorRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEntityId

`func (o *PerceptionVisitorRequest) GetEntityId() string`

GetEntityId returns the EntityId field if non-nil, zero value otherwise.

### GetEntityIdOk

`func (o *PerceptionVisitorRequest) GetEntityIdOk() (*string, bool)`

GetEntityIdOk returns a tuple with the EntityId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityId

`func (o *PerceptionVisitorRequest) SetEntityId(v string)`

SetEntityId sets EntityId field to given value.


### GetEvidence

`func (o *PerceptionVisitorRequest) GetEvidence() interface{}`

GetEvidence returns the Evidence field if non-nil, zero value otherwise.

### GetEvidenceOk

`func (o *PerceptionVisitorRequest) GetEvidenceOk() (*interface{}, bool)`

GetEvidenceOk returns a tuple with the Evidence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEvidence

`func (o *PerceptionVisitorRequest) SetEvidence(v interface{})`

SetEvidence sets Evidence field to given value.

### HasEvidence

`func (o *PerceptionVisitorRequest) HasEvidence() bool`

HasEvidence returns a boolean if a field has been set.

### SetEvidenceNil

`func (o *PerceptionVisitorRequest) SetEvidenceNil(b bool)`

 SetEvidenceNil sets the value for Evidence to be an explicit nil

### UnsetEvidence
`func (o *PerceptionVisitorRequest) UnsetEvidence()`

UnsetEvidence ensures that no value is present for Evidence, not even an explicit nil
### GetPortalId

`func (o *PerceptionVisitorRequest) GetPortalId() string`

GetPortalId returns the PortalId field if non-nil, zero value otherwise.

### GetPortalIdOk

`func (o *PerceptionVisitorRequest) GetPortalIdOk() (*string, bool)`

GetPortalIdOk returns a tuple with the PortalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPortalId

`func (o *PerceptionVisitorRequest) SetPortalId(v string)`

SetPortalId sets PortalId field to given value.

### HasPortalId

`func (o *PerceptionVisitorRequest) HasPortalId() bool`

HasPortalId returns a boolean if a field has been set.

### SetPortalIdNil

`func (o *PerceptionVisitorRequest) SetPortalIdNil(b bool)`

 SetPortalIdNil sets the value for PortalId to be an explicit nil

### UnsetPortalId
`func (o *PerceptionVisitorRequest) UnsetPortalId()`

UnsetPortalId ensures that no value is present for PortalId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
