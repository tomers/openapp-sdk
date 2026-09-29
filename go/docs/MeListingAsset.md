# MeListingAsset

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BuildingName** | Pointer to **interface{}** |  | [optional]
**CallEligible** | **bool** | True when this user would be a callee for intercom calls (policy + role + receives_calls). |
**DisplayName** | Pointer to **interface{}** |  | [optional]
**DndEnabled** | Pointer to **NullableBool** | True when calls are silenced for this listing (global DND or per-listing DND). | [optional]
**EntityId** | **string** |  |
**Floor** | Pointer to **interface{}** |  | [optional]
**FloorNumber** | Pointer to **NullableInt64** |  | [optional]
**ImageThumbUrl** | Pointer to **NullableString** | Presigned URL for the &#x60;thumb&#x60; rendition of the listing photo, for small renders. Absent when the source is already thumb-sized; fall back to &#x60;image_url&#x60;. | [optional]
**ImageUrl** | Pointer to **NullableString** | Presigned listing (entity) photo URL (best-effort). | [optional]
**IntegrationId** | **string** |  |
**ListingLabel** | Pointer to **interface{}** |  | [optional]
**ListingNumber** | Pointer to **NullableInt64** |  | [optional]
**OrgId** | **string** |  |
**OrgName** | Pointer to **interface{}** |  | [optional]
**ReceivesCalls** | **bool** |  |
**Role** | **string** |  |

## Methods

### NewMeListingAsset

`func NewMeListingAsset(callEligible bool, entityId string, integrationId string, orgId string, receivesCalls bool, role string, ) *MeListingAsset`

NewMeListingAsset instantiates a new MeListingAsset object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMeListingAssetWithDefaults

`func NewMeListingAssetWithDefaults() *MeListingAsset`

NewMeListingAssetWithDefaults instantiates a new MeListingAsset object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBuildingName

`func (o *MeListingAsset) GetBuildingName() interface{}`

GetBuildingName returns the BuildingName field if non-nil, zero value otherwise.

### GetBuildingNameOk

`func (o *MeListingAsset) GetBuildingNameOk() (*interface{}, bool)`

GetBuildingNameOk returns a tuple with the BuildingName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBuildingName

`func (o *MeListingAsset) SetBuildingName(v interface{})`

SetBuildingName sets BuildingName field to given value.

### HasBuildingName

`func (o *MeListingAsset) HasBuildingName() bool`

HasBuildingName returns a boolean if a field has been set.

### SetBuildingNameNil

`func (o *MeListingAsset) SetBuildingNameNil(b bool)`

 SetBuildingNameNil sets the value for BuildingName to be an explicit nil

### UnsetBuildingName
`func (o *MeListingAsset) UnsetBuildingName()`

UnsetBuildingName ensures that no value is present for BuildingName, not even an explicit nil
### GetCallEligible

`func (o *MeListingAsset) GetCallEligible() bool`

GetCallEligible returns the CallEligible field if non-nil, zero value otherwise.

### GetCallEligibleOk

`func (o *MeListingAsset) GetCallEligibleOk() (*bool, bool)`

GetCallEligibleOk returns a tuple with the CallEligible field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCallEligible

`func (o *MeListingAsset) SetCallEligible(v bool)`

SetCallEligible sets CallEligible field to given value.


### GetDisplayName

`func (o *MeListingAsset) GetDisplayName() interface{}`

GetDisplayName returns the DisplayName field if non-nil, zero value otherwise.

### GetDisplayNameOk

`func (o *MeListingAsset) GetDisplayNameOk() (*interface{}, bool)`

GetDisplayNameOk returns a tuple with the DisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisplayName

`func (o *MeListingAsset) SetDisplayName(v interface{})`

SetDisplayName sets DisplayName field to given value.

### HasDisplayName

`func (o *MeListingAsset) HasDisplayName() bool`

HasDisplayName returns a boolean if a field has been set.

### SetDisplayNameNil

`func (o *MeListingAsset) SetDisplayNameNil(b bool)`

 SetDisplayNameNil sets the value for DisplayName to be an explicit nil

### UnsetDisplayName
`func (o *MeListingAsset) UnsetDisplayName()`

UnsetDisplayName ensures that no value is present for DisplayName, not even an explicit nil
### GetDndEnabled

`func (o *MeListingAsset) GetDndEnabled() bool`

GetDndEnabled returns the DndEnabled field if non-nil, zero value otherwise.

### GetDndEnabledOk

`func (o *MeListingAsset) GetDndEnabledOk() (*bool, bool)`

GetDndEnabledOk returns a tuple with the DndEnabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDndEnabled

`func (o *MeListingAsset) SetDndEnabled(v bool)`

SetDndEnabled sets DndEnabled field to given value.

### HasDndEnabled

`func (o *MeListingAsset) HasDndEnabled() bool`

HasDndEnabled returns a boolean if a field has been set.

### SetDndEnabledNil

`func (o *MeListingAsset) SetDndEnabledNil(b bool)`

 SetDndEnabledNil sets the value for DndEnabled to be an explicit nil

### UnsetDndEnabled
`func (o *MeListingAsset) UnsetDndEnabled()`

UnsetDndEnabled ensures that no value is present for DndEnabled, not even an explicit nil
### GetEntityId

`func (o *MeListingAsset) GetEntityId() string`

GetEntityId returns the EntityId field if non-nil, zero value otherwise.

### GetEntityIdOk

`func (o *MeListingAsset) GetEntityIdOk() (*string, bool)`

GetEntityIdOk returns a tuple with the EntityId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityId

`func (o *MeListingAsset) SetEntityId(v string)`

SetEntityId sets EntityId field to given value.


### GetFloor

`func (o *MeListingAsset) GetFloor() interface{}`

GetFloor returns the Floor field if non-nil, zero value otherwise.

### GetFloorOk

`func (o *MeListingAsset) GetFloorOk() (*interface{}, bool)`

GetFloorOk returns a tuple with the Floor field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFloor

`func (o *MeListingAsset) SetFloor(v interface{})`

SetFloor sets Floor field to given value.

### HasFloor

`func (o *MeListingAsset) HasFloor() bool`

HasFloor returns a boolean if a field has been set.

### SetFloorNil

`func (o *MeListingAsset) SetFloorNil(b bool)`

 SetFloorNil sets the value for Floor to be an explicit nil

### UnsetFloor
`func (o *MeListingAsset) UnsetFloor()`

UnsetFloor ensures that no value is present for Floor, not even an explicit nil
### GetFloorNumber

`func (o *MeListingAsset) GetFloorNumber() int64`

GetFloorNumber returns the FloorNumber field if non-nil, zero value otherwise.

### GetFloorNumberOk

`func (o *MeListingAsset) GetFloorNumberOk() (*int64, bool)`

GetFloorNumberOk returns a tuple with the FloorNumber field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFloorNumber

`func (o *MeListingAsset) SetFloorNumber(v int64)`

SetFloorNumber sets FloorNumber field to given value.

### HasFloorNumber

`func (o *MeListingAsset) HasFloorNumber() bool`

HasFloorNumber returns a boolean if a field has been set.

### SetFloorNumberNil

`func (o *MeListingAsset) SetFloorNumberNil(b bool)`

 SetFloorNumberNil sets the value for FloorNumber to be an explicit nil

### UnsetFloorNumber
`func (o *MeListingAsset) UnsetFloorNumber()`

UnsetFloorNumber ensures that no value is present for FloorNumber, not even an explicit nil
### GetImageThumbUrl

`func (o *MeListingAsset) GetImageThumbUrl() string`

GetImageThumbUrl returns the ImageThumbUrl field if non-nil, zero value otherwise.

### GetImageThumbUrlOk

`func (o *MeListingAsset) GetImageThumbUrlOk() (*string, bool)`

GetImageThumbUrlOk returns a tuple with the ImageThumbUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageThumbUrl

`func (o *MeListingAsset) SetImageThumbUrl(v string)`

SetImageThumbUrl sets ImageThumbUrl field to given value.

### HasImageThumbUrl

`func (o *MeListingAsset) HasImageThumbUrl() bool`

HasImageThumbUrl returns a boolean if a field has been set.

### SetImageThumbUrlNil

`func (o *MeListingAsset) SetImageThumbUrlNil(b bool)`

 SetImageThumbUrlNil sets the value for ImageThumbUrl to be an explicit nil

### UnsetImageThumbUrl
`func (o *MeListingAsset) UnsetImageThumbUrl()`

UnsetImageThumbUrl ensures that no value is present for ImageThumbUrl, not even an explicit nil
### GetImageUrl

`func (o *MeListingAsset) GetImageUrl() string`

GetImageUrl returns the ImageUrl field if non-nil, zero value otherwise.

### GetImageUrlOk

`func (o *MeListingAsset) GetImageUrlOk() (*string, bool)`

GetImageUrlOk returns a tuple with the ImageUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageUrl

`func (o *MeListingAsset) SetImageUrl(v string)`

SetImageUrl sets ImageUrl field to given value.

### HasImageUrl

`func (o *MeListingAsset) HasImageUrl() bool`

HasImageUrl returns a boolean if a field has been set.

### SetImageUrlNil

`func (o *MeListingAsset) SetImageUrlNil(b bool)`

 SetImageUrlNil sets the value for ImageUrl to be an explicit nil

### UnsetImageUrl
`func (o *MeListingAsset) UnsetImageUrl()`

UnsetImageUrl ensures that no value is present for ImageUrl, not even an explicit nil
### GetIntegrationId

`func (o *MeListingAsset) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *MeListingAsset) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *MeListingAsset) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### GetListingLabel

`func (o *MeListingAsset) GetListingLabel() interface{}`

GetListingLabel returns the ListingLabel field if non-nil, zero value otherwise.

### GetListingLabelOk

`func (o *MeListingAsset) GetListingLabelOk() (*interface{}, bool)`

GetListingLabelOk returns a tuple with the ListingLabel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListingLabel

`func (o *MeListingAsset) SetListingLabel(v interface{})`

SetListingLabel sets ListingLabel field to given value.

### HasListingLabel

`func (o *MeListingAsset) HasListingLabel() bool`

HasListingLabel returns a boolean if a field has been set.

### SetListingLabelNil

`func (o *MeListingAsset) SetListingLabelNil(b bool)`

 SetListingLabelNil sets the value for ListingLabel to be an explicit nil

### UnsetListingLabel
`func (o *MeListingAsset) UnsetListingLabel()`

UnsetListingLabel ensures that no value is present for ListingLabel, not even an explicit nil
### GetListingNumber

`func (o *MeListingAsset) GetListingNumber() int64`

GetListingNumber returns the ListingNumber field if non-nil, zero value otherwise.

### GetListingNumberOk

`func (o *MeListingAsset) GetListingNumberOk() (*int64, bool)`

GetListingNumberOk returns a tuple with the ListingNumber field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListingNumber

`func (o *MeListingAsset) SetListingNumber(v int64)`

SetListingNumber sets ListingNumber field to given value.

### HasListingNumber

`func (o *MeListingAsset) HasListingNumber() bool`

HasListingNumber returns a boolean if a field has been set.

### SetListingNumberNil

`func (o *MeListingAsset) SetListingNumberNil(b bool)`

 SetListingNumberNil sets the value for ListingNumber to be an explicit nil

### UnsetListingNumber
`func (o *MeListingAsset) UnsetListingNumber()`

UnsetListingNumber ensures that no value is present for ListingNumber, not even an explicit nil
### GetOrgId

`func (o *MeListingAsset) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *MeListingAsset) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *MeListingAsset) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetOrgName

`func (o *MeListingAsset) GetOrgName() interface{}`

GetOrgName returns the OrgName field if non-nil, zero value otherwise.

### GetOrgNameOk

`func (o *MeListingAsset) GetOrgNameOk() (*interface{}, bool)`

GetOrgNameOk returns a tuple with the OrgName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgName

`func (o *MeListingAsset) SetOrgName(v interface{})`

SetOrgName sets OrgName field to given value.

### HasOrgName

`func (o *MeListingAsset) HasOrgName() bool`

HasOrgName returns a boolean if a field has been set.

### SetOrgNameNil

`func (o *MeListingAsset) SetOrgNameNil(b bool)`

 SetOrgNameNil sets the value for OrgName to be an explicit nil

### UnsetOrgName
`func (o *MeListingAsset) UnsetOrgName()`

UnsetOrgName ensures that no value is present for OrgName, not even an explicit nil
### GetReceivesCalls

`func (o *MeListingAsset) GetReceivesCalls() bool`

GetReceivesCalls returns the ReceivesCalls field if non-nil, zero value otherwise.

### GetReceivesCallsOk

`func (o *MeListingAsset) GetReceivesCallsOk() (*bool, bool)`

GetReceivesCallsOk returns a tuple with the ReceivesCalls field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReceivesCalls

`func (o *MeListingAsset) SetReceivesCalls(v bool)`

SetReceivesCalls sets ReceivesCalls field to given value.


### GetRole

`func (o *MeListingAsset) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *MeListingAsset) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *MeListingAsset) SetRole(v string)`

SetRole sets Role field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
