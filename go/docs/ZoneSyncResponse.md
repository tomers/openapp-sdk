# ZoneSyncResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DevicesPlaced** | **int64** | Devices whose zone this sync changed. |
**EntitiesPlaced** | **int64** | Entities whose placement this sync changed. |
**SkippedManual** | **int64** | Devices and entities left alone because a user changed their placement since the last sync, or because a door lists the entity and owns its zone. |
**ZonesCreated** | **int64** | Org zones created for provider zones with no unique name match. |
**ZonesLinked** | **int64** | Provider zones newly linked to an existing org zone (by unique normalized name). |

## Methods

### NewZoneSyncResponse

`func NewZoneSyncResponse(devicesPlaced int64, entitiesPlaced int64, skippedManual int64, zonesCreated int64, zonesLinked int64, ) *ZoneSyncResponse`

NewZoneSyncResponse instantiates a new ZoneSyncResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewZoneSyncResponseWithDefaults

`func NewZoneSyncResponseWithDefaults() *ZoneSyncResponse`

NewZoneSyncResponseWithDefaults instantiates a new ZoneSyncResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDevicesPlaced

`func (o *ZoneSyncResponse) GetDevicesPlaced() int64`

GetDevicesPlaced returns the DevicesPlaced field if non-nil, zero value otherwise.

### GetDevicesPlacedOk

`func (o *ZoneSyncResponse) GetDevicesPlacedOk() (*int64, bool)`

GetDevicesPlacedOk returns a tuple with the DevicesPlaced field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDevicesPlaced

`func (o *ZoneSyncResponse) SetDevicesPlaced(v int64)`

SetDevicesPlaced sets DevicesPlaced field to given value.


### GetEntitiesPlaced

`func (o *ZoneSyncResponse) GetEntitiesPlaced() int64`

GetEntitiesPlaced returns the EntitiesPlaced field if non-nil, zero value otherwise.

### GetEntitiesPlacedOk

`func (o *ZoneSyncResponse) GetEntitiesPlacedOk() (*int64, bool)`

GetEntitiesPlacedOk returns a tuple with the EntitiesPlaced field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntitiesPlaced

`func (o *ZoneSyncResponse) SetEntitiesPlaced(v int64)`

SetEntitiesPlaced sets EntitiesPlaced field to given value.


### GetSkippedManual

`func (o *ZoneSyncResponse) GetSkippedManual() int64`

GetSkippedManual returns the SkippedManual field if non-nil, zero value otherwise.

### GetSkippedManualOk

`func (o *ZoneSyncResponse) GetSkippedManualOk() (*int64, bool)`

GetSkippedManualOk returns a tuple with the SkippedManual field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSkippedManual

`func (o *ZoneSyncResponse) SetSkippedManual(v int64)`

SetSkippedManual sets SkippedManual field to given value.


### GetZonesCreated

`func (o *ZoneSyncResponse) GetZonesCreated() int64`

GetZonesCreated returns the ZonesCreated field if non-nil, zero value otherwise.

### GetZonesCreatedOk

`func (o *ZoneSyncResponse) GetZonesCreatedOk() (*int64, bool)`

GetZonesCreatedOk returns a tuple with the ZonesCreated field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZonesCreated

`func (o *ZoneSyncResponse) SetZonesCreated(v int64)`

SetZonesCreated sets ZonesCreated field to given value.


### GetZonesLinked

`func (o *ZoneSyncResponse) GetZonesLinked() int64`

GetZonesLinked returns the ZonesLinked field if non-nil, zero value otherwise.

### GetZonesLinkedOk

`func (o *ZoneSyncResponse) GetZonesLinkedOk() (*int64, bool)`

GetZonesLinkedOk returns a tuple with the ZonesLinked field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZonesLinked

`func (o *ZoneSyncResponse) SetZonesLinked(v int64)`

SetZonesLinked sets ZonesLinked field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
