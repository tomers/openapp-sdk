# MeInvitationAsset

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AccessControl** | Pointer to [**NullablePublicPortalAccessControl**](PublicPortalAccessControl.md) | Integration / building summary with photo (best-effort). | [optional]
**BuildingName** | Pointer to **interface{}** |  | [optional]
**CanRead** | **bool** | True when the requester has &#x60;integrations:read&#x60; on the invite&#39;s org, i.e. may open the invitation&#39;s management page. Mirrors the authz check in &#x60;list_integration_access_invites&#x60;. |
**ClaimedAt** | Pointer to **NullableString** |  | [optional]
**Grants** | [**[]PublicInviteGrant**](PublicInviteGrant.md) | One entry per grant in the invite, with label / door image / lights flag enriched the same way as &#x60;GET /public/access/invites/{token}&#x60;. Lets clients render multi-door invitations without an extra round trip. |
**IntegrationId** | **string** |  |
**InviteLinkId** | **string** |  |
**InviteeMessage** | Pointer to **interface{}** |  | [optional]
**LastUsedAt** | Pointer to **NullableString** |  | [optional]
**MaxUses** | Pointer to **NullableInt32** |  | [optional]
**Name** | Pointer to **NullableString** | Admin-defined invite name (mirrors &#x60;PublicInviteResponse.name&#x60;). | [optional]
**OrgId** | **string** |  |
**OrgImageThumbUrl** | Pointer to **NullableString** | Presigned URL for the &#x60;thumb&#x60; rendition of the org logo, for small renders. Absent when the source is already thumb-sized; fall back to &#x60;org_image_url&#x60;. | [optional]
**OrgImageUrl** | Pointer to **NullableString** | Presigned org logo URL (best-effort). | [optional]
**PhotoThumbUrl** | Pointer to **NullableString** | Presigned URL for the &#x60;thumb&#x60; rendition of the invite photo, for small renders. Absent when the source is already thumb-sized; fall back to &#x60;photo_url&#x60;. | [optional]
**PhotoUrl** | Pointer to **NullableString** | Presigned URL for the invite&#39;s main photo (best-effort). | [optional]
**State** | **string** |  |
**Uses** | **int32** | Number of times the invite has been used. |
**ValidFrom** | Pointer to **NullableString** |  | [optional]
**ValidTo** | Pointer to **NullableString** |  | [optional]

## Methods

### NewMeInvitationAsset

`func NewMeInvitationAsset(canRead bool, grants []PublicInviteGrant, integrationId string, inviteLinkId string, orgId string, state string, uses int32, ) *MeInvitationAsset`

NewMeInvitationAsset instantiates a new MeInvitationAsset object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMeInvitationAssetWithDefaults

`func NewMeInvitationAssetWithDefaults() *MeInvitationAsset`

NewMeInvitationAssetWithDefaults instantiates a new MeInvitationAsset object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAccessControl

`func (o *MeInvitationAsset) GetAccessControl() PublicPortalAccessControl`

GetAccessControl returns the AccessControl field if non-nil, zero value otherwise.

### GetAccessControlOk

`func (o *MeInvitationAsset) GetAccessControlOk() (*PublicPortalAccessControl, bool)`

GetAccessControlOk returns a tuple with the AccessControl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAccessControl

`func (o *MeInvitationAsset) SetAccessControl(v PublicPortalAccessControl)`

SetAccessControl sets AccessControl field to given value.

### HasAccessControl

`func (o *MeInvitationAsset) HasAccessControl() bool`

HasAccessControl returns a boolean if a field has been set.

### SetAccessControlNil

`func (o *MeInvitationAsset) SetAccessControlNil(b bool)`

 SetAccessControlNil sets the value for AccessControl to be an explicit nil

### UnsetAccessControl
`func (o *MeInvitationAsset) UnsetAccessControl()`

UnsetAccessControl ensures that no value is present for AccessControl, not even an explicit nil
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
### GetCanRead

`func (o *MeInvitationAsset) GetCanRead() bool`

GetCanRead returns the CanRead field if non-nil, zero value otherwise.

### GetCanReadOk

`func (o *MeInvitationAsset) GetCanReadOk() (*bool, bool)`

GetCanReadOk returns a tuple with the CanRead field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanRead

`func (o *MeInvitationAsset) SetCanRead(v bool)`

SetCanRead sets CanRead field to given value.


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
### GetGrants

`func (o *MeInvitationAsset) GetGrants() []PublicInviteGrant`

GetGrants returns the Grants field if non-nil, zero value otherwise.

### GetGrantsOk

`func (o *MeInvitationAsset) GetGrantsOk() (*[]PublicInviteGrant, bool)`

GetGrantsOk returns a tuple with the Grants field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGrants

`func (o *MeInvitationAsset) SetGrants(v []PublicInviteGrant)`

SetGrants sets Grants field to given value.


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


### GetInviteeMessage

`func (o *MeInvitationAsset) GetInviteeMessage() interface{}`

GetInviteeMessage returns the InviteeMessage field if non-nil, zero value otherwise.

### GetInviteeMessageOk

`func (o *MeInvitationAsset) GetInviteeMessageOk() (*interface{}, bool)`

GetInviteeMessageOk returns a tuple with the InviteeMessage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteeMessage

`func (o *MeInvitationAsset) SetInviteeMessage(v interface{})`

SetInviteeMessage sets InviteeMessage field to given value.

### HasInviteeMessage

`func (o *MeInvitationAsset) HasInviteeMessage() bool`

HasInviteeMessage returns a boolean if a field has been set.

### SetInviteeMessageNil

`func (o *MeInvitationAsset) SetInviteeMessageNil(b bool)`

 SetInviteeMessageNil sets the value for InviteeMessage to be an explicit nil

### UnsetInviteeMessage
`func (o *MeInvitationAsset) UnsetInviteeMessage()`

UnsetInviteeMessage ensures that no value is present for InviteeMessage, not even an explicit nil
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
### GetName

`func (o *MeInvitationAsset) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *MeInvitationAsset) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *MeInvitationAsset) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *MeInvitationAsset) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *MeInvitationAsset) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *MeInvitationAsset) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
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


### GetOrgImageThumbUrl

`func (o *MeInvitationAsset) GetOrgImageThumbUrl() string`

GetOrgImageThumbUrl returns the OrgImageThumbUrl field if non-nil, zero value otherwise.

### GetOrgImageThumbUrlOk

`func (o *MeInvitationAsset) GetOrgImageThumbUrlOk() (*string, bool)`

GetOrgImageThumbUrlOk returns a tuple with the OrgImageThumbUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgImageThumbUrl

`func (o *MeInvitationAsset) SetOrgImageThumbUrl(v string)`

SetOrgImageThumbUrl sets OrgImageThumbUrl field to given value.

### HasOrgImageThumbUrl

`func (o *MeInvitationAsset) HasOrgImageThumbUrl() bool`

HasOrgImageThumbUrl returns a boolean if a field has been set.

### SetOrgImageThumbUrlNil

`func (o *MeInvitationAsset) SetOrgImageThumbUrlNil(b bool)`

 SetOrgImageThumbUrlNil sets the value for OrgImageThumbUrl to be an explicit nil

### UnsetOrgImageThumbUrl
`func (o *MeInvitationAsset) UnsetOrgImageThumbUrl()`

UnsetOrgImageThumbUrl ensures that no value is present for OrgImageThumbUrl, not even an explicit nil
### GetOrgImageUrl

`func (o *MeInvitationAsset) GetOrgImageUrl() string`

GetOrgImageUrl returns the OrgImageUrl field if non-nil, zero value otherwise.

### GetOrgImageUrlOk

`func (o *MeInvitationAsset) GetOrgImageUrlOk() (*string, bool)`

GetOrgImageUrlOk returns a tuple with the OrgImageUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgImageUrl

`func (o *MeInvitationAsset) SetOrgImageUrl(v string)`

SetOrgImageUrl sets OrgImageUrl field to given value.

### HasOrgImageUrl

`func (o *MeInvitationAsset) HasOrgImageUrl() bool`

HasOrgImageUrl returns a boolean if a field has been set.

### SetOrgImageUrlNil

`func (o *MeInvitationAsset) SetOrgImageUrlNil(b bool)`

 SetOrgImageUrlNil sets the value for OrgImageUrl to be an explicit nil

### UnsetOrgImageUrl
`func (o *MeInvitationAsset) UnsetOrgImageUrl()`

UnsetOrgImageUrl ensures that no value is present for OrgImageUrl, not even an explicit nil
### GetPhotoThumbUrl

`func (o *MeInvitationAsset) GetPhotoThumbUrl() string`

GetPhotoThumbUrl returns the PhotoThumbUrl field if non-nil, zero value otherwise.

### GetPhotoThumbUrlOk

`func (o *MeInvitationAsset) GetPhotoThumbUrlOk() (*string, bool)`

GetPhotoThumbUrlOk returns a tuple with the PhotoThumbUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhotoThumbUrl

`func (o *MeInvitationAsset) SetPhotoThumbUrl(v string)`

SetPhotoThumbUrl sets PhotoThumbUrl field to given value.

### HasPhotoThumbUrl

`func (o *MeInvitationAsset) HasPhotoThumbUrl() bool`

HasPhotoThumbUrl returns a boolean if a field has been set.

### SetPhotoThumbUrlNil

`func (o *MeInvitationAsset) SetPhotoThumbUrlNil(b bool)`

 SetPhotoThumbUrlNil sets the value for PhotoThumbUrl to be an explicit nil

### UnsetPhotoThumbUrl
`func (o *MeInvitationAsset) UnsetPhotoThumbUrl()`

UnsetPhotoThumbUrl ensures that no value is present for PhotoThumbUrl, not even an explicit nil
### GetPhotoUrl

`func (o *MeInvitationAsset) GetPhotoUrl() string`

GetPhotoUrl returns the PhotoUrl field if non-nil, zero value otherwise.

### GetPhotoUrlOk

`func (o *MeInvitationAsset) GetPhotoUrlOk() (*string, bool)`

GetPhotoUrlOk returns a tuple with the PhotoUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhotoUrl

`func (o *MeInvitationAsset) SetPhotoUrl(v string)`

SetPhotoUrl sets PhotoUrl field to given value.

### HasPhotoUrl

`func (o *MeInvitationAsset) HasPhotoUrl() bool`

HasPhotoUrl returns a boolean if a field has been set.

### SetPhotoUrlNil

`func (o *MeInvitationAsset) SetPhotoUrlNil(b bool)`

 SetPhotoUrlNil sets the value for PhotoUrl to be an explicit nil

### UnsetPhotoUrl
`func (o *MeInvitationAsset) UnsetPhotoUrl()`

UnsetPhotoUrl ensures that no value is present for PhotoUrl, not even an explicit nil
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
