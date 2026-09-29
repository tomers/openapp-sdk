# DevicePhysicalStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AdminStatus** | Pointer to **NullableString** | Verified hardware-admin status of the integration&#39;s linked account: &#x60;admin&#x60; | &#x60;non_admin&#x60; | null (unknown / unverifiable / stale). | [optional]
**CanClaim** | **bool** | Whether the requester may claim/keep stewardship (verified admin + provider supports it + not held by someone else). |
**HardwareId** | Pointer to **NullableString** | Server-derived hardware identity of the device (controller-level), if any. | [optional]
**HasSteward** | **bool** | Whether some user currently holds the steward slot for this physical device. |
**IsPrimaryInstance** | **bool** | Whether this OpenApp device record is the canonical (primary) instance for the hardware. |
**IsSteward** | **bool** | Whether the requesting user currently holds the steward slot. |
**PhysicalDeviceId** | Pointer to **NullableString** | The physical-device registry id, once the device has been registered/claimed. | [optional]
**ProviderSupportsSteward** | **bool** | Whether the provider supports the device-steward authority tier at all. |

## Methods

### NewDevicePhysicalStatusResponse

`func NewDevicePhysicalStatusResponse(canClaim bool, hasSteward bool, isPrimaryInstance bool, isSteward bool, providerSupportsSteward bool, ) *DevicePhysicalStatusResponse`

NewDevicePhysicalStatusResponse instantiates a new DevicePhysicalStatusResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDevicePhysicalStatusResponseWithDefaults

`func NewDevicePhysicalStatusResponseWithDefaults() *DevicePhysicalStatusResponse`

NewDevicePhysicalStatusResponseWithDefaults instantiates a new DevicePhysicalStatusResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAdminStatus

`func (o *DevicePhysicalStatusResponse) GetAdminStatus() string`

GetAdminStatus returns the AdminStatus field if non-nil, zero value otherwise.

### GetAdminStatusOk

`func (o *DevicePhysicalStatusResponse) GetAdminStatusOk() (*string, bool)`

GetAdminStatusOk returns a tuple with the AdminStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAdminStatus

`func (o *DevicePhysicalStatusResponse) SetAdminStatus(v string)`

SetAdminStatus sets AdminStatus field to given value.

### HasAdminStatus

`func (o *DevicePhysicalStatusResponse) HasAdminStatus() bool`

HasAdminStatus returns a boolean if a field has been set.

### SetAdminStatusNil

`func (o *DevicePhysicalStatusResponse) SetAdminStatusNil(b bool)`

 SetAdminStatusNil sets the value for AdminStatus to be an explicit nil

### UnsetAdminStatus
`func (o *DevicePhysicalStatusResponse) UnsetAdminStatus()`

UnsetAdminStatus ensures that no value is present for AdminStatus, not even an explicit nil
### GetCanClaim

`func (o *DevicePhysicalStatusResponse) GetCanClaim() bool`

GetCanClaim returns the CanClaim field if non-nil, zero value otherwise.

### GetCanClaimOk

`func (o *DevicePhysicalStatusResponse) GetCanClaimOk() (*bool, bool)`

GetCanClaimOk returns a tuple with the CanClaim field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanClaim

`func (o *DevicePhysicalStatusResponse) SetCanClaim(v bool)`

SetCanClaim sets CanClaim field to given value.


### GetHardwareId

`func (o *DevicePhysicalStatusResponse) GetHardwareId() string`

GetHardwareId returns the HardwareId field if non-nil, zero value otherwise.

### GetHardwareIdOk

`func (o *DevicePhysicalStatusResponse) GetHardwareIdOk() (*string, bool)`

GetHardwareIdOk returns a tuple with the HardwareId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHardwareId

`func (o *DevicePhysicalStatusResponse) SetHardwareId(v string)`

SetHardwareId sets HardwareId field to given value.

### HasHardwareId

`func (o *DevicePhysicalStatusResponse) HasHardwareId() bool`

HasHardwareId returns a boolean if a field has been set.

### SetHardwareIdNil

`func (o *DevicePhysicalStatusResponse) SetHardwareIdNil(b bool)`

 SetHardwareIdNil sets the value for HardwareId to be an explicit nil

### UnsetHardwareId
`func (o *DevicePhysicalStatusResponse) UnsetHardwareId()`

UnsetHardwareId ensures that no value is present for HardwareId, not even an explicit nil
### GetHasSteward

`func (o *DevicePhysicalStatusResponse) GetHasSteward() bool`

GetHasSteward returns the HasSteward field if non-nil, zero value otherwise.

### GetHasStewardOk

`func (o *DevicePhysicalStatusResponse) GetHasStewardOk() (*bool, bool)`

GetHasStewardOk returns a tuple with the HasSteward field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasSteward

`func (o *DevicePhysicalStatusResponse) SetHasSteward(v bool)`

SetHasSteward sets HasSteward field to given value.


### GetIsPrimaryInstance

`func (o *DevicePhysicalStatusResponse) GetIsPrimaryInstance() bool`

GetIsPrimaryInstance returns the IsPrimaryInstance field if non-nil, zero value otherwise.

### GetIsPrimaryInstanceOk

`func (o *DevicePhysicalStatusResponse) GetIsPrimaryInstanceOk() (*bool, bool)`

GetIsPrimaryInstanceOk returns a tuple with the IsPrimaryInstance field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsPrimaryInstance

`func (o *DevicePhysicalStatusResponse) SetIsPrimaryInstance(v bool)`

SetIsPrimaryInstance sets IsPrimaryInstance field to given value.


### GetIsSteward

`func (o *DevicePhysicalStatusResponse) GetIsSteward() bool`

GetIsSteward returns the IsSteward field if non-nil, zero value otherwise.

### GetIsStewardOk

`func (o *DevicePhysicalStatusResponse) GetIsStewardOk() (*bool, bool)`

GetIsStewardOk returns a tuple with the IsSteward field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsSteward

`func (o *DevicePhysicalStatusResponse) SetIsSteward(v bool)`

SetIsSteward sets IsSteward field to given value.


### GetPhysicalDeviceId

`func (o *DevicePhysicalStatusResponse) GetPhysicalDeviceId() string`

GetPhysicalDeviceId returns the PhysicalDeviceId field if non-nil, zero value otherwise.

### GetPhysicalDeviceIdOk

`func (o *DevicePhysicalStatusResponse) GetPhysicalDeviceIdOk() (*string, bool)`

GetPhysicalDeviceIdOk returns a tuple with the PhysicalDeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhysicalDeviceId

`func (o *DevicePhysicalStatusResponse) SetPhysicalDeviceId(v string)`

SetPhysicalDeviceId sets PhysicalDeviceId field to given value.

### HasPhysicalDeviceId

`func (o *DevicePhysicalStatusResponse) HasPhysicalDeviceId() bool`

HasPhysicalDeviceId returns a boolean if a field has been set.

### SetPhysicalDeviceIdNil

`func (o *DevicePhysicalStatusResponse) SetPhysicalDeviceIdNil(b bool)`

 SetPhysicalDeviceIdNil sets the value for PhysicalDeviceId to be an explicit nil

### UnsetPhysicalDeviceId
`func (o *DevicePhysicalStatusResponse) UnsetPhysicalDeviceId()`

UnsetPhysicalDeviceId ensures that no value is present for PhysicalDeviceId, not even an explicit nil
### GetProviderSupportsSteward

`func (o *DevicePhysicalStatusResponse) GetProviderSupportsSteward() bool`

GetProviderSupportsSteward returns the ProviderSupportsSteward field if non-nil, zero value otherwise.

### GetProviderSupportsStewardOk

`func (o *DevicePhysicalStatusResponse) GetProviderSupportsStewardOk() (*bool, bool)`

GetProviderSupportsStewardOk returns a tuple with the ProviderSupportsSteward field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderSupportsSteward

`func (o *DevicePhysicalStatusResponse) SetProviderSupportsSteward(v bool)`

SetProviderSupportsSteward sets ProviderSupportsSteward field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
