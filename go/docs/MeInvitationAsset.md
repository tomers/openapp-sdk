# MeInvitationAsset

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BuildingName** | Pointer to **interface{}** |  | [optional]
**ClaimedAt** | Pointer to **NullableString** |  | [optional]
**IntegrationId** | **string** |  |
**InviteLinkId** | **string** |  |
**LastUsedAt** | Pointer to **NullableString** |  | [optional]
**MaxUses** | Pointer to **NullableInt32** |  | [optional]
**OrgId** | **string** |  |
**PortalIds** | **[]string** | Portal IDs the invite grants (for display: \&quot;Scan portal QR to open\&quot;). |
**State** | **string** |  |
**Uses** | **int32** | Number of times the invite has been used. |
**ValidFrom** | Pointer to **NullableString** |  | [optional]
**ValidTo** | Pointer to **NullableString** |  | [optional]

## Methods

### NewMeInvitationAsset

`func NewMeInvitationAsset(integrationId string, inviteLinkId string, orgId string, portalIds []string, state string, uses int32, ) *MeInvitationAsset`

NewMeInvitationAsset instantiates a new MeInvitationAsset object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMeInvitationAssetWithDefaults

`func NewMeInvitationAssetWithDefaults() *MeInvitationAsset`

NewMeInvitationAssetWithDefaults instantiates a new MeInvitationAsset object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBuildingName

`func (o *MeInvitationAsset) GetBuildingName() interface{}`

GetBuildingName returns the BuildingName field if non-nil, zero value otherwise.

### GetBuildingNameOk

`func (o *MeInvitationAsset) GetBuildingNameOk() (*interface{}, bool)`

GetBuildingNameOk returns a tuple with the BuildingName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBuildingName

`func (o *MeInvitationAsset) SetBuildingName(v interface{})`

SetBuildingName sets BuildingName field to given value.

### HasBuildingName

`func (o *MeInvitationAsset) HasBuildingName() bool`

HasBuildingName returns a boolean if a field has been set.

### SetBuildingNameNil

`func (o *MeInvitationAsset) SetBuildingNameNil(b bool)`

 SetBuildingNameNil sets the value for BuildingName to be an explicit nil

### UnsetBuildingName
`func (o *MeInvitationAsset) UnsetBuildingName()`

UnsetBuildingName ensures that no value is present for BuildingName, not even an explicit nil
### GetClaimedAt

`func (o *MeInvitationAsset) GetClaimedAt() string`

GetClaimedAt returns the ClaimedAt field if non-nil, zero value otherwise.

### GetClaimedAtOk

`func (o *MeInvitationAsset) GetClaimedAtOk() (*string, bool)`

GetClaimedAtOk returns a tuple with the ClaimedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetClaimedAt

`func (o *MeInvitationAsset) SetClaimedAt(v string)`

SetClaimedAt sets ClaimedAt field to given value.

### HasClaimedAt

`func (o *MeInvitationAsset) HasClaimedAt() bool`

HasClaimedAt returns a boolean if a field has been set.

### SetClaimedAtNil

`func (o *MeInvitationAsset) SetClaimedAtNil(b bool)`

 SetClaimedAtNil sets the value for ClaimedAt to be an explicit nil

### UnsetClaimedAt
`func (o *MeInvitationAsset) UnsetClaimedAt()`

UnsetClaimedAt ensures that no value is present for ClaimedAt, not even an explicit nil
### GetIntegrationId

`func (o *MeInvitationAsset) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *MeInvitationAsset) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *MeInvitationAsset) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### GetInviteLinkId

`func (o *MeInvitationAsset) GetInviteLinkId() string`

GetInviteLinkId returns the InviteLinkId field if non-nil, zero value otherwise.

### GetInviteLinkIdOk

`func (o *MeInvitationAsset) GetInviteLinkIdOk() (*string, bool)`

GetInviteLinkIdOk returns a tuple with the InviteLinkId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteLinkId

`func (o *MeInvitationAsset) SetInviteLinkId(v string)`

SetInviteLinkId sets InviteLinkId field to given value.


### GetLastUsedAt

`func (o *MeInvitationAsset) GetLastUsedAt() string`

GetLastUsedAt returns the LastUsedAt field if non-nil, zero value otherwise.

### GetLastUsedAtOk

`func (o *MeInvitationAsset) GetLastUsedAtOk() (*string, bool)`

GetLastUsedAtOk returns a tuple with the LastUsedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastUsedAt

`func (o *MeInvitationAsset) SetLastUsedAt(v string)`

SetLastUsedAt sets LastUsedAt field to given value.

### HasLastUsedAt

`func (o *MeInvitationAsset) HasLastUsedAt() bool`

HasLastUsedAt returns a boolean if a field has been set.

### SetLastUsedAtNil

`func (o *MeInvitationAsset) SetLastUsedAtNil(b bool)`

 SetLastUsedAtNil sets the value for LastUsedAt to be an explicit nil

### UnsetLastUsedAt
`func (o *MeInvitationAsset) UnsetLastUsedAt()`

UnsetLastUsedAt ensures that no value is present for LastUsedAt, not even an explicit nil
### GetMaxUses

`func (o *MeInvitationAsset) GetMaxUses() int32`

GetMaxUses returns the MaxUses field if non-nil, zero value otherwise.

### GetMaxUsesOk

`func (o *MeInvitationAsset) GetMaxUsesOk() (*int32, bool)`

GetMaxUsesOk returns a tuple with the MaxUses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxUses

`func (o *MeInvitationAsset) SetMaxUses(v int32)`

SetMaxUses sets MaxUses field to given value.

### HasMaxUses

`func (o *MeInvitationAsset) HasMaxUses() bool`

HasMaxUses returns a boolean if a field has been set.

### SetMaxUsesNil

`func (o *MeInvitationAsset) SetMaxUsesNil(b bool)`

 SetMaxUsesNil sets the value for MaxUses to be an explicit nil

### UnsetMaxUses
`func (o *MeInvitationAsset) UnsetMaxUses()`

UnsetMaxUses ensures that no value is present for MaxUses, not even an explicit nil
### GetOrgId

`func (o *MeInvitationAsset) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *MeInvitationAsset) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *MeInvitationAsset) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetPortalIds

`func (o *MeInvitationAsset) GetPortalIds() []string`

GetPortalIds returns the PortalIds field if non-nil, zero value otherwise.

### GetPortalIdsOk

`func (o *MeInvitationAsset) GetPortalIdsOk() (*[]string, bool)`

GetPortalIdsOk returns a tuple with the PortalIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPortalIds

`func (o *MeInvitationAsset) SetPortalIds(v []string)`

SetPortalIds sets PortalIds field to given value.


### GetState

`func (o *MeInvitationAsset) GetState() string`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *MeInvitationAsset) GetStateOk() (*string, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *MeInvitationAsset) SetState(v string)`

SetState sets State field to given value.


### GetUses

`func (o *MeInvitationAsset) GetUses() int32`

GetUses returns the Uses field if non-nil, zero value otherwise.

### GetUsesOk

`func (o *MeInvitationAsset) GetUsesOk() (*int32, bool)`

GetUsesOk returns a tuple with the Uses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUses

`func (o *MeInvitationAsset) SetUses(v int32)`

SetUses sets Uses field to given value.


### GetValidFrom

`func (o *MeInvitationAsset) GetValidFrom() string`

GetValidFrom returns the ValidFrom field if non-nil, zero value otherwise.

### GetValidFromOk

`func (o *MeInvitationAsset) GetValidFromOk() (*string, bool)`

GetValidFromOk returns a tuple with the ValidFrom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidFrom

`func (o *MeInvitationAsset) SetValidFrom(v string)`

SetValidFrom sets ValidFrom field to given value.

### HasValidFrom

`func (o *MeInvitationAsset) HasValidFrom() bool`

HasValidFrom returns a boolean if a field has been set.

### SetValidFromNil

`func (o *MeInvitationAsset) SetValidFromNil(b bool)`

 SetValidFromNil sets the value for ValidFrom to be an explicit nil

### UnsetValidFrom
`func (o *MeInvitationAsset) UnsetValidFrom()`

UnsetValidFrom ensures that no value is present for ValidFrom, not even an explicit nil
### GetValidTo

`func (o *MeInvitationAsset) GetValidTo() string`

GetValidTo returns the ValidTo field if non-nil, zero value otherwise.

### GetValidToOk

`func (o *MeInvitationAsset) GetValidToOk() (*string, bool)`

GetValidToOk returns a tuple with the ValidTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidTo

`func (o *MeInvitationAsset) SetValidTo(v string)`

SetValidTo sets ValidTo field to given value.

### HasValidTo

`func (o *MeInvitationAsset) HasValidTo() bool`

HasValidTo returns a boolean if a field has been set.

### SetValidToNil

`func (o *MeInvitationAsset) SetValidToNil(b bool)`

 SetValidToNil sets the value for ValidTo to be an explicit nil

### UnsetValidTo
`func (o *MeInvitationAsset) UnsetValidTo()`

UnsetValidTo ensures that no value is present for ValidTo, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
