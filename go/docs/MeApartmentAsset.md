# MeApartmentAsset

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ApartmentLabel** | Pointer to **interface{}** |  | [optional]
**ApartmentNumber** | Pointer to **NullableInt64** |  | [optional]
**BuildingName** | Pointer to **interface{}** |  | [optional]
**CallEligible** | **bool** | True when this user would be a callee for intercom calls (policy + role + receives_calls). |
**DisplayName** | Pointer to **interface{}** |  | [optional]
**DndEnabled** | Pointer to **NullableBool** | True when calls are silenced for this apartment (global DND or per-apartment DND). | [optional]
**EntityId** | **string** |  |
**Floor** | Pointer to **interface{}** |  | [optional]
**FloorNumber** | Pointer to **NullableInt64** |  | [optional]
**IntegrationId** | **string** |  |
**OrgId** | **string** |  |
**ReceivesCalls** | **bool** |  |
**Role** | **string** |  |

## Methods

### NewMeApartmentAsset

`func NewMeApartmentAsset(callEligible bool, entityId string, integrationId string, orgId string, receivesCalls bool, role string, ) *MeApartmentAsset`

NewMeApartmentAsset instantiates a new MeApartmentAsset object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMeApartmentAssetWithDefaults

`func NewMeApartmentAssetWithDefaults() *MeApartmentAsset`

NewMeApartmentAssetWithDefaults instantiates a new MeApartmentAsset object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetApartmentLabel

`func (o *MeApartmentAsset) GetApartmentLabel() interface{}`

GetApartmentLabel returns the ApartmentLabel field if non-nil, zero value otherwise.

### GetApartmentLabelOk

`func (o *MeApartmentAsset) GetApartmentLabelOk() (*interface{}, bool)`

GetApartmentLabelOk returns a tuple with the ApartmentLabel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetApartmentLabel

`func (o *MeApartmentAsset) SetApartmentLabel(v interface{})`

SetApartmentLabel sets ApartmentLabel field to given value.

### HasApartmentLabel

`func (o *MeApartmentAsset) HasApartmentLabel() bool`

HasApartmentLabel returns a boolean if a field has been set.

### SetApartmentLabelNil

`func (o *MeApartmentAsset) SetApartmentLabelNil(b bool)`

 SetApartmentLabelNil sets the value for ApartmentLabel to be an explicit nil

### UnsetApartmentLabel
`func (o *MeApartmentAsset) UnsetApartmentLabel()`

UnsetApartmentLabel ensures that no value is present for ApartmentLabel, not even an explicit nil
### GetApartmentNumber

`func (o *MeApartmentAsset) GetApartmentNumber() int64`

GetApartmentNumber returns the ApartmentNumber field if non-nil, zero value otherwise.

### GetApartmentNumberOk

`func (o *MeApartmentAsset) GetApartmentNumberOk() (*int64, bool)`

GetApartmentNumberOk returns a tuple with the ApartmentNumber field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetApartmentNumber

`func (o *MeApartmentAsset) SetApartmentNumber(v int64)`

SetApartmentNumber sets ApartmentNumber field to given value.

### HasApartmentNumber

`func (o *MeApartmentAsset) HasApartmentNumber() bool`

HasApartmentNumber returns a boolean if a field has been set.

### SetApartmentNumberNil

`func (o *MeApartmentAsset) SetApartmentNumberNil(b bool)`

 SetApartmentNumberNil sets the value for ApartmentNumber to be an explicit nil

### UnsetApartmentNumber
`func (o *MeApartmentAsset) UnsetApartmentNumber()`

UnsetApartmentNumber ensures that no value is present for ApartmentNumber, not even an explicit nil
### GetBuildingName

`func (o *MeApartmentAsset) GetBuildingName() interface{}`

GetBuildingName returns the BuildingName field if non-nil, zero value otherwise.

### GetBuildingNameOk

`func (o *MeApartmentAsset) GetBuildingNameOk() (*interface{}, bool)`

GetBuildingNameOk returns a tuple with the BuildingName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBuildingName

`func (o *MeApartmentAsset) SetBuildingName(v interface{})`

SetBuildingName sets BuildingName field to given value.

### HasBuildingName

`func (o *MeApartmentAsset) HasBuildingName() bool`

HasBuildingName returns a boolean if a field has been set.

### SetBuildingNameNil

`func (o *MeApartmentAsset) SetBuildingNameNil(b bool)`

 SetBuildingNameNil sets the value for BuildingName to be an explicit nil

### UnsetBuildingName
`func (o *MeApartmentAsset) UnsetBuildingName()`

UnsetBuildingName ensures that no value is present for BuildingName, not even an explicit nil
### GetCallEligible

`func (o *MeApartmentAsset) GetCallEligible() bool`

GetCallEligible returns the CallEligible field if non-nil, zero value otherwise.

### GetCallEligibleOk

`func (o *MeApartmentAsset) GetCallEligibleOk() (*bool, bool)`

GetCallEligibleOk returns a tuple with the CallEligible field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCallEligible

`func (o *MeApartmentAsset) SetCallEligible(v bool)`

SetCallEligible sets CallEligible field to given value.


### GetDisplayName

`func (o *MeApartmentAsset) GetDisplayName() interface{}`

GetDisplayName returns the DisplayName field if non-nil, zero value otherwise.

### GetDisplayNameOk

`func (o *MeApartmentAsset) GetDisplayNameOk() (*interface{}, bool)`

GetDisplayNameOk returns a tuple with the DisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisplayName

`func (o *MeApartmentAsset) SetDisplayName(v interface{})`

SetDisplayName sets DisplayName field to given value.

### HasDisplayName

`func (o *MeApartmentAsset) HasDisplayName() bool`

HasDisplayName returns a boolean if a field has been set.

### SetDisplayNameNil

`func (o *MeApartmentAsset) SetDisplayNameNil(b bool)`

 SetDisplayNameNil sets the value for DisplayName to be an explicit nil

### UnsetDisplayName
`func (o *MeApartmentAsset) UnsetDisplayName()`

UnsetDisplayName ensures that no value is present for DisplayName, not even an explicit nil
### GetDndEnabled

`func (o *MeApartmentAsset) GetDndEnabled() bool`

GetDndEnabled returns the DndEnabled field if non-nil, zero value otherwise.

### GetDndEnabledOk

`func (o *MeApartmentAsset) GetDndEnabledOk() (*bool, bool)`

GetDndEnabledOk returns a tuple with the DndEnabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDndEnabled

`func (o *MeApartmentAsset) SetDndEnabled(v bool)`

SetDndEnabled sets DndEnabled field to given value.

### HasDndEnabled

`func (o *MeApartmentAsset) HasDndEnabled() bool`

HasDndEnabled returns a boolean if a field has been set.

### SetDndEnabledNil

`func (o *MeApartmentAsset) SetDndEnabledNil(b bool)`

 SetDndEnabledNil sets the value for DndEnabled to be an explicit nil

### UnsetDndEnabled
`func (o *MeApartmentAsset) UnsetDndEnabled()`

UnsetDndEnabled ensures that no value is present for DndEnabled, not even an explicit nil
### GetEntityId

`func (o *MeApartmentAsset) GetEntityId() string`

GetEntityId returns the EntityId field if non-nil, zero value otherwise.

### GetEntityIdOk

`func (o *MeApartmentAsset) GetEntityIdOk() (*string, bool)`

GetEntityIdOk returns a tuple with the EntityId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityId

`func (o *MeApartmentAsset) SetEntityId(v string)`

SetEntityId sets EntityId field to given value.


### GetFloor

`func (o *MeApartmentAsset) GetFloor() interface{}`

GetFloor returns the Floor field if non-nil, zero value otherwise.

### GetFloorOk

`func (o *MeApartmentAsset) GetFloorOk() (*interface{}, bool)`

GetFloorOk returns a tuple with the Floor field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFloor

`func (o *MeApartmentAsset) SetFloor(v interface{})`

SetFloor sets Floor field to given value.

### HasFloor

`func (o *MeApartmentAsset) HasFloor() bool`

HasFloor returns a boolean if a field has been set.

### SetFloorNil

`func (o *MeApartmentAsset) SetFloorNil(b bool)`

 SetFloorNil sets the value for Floor to be an explicit nil

### UnsetFloor
`func (o *MeApartmentAsset) UnsetFloor()`

UnsetFloor ensures that no value is present for Floor, not even an explicit nil
### GetFloorNumber

`func (o *MeApartmentAsset) GetFloorNumber() int64`

GetFloorNumber returns the FloorNumber field if non-nil, zero value otherwise.

### GetFloorNumberOk

`func (o *MeApartmentAsset) GetFloorNumberOk() (*int64, bool)`

GetFloorNumberOk returns a tuple with the FloorNumber field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFloorNumber

`func (o *MeApartmentAsset) SetFloorNumber(v int64)`

SetFloorNumber sets FloorNumber field to given value.

### HasFloorNumber

`func (o *MeApartmentAsset) HasFloorNumber() bool`

HasFloorNumber returns a boolean if a field has been set.

### SetFloorNumberNil

`func (o *MeApartmentAsset) SetFloorNumberNil(b bool)`

 SetFloorNumberNil sets the value for FloorNumber to be an explicit nil

### UnsetFloorNumber
`func (o *MeApartmentAsset) UnsetFloorNumber()`

UnsetFloorNumber ensures that no value is present for FloorNumber, not even an explicit nil
### GetIntegrationId

`func (o *MeApartmentAsset) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *MeApartmentAsset) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *MeApartmentAsset) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### GetOrgId

`func (o *MeApartmentAsset) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *MeApartmentAsset) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *MeApartmentAsset) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetReceivesCalls

`func (o *MeApartmentAsset) GetReceivesCalls() bool`

GetReceivesCalls returns the ReceivesCalls field if non-nil, zero value otherwise.

### GetReceivesCallsOk

`func (o *MeApartmentAsset) GetReceivesCallsOk() (*bool, bool)`

GetReceivesCallsOk returns a tuple with the ReceivesCalls field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReceivesCalls

`func (o *MeApartmentAsset) SetReceivesCalls(v bool)`

SetReceivesCalls sets ReceivesCalls field to given value.


### GetRole

`func (o *MeApartmentAsset) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *MeApartmentAsset) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *MeApartmentAsset) SetRole(v string)`

SetRole sets Role field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
