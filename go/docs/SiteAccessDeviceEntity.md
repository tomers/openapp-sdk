# SiteAccessDeviceEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Doors** | [**[]SiteAccessDeviceDoor**](SiteAccessDeviceDoor.md) | Doors this entity serves, each carrying that door&#39;s light auto-off config. |
**EntityId** | **string** |  |
**EntityType** | **string** | &#x60;entity_type&#x60; as stored (&#x60;switch&#x60;, &#x60;door&#x60;, &#x60;light&#x60;, …). |
**Label** | Pointer to **NullableString** | The label the door gave this opener/light, when set. | [optional]
**Name** | **string** |  |

## Methods

### NewSiteAccessDeviceEntity

`func NewSiteAccessDeviceEntity(doors []SiteAccessDeviceDoor, entityId string, entityType string, name string, ) *SiteAccessDeviceEntity`

NewSiteAccessDeviceEntity instantiates a new SiteAccessDeviceEntity object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSiteAccessDeviceEntityWithDefaults

`func NewSiteAccessDeviceEntityWithDefaults() *SiteAccessDeviceEntity`

NewSiteAccessDeviceEntityWithDefaults instantiates a new SiteAccessDeviceEntity object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDoors

`func (o *SiteAccessDeviceEntity) GetDoors() []SiteAccessDeviceDoor`

GetDoors returns the Doors field if non-nil, zero value otherwise.

### GetDoorsOk

`func (o *SiteAccessDeviceEntity) GetDoorsOk() (*[]SiteAccessDeviceDoor, bool)`

GetDoorsOk returns a tuple with the Doors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoors

`func (o *SiteAccessDeviceEntity) SetDoors(v []SiteAccessDeviceDoor)`

SetDoors sets Doors field to given value.


### GetEntityId

`func (o *SiteAccessDeviceEntity) GetEntityId() string`

GetEntityId returns the EntityId field if non-nil, zero value otherwise.

### GetEntityIdOk

`func (o *SiteAccessDeviceEntity) GetEntityIdOk() (*string, bool)`

GetEntityIdOk returns a tuple with the EntityId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityId

`func (o *SiteAccessDeviceEntity) SetEntityId(v string)`

SetEntityId sets EntityId field to given value.


### GetEntityType

`func (o *SiteAccessDeviceEntity) GetEntityType() string`

GetEntityType returns the EntityType field if non-nil, zero value otherwise.

### GetEntityTypeOk

`func (o *SiteAccessDeviceEntity) GetEntityTypeOk() (*string, bool)`

GetEntityTypeOk returns a tuple with the EntityType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityType

`func (o *SiteAccessDeviceEntity) SetEntityType(v string)`

SetEntityType sets EntityType field to given value.


### GetLabel

`func (o *SiteAccessDeviceEntity) GetLabel() string`

GetLabel returns the Label field if non-nil, zero value otherwise.

### GetLabelOk

`func (o *SiteAccessDeviceEntity) GetLabelOk() (*string, bool)`

GetLabelOk returns a tuple with the Label field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLabel

`func (o *SiteAccessDeviceEntity) SetLabel(v string)`

SetLabel sets Label field to given value.

### HasLabel

`func (o *SiteAccessDeviceEntity) HasLabel() bool`

HasLabel returns a boolean if a field has been set.

### SetLabelNil

`func (o *SiteAccessDeviceEntity) SetLabelNil(b bool)`

 SetLabelNil sets the value for Label to be an explicit nil

### UnsetLabel
`func (o *SiteAccessDeviceEntity) UnsetLabel()`

UnsetLabel ensures that no value is present for Label, not even an explicit nil
### GetName

`func (o *SiteAccessDeviceEntity) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *SiteAccessDeviceEntity) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *SiteAccessDeviceEntity) SetName(v string)`

SetName sets Name field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
