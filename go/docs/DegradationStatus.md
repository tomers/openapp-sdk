# DegradationStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DegradationLevel** | **float32** |  |
**Flagged** | [**[]FlaggedResource**](FlaggedResource.md) |  |
**OrgId** | **NullableString** |  |
**OverCapacity** | **bool** |  |
**OverCapacitySince** | Pointer to **NullableString** |  | [optional]
**Overages** | [**[]KeyOverage**](KeyOverage.md) |  |
**SelectionPolicy** | [**SelectionPolicy**](SelectionPolicy.md) |  |
**Stage** | [**DegradationStage**](DegradationStage.md) |  |

## Methods

### NewDegradationStatus

`func NewDegradationStatus(degradationLevel float32, flagged []FlaggedResource, orgId NullableString, overCapacity bool, overages []KeyOverage, selectionPolicy SelectionPolicy, stage DegradationStage, ) *DegradationStatus`

NewDegradationStatus instantiates a new DegradationStatus object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDegradationStatusWithDefaults

`func NewDegradationStatusWithDefaults() *DegradationStatus`

NewDegradationStatusWithDefaults instantiates a new DegradationStatus object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDegradationLevel

`func (o *DegradationStatus) GetDegradationLevel() float32`

GetDegradationLevel returns the DegradationLevel field if non-nil, zero value otherwise.

### GetDegradationLevelOk

`func (o *DegradationStatus) GetDegradationLevelOk() (*float32, bool)`

GetDegradationLevelOk returns a tuple with the DegradationLevel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDegradationLevel

`func (o *DegradationStatus) SetDegradationLevel(v float32)`

SetDegradationLevel sets DegradationLevel field to given value.


### GetFlagged

`func (o *DegradationStatus) GetFlagged() []FlaggedResource`

GetFlagged returns the Flagged field if non-nil, zero value otherwise.

### GetFlaggedOk

`func (o *DegradationStatus) GetFlaggedOk() (*[]FlaggedResource, bool)`

GetFlaggedOk returns a tuple with the Flagged field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFlagged

`func (o *DegradationStatus) SetFlagged(v []FlaggedResource)`

SetFlagged sets Flagged field to given value.


### GetOrgId

`func (o *DegradationStatus) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *DegradationStatus) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *DegradationStatus) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### SetOrgIdNil

`func (o *DegradationStatus) SetOrgIdNil(b bool)`

 SetOrgIdNil sets the value for OrgId to be an explicit nil

### UnsetOrgId
`func (o *DegradationStatus) UnsetOrgId()`

UnsetOrgId ensures that no value is present for OrgId, not even an explicit nil
### GetOverCapacity

`func (o *DegradationStatus) GetOverCapacity() bool`

GetOverCapacity returns the OverCapacity field if non-nil, zero value otherwise.

### GetOverCapacityOk

`func (o *DegradationStatus) GetOverCapacityOk() (*bool, bool)`

GetOverCapacityOk returns a tuple with the OverCapacity field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOverCapacity

`func (o *DegradationStatus) SetOverCapacity(v bool)`

SetOverCapacity sets OverCapacity field to given value.


### GetOverCapacitySince

`func (o *DegradationStatus) GetOverCapacitySince() string`

GetOverCapacitySince returns the OverCapacitySince field if non-nil, zero value otherwise.

### GetOverCapacitySinceOk

`func (o *DegradationStatus) GetOverCapacitySinceOk() (*string, bool)`

GetOverCapacitySinceOk returns a tuple with the OverCapacitySince field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOverCapacitySince

`func (o *DegradationStatus) SetOverCapacitySince(v string)`

SetOverCapacitySince sets OverCapacitySince field to given value.

### HasOverCapacitySince

`func (o *DegradationStatus) HasOverCapacitySince() bool`

HasOverCapacitySince returns a boolean if a field has been set.

### SetOverCapacitySinceNil

`func (o *DegradationStatus) SetOverCapacitySinceNil(b bool)`

 SetOverCapacitySinceNil sets the value for OverCapacitySince to be an explicit nil

### UnsetOverCapacitySince
`func (o *DegradationStatus) UnsetOverCapacitySince()`

UnsetOverCapacitySince ensures that no value is present for OverCapacitySince, not even an explicit nil
### GetOverages

`func (o *DegradationStatus) GetOverages() []KeyOverage`

GetOverages returns the Overages field if non-nil, zero value otherwise.

### GetOveragesOk

`func (o *DegradationStatus) GetOveragesOk() (*[]KeyOverage, bool)`

GetOveragesOk returns a tuple with the Overages field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOverages

`func (o *DegradationStatus) SetOverages(v []KeyOverage)`

SetOverages sets Overages field to given value.


### GetSelectionPolicy

`func (o *DegradationStatus) GetSelectionPolicy() SelectionPolicy`

GetSelectionPolicy returns the SelectionPolicy field if non-nil, zero value otherwise.

### GetSelectionPolicyOk

`func (o *DegradationStatus) GetSelectionPolicyOk() (*SelectionPolicy, bool)`

GetSelectionPolicyOk returns a tuple with the SelectionPolicy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSelectionPolicy

`func (o *DegradationStatus) SetSelectionPolicy(v SelectionPolicy)`

SetSelectionPolicy sets SelectionPolicy field to given value.


### GetStage

`func (o *DegradationStatus) GetStage() DegradationStage`

GetStage returns the Stage field if non-nil, zero value otherwise.

### GetStageOk

`func (o *DegradationStatus) GetStageOk() (*DegradationStage, bool)`

GetStageOk returns a tuple with the Stage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStage

`func (o *DegradationStatus) SetStage(v DegradationStage)`

SetStage sets Stage field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
