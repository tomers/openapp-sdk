# SiteAccessDevicesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Cameras** | [**[]SiteAccessDevice**](SiteAccessDevice.md) |  |
**Lights** | [**[]SiteAccessDevice**](SiteAccessDevice.md) |  |
**Openers** | [**[]SiteAccessDevice**](SiteAccessDevice.md) |  |
**Unresolved** | [**[]SiteAccessUnresolvedRef**](SiteAccessUnresolvedRef.md) | Door references that resolve to no device. Never empty-by-omission. |

## Methods

### NewSiteAccessDevicesResponse

`func NewSiteAccessDevicesResponse(cameras []SiteAccessDevice, lights []SiteAccessDevice, openers []SiteAccessDevice, unresolved []SiteAccessUnresolvedRef, ) *SiteAccessDevicesResponse`

NewSiteAccessDevicesResponse instantiates a new SiteAccessDevicesResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSiteAccessDevicesResponseWithDefaults

`func NewSiteAccessDevicesResponseWithDefaults() *SiteAccessDevicesResponse`

NewSiteAccessDevicesResponseWithDefaults instantiates a new SiteAccessDevicesResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCameras

`func (o *SiteAccessDevicesResponse) GetCameras() []SiteAccessDevice`

GetCameras returns the Cameras field if non-nil, zero value otherwise.

### GetCamerasOk

`func (o *SiteAccessDevicesResponse) GetCamerasOk() (*[]SiteAccessDevice, bool)`

GetCamerasOk returns a tuple with the Cameras field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCameras

`func (o *SiteAccessDevicesResponse) SetCameras(v []SiteAccessDevice)`

SetCameras sets Cameras field to given value.


### GetLights

`func (o *SiteAccessDevicesResponse) GetLights() []SiteAccessDevice`

GetLights returns the Lights field if non-nil, zero value otherwise.

### GetLightsOk

`func (o *SiteAccessDevicesResponse) GetLightsOk() (*[]SiteAccessDevice, bool)`

GetLightsOk returns a tuple with the Lights field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLights

`func (o *SiteAccessDevicesResponse) SetLights(v []SiteAccessDevice)`

SetLights sets Lights field to given value.


### GetOpeners

`func (o *SiteAccessDevicesResponse) GetOpeners() []SiteAccessDevice`

GetOpeners returns the Openers field if non-nil, zero value otherwise.

### GetOpenersOk

`func (o *SiteAccessDevicesResponse) GetOpenersOk() (*[]SiteAccessDevice, bool)`

GetOpenersOk returns a tuple with the Openers field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpeners

`func (o *SiteAccessDevicesResponse) SetOpeners(v []SiteAccessDevice)`

SetOpeners sets Openers field to given value.


### GetUnresolved

`func (o *SiteAccessDevicesResponse) GetUnresolved() []SiteAccessUnresolvedRef`

GetUnresolved returns the Unresolved field if non-nil, zero value otherwise.

### GetUnresolvedOk

`func (o *SiteAccessDevicesResponse) GetUnresolvedOk() (*[]SiteAccessUnresolvedRef, bool)`

GetUnresolvedOk returns a tuple with the Unresolved field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnresolved

`func (o *SiteAccessDevicesResponse) SetUnresolved(v []SiteAccessUnresolvedRef)`

SetUnresolved sets Unresolved field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
