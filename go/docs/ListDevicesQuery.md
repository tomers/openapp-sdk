# ListDevicesQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IncludeDeleted** | **bool** |  |
**IncludeMetadata** | **bool** |  |
**OnlyDeleted** | **bool** |  |
**Limit** | Pointer to **int32** | Number of items per page. Default from config, max 200. | [optional]
**Offset** | Pointer to **int32** | Number of items to skip. Default 0. | [optional]
**DeviceKind** | Pointer to **NullableString** | Optional filter: only devices with metadata.kind equal to this value (e.g. virtual_access_portal). Requires integration_id. Filtering done at SQL level. | [optional]
**ExternalId** | Pointer to **NullableString** | Optional filter: only devices with this external_id. Requires integration_id and device_kind. Returns at most 1 device. Filtering done at SQL level. | [optional]
**HasExternalId** | Pointer to **NullableBool** | When true, only devices with a non-empty &#x60;external_id&#x60;. Requires &#x60;integration_id&#x60;. | [optional]
**HasGo2rtcChannel** | Pointer to **NullableBool** | When true, only devices whose metadata JSON has a non-empty &#x60;channel&#x60; (go2rtc cameras). Requires &#x60;integration_id&#x60;. | [optional]
**IncludeStale** | Pointer to **NullableBool** | When true with &#x60;integration_id&#x60;, include &#x60;stale&#x60; per device when the provider supports it. | [optional]
**IntegrationId** | Pointer to **NullableString** | Optional filter: only devices belonging to this integration. | [optional]
**Q** | Pointer to **NullableString** | Case-insensitive substring match on localized device name (JSON). Best-effort when &#x60;integration_id&#x60; is set (SQL ILIKE). | [optional]

## Methods

### NewListDevicesQuery

`func NewListDevicesQuery(includeDeleted bool, includeMetadata bool, onlyDeleted bool, ) *ListDevicesQuery`

NewListDevicesQuery instantiates a new ListDevicesQuery object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListDevicesQueryWithDefaults

`func NewListDevicesQueryWithDefaults() *ListDevicesQuery`

NewListDevicesQueryWithDefaults instantiates a new ListDevicesQuery object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIncludeDeleted

`func (o *ListDevicesQuery) GetIncludeDeleted() bool`

GetIncludeDeleted returns the IncludeDeleted field if non-nil, zero value otherwise.

### GetIncludeDeletedOk

`func (o *ListDevicesQuery) GetIncludeDeletedOk() (*bool, bool)`

GetIncludeDeletedOk returns a tuple with the IncludeDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeDeleted

`func (o *ListDevicesQuery) SetIncludeDeleted(v bool)`

SetIncludeDeleted sets IncludeDeleted field to given value.


### GetIncludeMetadata

`func (o *ListDevicesQuery) GetIncludeMetadata() bool`

GetIncludeMetadata returns the IncludeMetadata field if non-nil, zero value otherwise.

### GetIncludeMetadataOk

`func (o *ListDevicesQuery) GetIncludeMetadataOk() (*bool, bool)`

GetIncludeMetadataOk returns a tuple with the IncludeMetadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeMetadata

`func (o *ListDevicesQuery) SetIncludeMetadata(v bool)`

SetIncludeMetadata sets IncludeMetadata field to given value.


### GetOnlyDeleted

`func (o *ListDevicesQuery) GetOnlyDeleted() bool`

GetOnlyDeleted returns the OnlyDeleted field if non-nil, zero value otherwise.

### GetOnlyDeletedOk

`func (o *ListDevicesQuery) GetOnlyDeletedOk() (*bool, bool)`

GetOnlyDeletedOk returns a tuple with the OnlyDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOnlyDeleted

`func (o *ListDevicesQuery) SetOnlyDeleted(v bool)`

SetOnlyDeleted sets OnlyDeleted field to given value.


### GetLimit

`func (o *ListDevicesQuery) GetLimit() int32`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *ListDevicesQuery) GetLimitOk() (*int32, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *ListDevicesQuery) SetLimit(v int32)`

SetLimit sets Limit field to given value.

### HasLimit

`func (o *ListDevicesQuery) HasLimit() bool`

HasLimit returns a boolean if a field has been set.

### GetOffset

`func (o *ListDevicesQuery) GetOffset() int32`

GetOffset returns the Offset field if non-nil, zero value otherwise.

### GetOffsetOk

`func (o *ListDevicesQuery) GetOffsetOk() (*int32, bool)`

GetOffsetOk returns a tuple with the Offset field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOffset

`func (o *ListDevicesQuery) SetOffset(v int32)`

SetOffset sets Offset field to given value.

### HasOffset

`func (o *ListDevicesQuery) HasOffset() bool`

HasOffset returns a boolean if a field has been set.

### GetDeviceKind

`func (o *ListDevicesQuery) GetDeviceKind() string`

GetDeviceKind returns the DeviceKind field if non-nil, zero value otherwise.

### GetDeviceKindOk

`func (o *ListDevicesQuery) GetDeviceKindOk() (*string, bool)`

GetDeviceKindOk returns a tuple with the DeviceKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceKind

`func (o *ListDevicesQuery) SetDeviceKind(v string)`

SetDeviceKind sets DeviceKind field to given value.

### HasDeviceKind

`func (o *ListDevicesQuery) HasDeviceKind() bool`

HasDeviceKind returns a boolean if a field has been set.

### SetDeviceKindNil

`func (o *ListDevicesQuery) SetDeviceKindNil(b bool)`

 SetDeviceKindNil sets the value for DeviceKind to be an explicit nil

### UnsetDeviceKind
`func (o *ListDevicesQuery) UnsetDeviceKind()`

UnsetDeviceKind ensures that no value is present for DeviceKind, not even an explicit nil
### GetExternalId

`func (o *ListDevicesQuery) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *ListDevicesQuery) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *ListDevicesQuery) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.

### HasExternalId

`func (o *ListDevicesQuery) HasExternalId() bool`

HasExternalId returns a boolean if a field has been set.

### SetExternalIdNil

`func (o *ListDevicesQuery) SetExternalIdNil(b bool)`

 SetExternalIdNil sets the value for ExternalId to be an explicit nil

### UnsetExternalId
`func (o *ListDevicesQuery) UnsetExternalId()`

UnsetExternalId ensures that no value is present for ExternalId, not even an explicit nil
### GetHasExternalId

`func (o *ListDevicesQuery) GetHasExternalId() bool`

GetHasExternalId returns the HasExternalId field if non-nil, zero value otherwise.

### GetHasExternalIdOk

`func (o *ListDevicesQuery) GetHasExternalIdOk() (*bool, bool)`

GetHasExternalIdOk returns a tuple with the HasExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasExternalId

`func (o *ListDevicesQuery) SetHasExternalId(v bool)`

SetHasExternalId sets HasExternalId field to given value.

### HasHasExternalId

`func (o *ListDevicesQuery) HasHasExternalId() bool`

HasHasExternalId returns a boolean if a field has been set.

### SetHasExternalIdNil

`func (o *ListDevicesQuery) SetHasExternalIdNil(b bool)`

 SetHasExternalIdNil sets the value for HasExternalId to be an explicit nil

### UnsetHasExternalId
`func (o *ListDevicesQuery) UnsetHasExternalId()`

UnsetHasExternalId ensures that no value is present for HasExternalId, not even an explicit nil
### GetHasGo2rtcChannel

`func (o *ListDevicesQuery) GetHasGo2rtcChannel() bool`

GetHasGo2rtcChannel returns the HasGo2rtcChannel field if non-nil, zero value otherwise.

### GetHasGo2rtcChannelOk

`func (o *ListDevicesQuery) GetHasGo2rtcChannelOk() (*bool, bool)`

GetHasGo2rtcChannelOk returns a tuple with the HasGo2rtcChannel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasGo2rtcChannel

`func (o *ListDevicesQuery) SetHasGo2rtcChannel(v bool)`

SetHasGo2rtcChannel sets HasGo2rtcChannel field to given value.

### HasHasGo2rtcChannel

`func (o *ListDevicesQuery) HasHasGo2rtcChannel() bool`

HasHasGo2rtcChannel returns a boolean if a field has been set.

### SetHasGo2rtcChannelNil

`func (o *ListDevicesQuery) SetHasGo2rtcChannelNil(b bool)`

 SetHasGo2rtcChannelNil sets the value for HasGo2rtcChannel to be an explicit nil

### UnsetHasGo2rtcChannel
`func (o *ListDevicesQuery) UnsetHasGo2rtcChannel()`

UnsetHasGo2rtcChannel ensures that no value is present for HasGo2rtcChannel, not even an explicit nil
### GetIncludeStale

`func (o *ListDevicesQuery) GetIncludeStale() bool`

GetIncludeStale returns the IncludeStale field if non-nil, zero value otherwise.

### GetIncludeStaleOk

`func (o *ListDevicesQuery) GetIncludeStaleOk() (*bool, bool)`

GetIncludeStaleOk returns a tuple with the IncludeStale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeStale

`func (o *ListDevicesQuery) SetIncludeStale(v bool)`

SetIncludeStale sets IncludeStale field to given value.

### HasIncludeStale

`func (o *ListDevicesQuery) HasIncludeStale() bool`

HasIncludeStale returns a boolean if a field has been set.

### SetIncludeStaleNil

`func (o *ListDevicesQuery) SetIncludeStaleNil(b bool)`

 SetIncludeStaleNil sets the value for IncludeStale to be an explicit nil

### UnsetIncludeStale
`func (o *ListDevicesQuery) UnsetIncludeStale()`

UnsetIncludeStale ensures that no value is present for IncludeStale, not even an explicit nil
### GetIntegrationId

`func (o *ListDevicesQuery) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *ListDevicesQuery) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *ListDevicesQuery) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.

### HasIntegrationId

`func (o *ListDevicesQuery) HasIntegrationId() bool`

HasIntegrationId returns a boolean if a field has been set.

### SetIntegrationIdNil

`func (o *ListDevicesQuery) SetIntegrationIdNil(b bool)`

 SetIntegrationIdNil sets the value for IntegrationId to be an explicit nil

### UnsetIntegrationId
`func (o *ListDevicesQuery) UnsetIntegrationId()`

UnsetIntegrationId ensures that no value is present for IntegrationId, not even an explicit nil
### GetQ

`func (o *ListDevicesQuery) GetQ() string`

GetQ returns the Q field if non-nil, zero value otherwise.

### GetQOk

`func (o *ListDevicesQuery) GetQOk() (*string, bool)`

GetQOk returns a tuple with the Q field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQ

`func (o *ListDevicesQuery) SetQ(v string)`

SetQ sets Q field to given value.

### HasQ

`func (o *ListDevicesQuery) HasQ() bool`

HasQ returns a boolean if a field has been set.

### SetQNil

`func (o *ListDevicesQuery) SetQNil(b bool)`

 SetQNil sets the value for Q to be an explicit nil

### UnsetQ
`func (o *ListDevicesQuery) UnsetQ()`

UnsetQ ensures that no value is present for Q, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
