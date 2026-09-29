# SiteAccessDeviceDoor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AutoOffConfig** | Pointer to **interface{}** |  | [optional]
**DoorDeviceId** | **string** |  |
**LifeSafety** | Pointer to [**NullableSiteAccessDoorLifeSafety**](SiteAccessDoorLifeSafety.md) | The door&#39;s life-safety class and attestation, for open gating. | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) |  |

## Methods

### NewSiteAccessDeviceDoor

`func NewSiteAccessDeviceDoor(doorDeviceId string, name LocalizedString, ) *SiteAccessDeviceDoor`

NewSiteAccessDeviceDoor instantiates a new SiteAccessDeviceDoor object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSiteAccessDeviceDoorWithDefaults

`func NewSiteAccessDeviceDoorWithDefaults() *SiteAccessDeviceDoor`

NewSiteAccessDeviceDoorWithDefaults instantiates a new SiteAccessDeviceDoor object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAutoOffConfig

`func (o *SiteAccessDeviceDoor) GetAutoOffConfig() interface{}`

GetAutoOffConfig returns the AutoOffConfig field if non-nil, zero value otherwise.

### GetAutoOffConfigOk

`func (o *SiteAccessDeviceDoor) GetAutoOffConfigOk() (*interface{}, bool)`

GetAutoOffConfigOk returns a tuple with the AutoOffConfig field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAutoOffConfig

`func (o *SiteAccessDeviceDoor) SetAutoOffConfig(v interface{})`

SetAutoOffConfig sets AutoOffConfig field to given value.

### HasAutoOffConfig

`func (o *SiteAccessDeviceDoor) HasAutoOffConfig() bool`

HasAutoOffConfig returns a boolean if a field has been set.

### SetAutoOffConfigNil

`func (o *SiteAccessDeviceDoor) SetAutoOffConfigNil(b bool)`

 SetAutoOffConfigNil sets the value for AutoOffConfig to be an explicit nil

### UnsetAutoOffConfig
`func (o *SiteAccessDeviceDoor) UnsetAutoOffConfig()`

UnsetAutoOffConfig ensures that no value is present for AutoOffConfig, not even an explicit nil
### GetDoorDeviceId

`func (o *SiteAccessDeviceDoor) GetDoorDeviceId() string`

GetDoorDeviceId returns the DoorDeviceId field if non-nil, zero value otherwise.

### GetDoorDeviceIdOk

`func (o *SiteAccessDeviceDoor) GetDoorDeviceIdOk() (*string, bool)`

GetDoorDeviceIdOk returns a tuple with the DoorDeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorDeviceId

`func (o *SiteAccessDeviceDoor) SetDoorDeviceId(v string)`

SetDoorDeviceId sets DoorDeviceId field to given value.


### GetLifeSafety

`func (o *SiteAccessDeviceDoor) GetLifeSafety() SiteAccessDoorLifeSafety`

GetLifeSafety returns the LifeSafety field if non-nil, zero value otherwise.

### GetLifeSafetyOk

`func (o *SiteAccessDeviceDoor) GetLifeSafetyOk() (*SiteAccessDoorLifeSafety, bool)`

GetLifeSafetyOk returns a tuple with the LifeSafety field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLifeSafety

`func (o *SiteAccessDeviceDoor) SetLifeSafety(v SiteAccessDoorLifeSafety)`

SetLifeSafety sets LifeSafety field to given value.

### HasLifeSafety

`func (o *SiteAccessDeviceDoor) HasLifeSafety() bool`

HasLifeSafety returns a boolean if a field has been set.

### SetLifeSafetyNil

`func (o *SiteAccessDeviceDoor) SetLifeSafetyNil(b bool)`

 SetLifeSafetyNil sets the value for LifeSafety to be an explicit nil

### UnsetLifeSafety
`func (o *SiteAccessDeviceDoor) UnsetLifeSafety()`

UnsetLifeSafety ensures that no value is present for LifeSafety, not even an explicit nil
### GetName

`func (o *SiteAccessDeviceDoor) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *SiteAccessDeviceDoor) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *SiteAccessDeviceDoor) SetName(v LocalizedString)`

SetName sets Name field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
