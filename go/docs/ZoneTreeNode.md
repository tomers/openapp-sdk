# ZoneTreeNode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Children** | [**[]ZoneTreeNode**](ZoneTreeNode.md) |  |
**DeviceCount** | **int64** | Devices placed directly in this zone. |
**EntityCount** | **int64** | Entities whose effective zone is this zone (not descendants). |
**Zone** | [**ZoneResponse**](ZoneResponse.md) |  |

## Methods

### NewZoneTreeNode

`func NewZoneTreeNode(children []ZoneTreeNode, deviceCount int64, entityCount int64, zone ZoneResponse, ) *ZoneTreeNode`

NewZoneTreeNode instantiates a new ZoneTreeNode object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewZoneTreeNodeWithDefaults

`func NewZoneTreeNodeWithDefaults() *ZoneTreeNode`

NewZoneTreeNodeWithDefaults instantiates a new ZoneTreeNode object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetChildren

`func (o *ZoneTreeNode) GetChildren() []ZoneTreeNode`

GetChildren returns the Children field if non-nil, zero value otherwise.

### GetChildrenOk

`func (o *ZoneTreeNode) GetChildrenOk() (*[]ZoneTreeNode, bool)`

GetChildrenOk returns a tuple with the Children field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChildren

`func (o *ZoneTreeNode) SetChildren(v []ZoneTreeNode)`

SetChildren sets Children field to given value.


### GetDeviceCount

`func (o *ZoneTreeNode) GetDeviceCount() int64`

GetDeviceCount returns the DeviceCount field if non-nil, zero value otherwise.

### GetDeviceCountOk

`func (o *ZoneTreeNode) GetDeviceCountOk() (*int64, bool)`

GetDeviceCountOk returns a tuple with the DeviceCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceCount

`func (o *ZoneTreeNode) SetDeviceCount(v int64)`

SetDeviceCount sets DeviceCount field to given value.


### GetEntityCount

`func (o *ZoneTreeNode) GetEntityCount() int64`

GetEntityCount returns the EntityCount field if non-nil, zero value otherwise.

### GetEntityCountOk

`func (o *ZoneTreeNode) GetEntityCountOk() (*int64, bool)`

GetEntityCountOk returns a tuple with the EntityCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityCount

`func (o *ZoneTreeNode) SetEntityCount(v int64)`

SetEntityCount sets EntityCount field to given value.


### GetZone

`func (o *ZoneTreeNode) GetZone() ZoneResponse`

GetZone returns the Zone field if non-nil, zero value otherwise.

### GetZoneOk

`func (o *ZoneTreeNode) GetZoneOk() (*ZoneResponse, bool)`

GetZoneOk returns a tuple with the Zone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZone

`func (o *ZoneTreeNode) SetZone(v ZoneResponse)`

SetZone sets Zone field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
