# LifeSafetyAttestation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AttestedAt** | **time.Time** |  |
**AttestedByUserId** | Pointer to **NullableString** |  | [optional]
**EntrapmentProtectionPresent** | Pointer to **NullableBool** |  | [optional]
**FireDoorListingUnaltered** | **bool** |  |
**IndependentEgress** | **bool** |  |
**InstallerName** | **NullableString** |  |
**ListedLocalSystem** | Pointer to **NullableBool** |  | [optional]
**OpenappNotFireAlarmInterface** | **bool** |  |
**PowerFailBehavior** | [**PowerFailBehavior**](PowerFailBehavior.md) |  |

## Methods

### NewLifeSafetyAttestation

`func NewLifeSafetyAttestation(attestedAt time.Time, fireDoorListingUnaltered bool, independentEgress bool, installerName NullableString, openappNotFireAlarmInterface bool, powerFailBehavior PowerFailBehavior, ) *LifeSafetyAttestation`

NewLifeSafetyAttestation instantiates a new LifeSafetyAttestation object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewLifeSafetyAttestationWithDefaults

`func NewLifeSafetyAttestationWithDefaults() *LifeSafetyAttestation`

NewLifeSafetyAttestationWithDefaults instantiates a new LifeSafetyAttestation object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAttestedAt

`func (o *LifeSafetyAttestation) GetAttestedAt() time.Time`

GetAttestedAt returns the AttestedAt field if non-nil, zero value otherwise.

### GetAttestedAtOk

`func (o *LifeSafetyAttestation) GetAttestedAtOk() (*time.Time, bool)`

GetAttestedAtOk returns a tuple with the AttestedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAttestedAt

`func (o *LifeSafetyAttestation) SetAttestedAt(v time.Time)`

SetAttestedAt sets AttestedAt field to given value.


### GetAttestedByUserId

`func (o *LifeSafetyAttestation) GetAttestedByUserId() string`

GetAttestedByUserId returns the AttestedByUserId field if non-nil, zero value otherwise.

### GetAttestedByUserIdOk

`func (o *LifeSafetyAttestation) GetAttestedByUserIdOk() (*string, bool)`

GetAttestedByUserIdOk returns a tuple with the AttestedByUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAttestedByUserId

`func (o *LifeSafetyAttestation) SetAttestedByUserId(v string)`

SetAttestedByUserId sets AttestedByUserId field to given value.

### HasAttestedByUserId

`func (o *LifeSafetyAttestation) HasAttestedByUserId() bool`

HasAttestedByUserId returns a boolean if a field has been set.

### SetAttestedByUserIdNil

`func (o *LifeSafetyAttestation) SetAttestedByUserIdNil(b bool)`

 SetAttestedByUserIdNil sets the value for AttestedByUserId to be an explicit nil

### UnsetAttestedByUserId
`func (o *LifeSafetyAttestation) UnsetAttestedByUserId()`

UnsetAttestedByUserId ensures that no value is present for AttestedByUserId, not even an explicit nil
### GetEntrapmentProtectionPresent

`func (o *LifeSafetyAttestation) GetEntrapmentProtectionPresent() bool`

GetEntrapmentProtectionPresent returns the EntrapmentProtectionPresent field if non-nil, zero value otherwise.

### GetEntrapmentProtectionPresentOk

`func (o *LifeSafetyAttestation) GetEntrapmentProtectionPresentOk() (*bool, bool)`

GetEntrapmentProtectionPresentOk returns a tuple with the EntrapmentProtectionPresent field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntrapmentProtectionPresent

`func (o *LifeSafetyAttestation) SetEntrapmentProtectionPresent(v bool)`

SetEntrapmentProtectionPresent sets EntrapmentProtectionPresent field to given value.

### HasEntrapmentProtectionPresent

`func (o *LifeSafetyAttestation) HasEntrapmentProtectionPresent() bool`

HasEntrapmentProtectionPresent returns a boolean if a field has been set.

### SetEntrapmentProtectionPresentNil

`func (o *LifeSafetyAttestation) SetEntrapmentProtectionPresentNil(b bool)`

 SetEntrapmentProtectionPresentNil sets the value for EntrapmentProtectionPresent to be an explicit nil

### UnsetEntrapmentProtectionPresent
`func (o *LifeSafetyAttestation) UnsetEntrapmentProtectionPresent()`

UnsetEntrapmentProtectionPresent ensures that no value is present for EntrapmentProtectionPresent, not even an explicit nil
### GetFireDoorListingUnaltered

`func (o *LifeSafetyAttestation) GetFireDoorListingUnaltered() bool`

GetFireDoorListingUnaltered returns the FireDoorListingUnaltered field if non-nil, zero value otherwise.

### GetFireDoorListingUnalteredOk

`func (o *LifeSafetyAttestation) GetFireDoorListingUnalteredOk() (*bool, bool)`

GetFireDoorListingUnalteredOk returns a tuple with the FireDoorListingUnaltered field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFireDoorListingUnaltered

`func (o *LifeSafetyAttestation) SetFireDoorListingUnaltered(v bool)`

SetFireDoorListingUnaltered sets FireDoorListingUnaltered field to given value.


### GetIndependentEgress

`func (o *LifeSafetyAttestation) GetIndependentEgress() bool`

GetIndependentEgress returns the IndependentEgress field if non-nil, zero value otherwise.

### GetIndependentEgressOk

`func (o *LifeSafetyAttestation) GetIndependentEgressOk() (*bool, bool)`

GetIndependentEgressOk returns a tuple with the IndependentEgress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIndependentEgress

`func (o *LifeSafetyAttestation) SetIndependentEgress(v bool)`

SetIndependentEgress sets IndependentEgress field to given value.


### GetInstallerName

`func (o *LifeSafetyAttestation) GetInstallerName() string`

GetInstallerName returns the InstallerName field if non-nil, zero value otherwise.

### GetInstallerNameOk

`func (o *LifeSafetyAttestation) GetInstallerNameOk() (*string, bool)`

GetInstallerNameOk returns a tuple with the InstallerName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInstallerName

`func (o *LifeSafetyAttestation) SetInstallerName(v string)`

SetInstallerName sets InstallerName field to given value.


### SetInstallerNameNil

`func (o *LifeSafetyAttestation) SetInstallerNameNil(b bool)`

 SetInstallerNameNil sets the value for InstallerName to be an explicit nil

### UnsetInstallerName
`func (o *LifeSafetyAttestation) UnsetInstallerName()`

UnsetInstallerName ensures that no value is present for InstallerName, not even an explicit nil
### GetListedLocalSystem

`func (o *LifeSafetyAttestation) GetListedLocalSystem() bool`

GetListedLocalSystem returns the ListedLocalSystem field if non-nil, zero value otherwise.

### GetListedLocalSystemOk

`func (o *LifeSafetyAttestation) GetListedLocalSystemOk() (*bool, bool)`

GetListedLocalSystemOk returns a tuple with the ListedLocalSystem field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListedLocalSystem

`func (o *LifeSafetyAttestation) SetListedLocalSystem(v bool)`

SetListedLocalSystem sets ListedLocalSystem field to given value.

### HasListedLocalSystem

`func (o *LifeSafetyAttestation) HasListedLocalSystem() bool`

HasListedLocalSystem returns a boolean if a field has been set.

### SetListedLocalSystemNil

`func (o *LifeSafetyAttestation) SetListedLocalSystemNil(b bool)`

 SetListedLocalSystemNil sets the value for ListedLocalSystem to be an explicit nil

### UnsetListedLocalSystem
`func (o *LifeSafetyAttestation) UnsetListedLocalSystem()`

UnsetListedLocalSystem ensures that no value is present for ListedLocalSystem, not even an explicit nil
### GetOpenappNotFireAlarmInterface

`func (o *LifeSafetyAttestation) GetOpenappNotFireAlarmInterface() bool`

GetOpenappNotFireAlarmInterface returns the OpenappNotFireAlarmInterface field if non-nil, zero value otherwise.

### GetOpenappNotFireAlarmInterfaceOk

`func (o *LifeSafetyAttestation) GetOpenappNotFireAlarmInterfaceOk() (*bool, bool)`

GetOpenappNotFireAlarmInterfaceOk returns a tuple with the OpenappNotFireAlarmInterface field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpenappNotFireAlarmInterface

`func (o *LifeSafetyAttestation) SetOpenappNotFireAlarmInterface(v bool)`

SetOpenappNotFireAlarmInterface sets OpenappNotFireAlarmInterface field to given value.


### GetPowerFailBehavior

`func (o *LifeSafetyAttestation) GetPowerFailBehavior() PowerFailBehavior`

GetPowerFailBehavior returns the PowerFailBehavior field if non-nil, zero value otherwise.

### GetPowerFailBehaviorOk

`func (o *LifeSafetyAttestation) GetPowerFailBehaviorOk() (*PowerFailBehavior, bool)`

GetPowerFailBehaviorOk returns a tuple with the PowerFailBehavior field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPowerFailBehavior

`func (o *LifeSafetyAttestation) SetPowerFailBehavior(v PowerFailBehavior)`

SetPowerFailBehavior sets PowerFailBehavior field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
