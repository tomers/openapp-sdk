# SiteAccessUnresolvedRef

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Doors** | [**[]SiteAccessDeviceDoor**](SiteAccessDeviceDoor.md) |  |
**Reason** | [**SiteAccessUnresolvedReason**](SiteAccessUnresolvedReason.md) |  |
**Reference** | **string** | The raw id from the door&#39;s metadata (entity id for openers/lights, device id for cameras). |
**Role** | [**SiteAccessDeviceRole**](SiteAccessDeviceRole.md) |  |

## Methods

### NewSiteAccessUnresolvedRef

`func NewSiteAccessUnresolvedRef(doors []SiteAccessDeviceDoor, reason SiteAccessUnresolvedReason, reference string, role SiteAccessDeviceRole, ) *SiteAccessUnresolvedRef`

NewSiteAccessUnresolvedRef instantiates a new SiteAccessUnresolvedRef object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSiteAccessUnresolvedRefWithDefaults

`func NewSiteAccessUnresolvedRefWithDefaults() *SiteAccessUnresolvedRef`

NewSiteAccessUnresolvedRefWithDefaults instantiates a new SiteAccessUnresolvedRef object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDoors

`func (o *SiteAccessUnresolvedRef) GetDoors() []SiteAccessDeviceDoor`

GetDoors returns the Doors field if non-nil, zero value otherwise.

### GetDoorsOk

`func (o *SiteAccessUnresolvedRef) GetDoorsOk() (*[]SiteAccessDeviceDoor, bool)`

GetDoorsOk returns a tuple with the Doors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoors

`func (o *SiteAccessUnresolvedRef) SetDoors(v []SiteAccessDeviceDoor)`

SetDoors sets Doors field to given value.


### GetReason

`func (o *SiteAccessUnresolvedRef) GetReason() SiteAccessUnresolvedReason`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *SiteAccessUnresolvedRef) GetReasonOk() (*SiteAccessUnresolvedReason, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *SiteAccessUnresolvedRef) SetReason(v SiteAccessUnresolvedReason)`

SetReason sets Reason field to given value.


### GetReference

`func (o *SiteAccessUnresolvedRef) GetReference() string`

GetReference returns the Reference field if non-nil, zero value otherwise.

### GetReferenceOk

`func (o *SiteAccessUnresolvedRef) GetReferenceOk() (*string, bool)`

GetReferenceOk returns a tuple with the Reference field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReference

`func (o *SiteAccessUnresolvedRef) SetReference(v string)`

SetReference sets Reference field to given value.


### GetRole

`func (o *SiteAccessUnresolvedRef) GetRole() SiteAccessDeviceRole`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *SiteAccessUnresolvedRef) GetRoleOk() (*SiteAccessDeviceRole, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *SiteAccessUnresolvedRef) SetRole(v SiteAccessDeviceRole)`

SetRole sets Role field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
