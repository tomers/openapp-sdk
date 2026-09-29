# MeShareAuthoringResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AllowUnlimited** | **bool** |  |
**BlockedReason** | Pointer to **NullableString** |  | [optional]
**DefaultMaxUses** | **int32** |  |
**IntegrationId** | **string** |  |
**MaxDurationSeconds** | **int64** | Effective share TTL cap (seconds): min of invitation max duration and share max (system default 86400 when &#x60;share_max_duration&#x60; is unset). |
**MaxUses** | Pointer to **NullableInt32** |  | [optional]
**OrgId** | **string** |  |
**Portals** | [**[]MeShareAuthoringPortal**](MeShareAuthoringPortal.md) |  |
**RequireJustification** | **bool** |  |
**UsesEditable** | **bool** |  |

## Methods

### NewMeShareAuthoringResponse

`func NewMeShareAuthoringResponse(allowUnlimited bool, defaultMaxUses int32, integrationId string, maxDurationSeconds int64, orgId string, portals []MeShareAuthoringPortal, requireJustification bool, usesEditable bool, ) *MeShareAuthoringResponse`

NewMeShareAuthoringResponse instantiates a new MeShareAuthoringResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMeShareAuthoringResponseWithDefaults

`func NewMeShareAuthoringResponseWithDefaults() *MeShareAuthoringResponse`

NewMeShareAuthoringResponseWithDefaults instantiates a new MeShareAuthoringResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAllowUnlimited

`func (o *MeShareAuthoringResponse) GetAllowUnlimited() bool`

GetAllowUnlimited returns the AllowUnlimited field if non-nil, zero value otherwise.

### GetAllowUnlimitedOk

`func (o *MeShareAuthoringResponse) GetAllowUnlimitedOk() (*bool, bool)`

GetAllowUnlimitedOk returns a tuple with the AllowUnlimited field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowUnlimited

`func (o *MeShareAuthoringResponse) SetAllowUnlimited(v bool)`

SetAllowUnlimited sets AllowUnlimited field to given value.


### GetBlockedReason

`func (o *MeShareAuthoringResponse) GetBlockedReason() string`

GetBlockedReason returns the BlockedReason field if non-nil, zero value otherwise.

### GetBlockedReasonOk

`func (o *MeShareAuthoringResponse) GetBlockedReasonOk() (*string, bool)`

GetBlockedReasonOk returns a tuple with the BlockedReason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBlockedReason

`func (o *MeShareAuthoringResponse) SetBlockedReason(v string)`

SetBlockedReason sets BlockedReason field to given value.

### HasBlockedReason

`func (o *MeShareAuthoringResponse) HasBlockedReason() bool`

HasBlockedReason returns a boolean if a field has been set.

### SetBlockedReasonNil

`func (o *MeShareAuthoringResponse) SetBlockedReasonNil(b bool)`

 SetBlockedReasonNil sets the value for BlockedReason to be an explicit nil

### UnsetBlockedReason
`func (o *MeShareAuthoringResponse) UnsetBlockedReason()`

UnsetBlockedReason ensures that no value is present for BlockedReason, not even an explicit nil
### GetDefaultMaxUses

`func (o *MeShareAuthoringResponse) GetDefaultMaxUses() int32`

GetDefaultMaxUses returns the DefaultMaxUses field if non-nil, zero value otherwise.

### GetDefaultMaxUsesOk

`func (o *MeShareAuthoringResponse) GetDefaultMaxUsesOk() (*int32, bool)`

GetDefaultMaxUsesOk returns a tuple with the DefaultMaxUses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultMaxUses

`func (o *MeShareAuthoringResponse) SetDefaultMaxUses(v int32)`

SetDefaultMaxUses sets DefaultMaxUses field to given value.


### GetIntegrationId

`func (o *MeShareAuthoringResponse) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *MeShareAuthoringResponse) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *MeShareAuthoringResponse) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### GetMaxDurationSeconds

`func (o *MeShareAuthoringResponse) GetMaxDurationSeconds() int64`

GetMaxDurationSeconds returns the MaxDurationSeconds field if non-nil, zero value otherwise.

### GetMaxDurationSecondsOk

`func (o *MeShareAuthoringResponse) GetMaxDurationSecondsOk() (*int64, bool)`

GetMaxDurationSecondsOk returns a tuple with the MaxDurationSeconds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxDurationSeconds

`func (o *MeShareAuthoringResponse) SetMaxDurationSeconds(v int64)`

SetMaxDurationSeconds sets MaxDurationSeconds field to given value.


### GetMaxUses

`func (o *MeShareAuthoringResponse) GetMaxUses() int32`

GetMaxUses returns the MaxUses field if non-nil, zero value otherwise.

### GetMaxUsesOk

`func (o *MeShareAuthoringResponse) GetMaxUsesOk() (*int32, bool)`

GetMaxUsesOk returns a tuple with the MaxUses field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxUses

`func (o *MeShareAuthoringResponse) SetMaxUses(v int32)`

SetMaxUses sets MaxUses field to given value.

### HasMaxUses

`func (o *MeShareAuthoringResponse) HasMaxUses() bool`

HasMaxUses returns a boolean if a field has been set.

### SetMaxUsesNil

`func (o *MeShareAuthoringResponse) SetMaxUsesNil(b bool)`

 SetMaxUsesNil sets the value for MaxUses to be an explicit nil

### UnsetMaxUses
`func (o *MeShareAuthoringResponse) UnsetMaxUses()`

UnsetMaxUses ensures that no value is present for MaxUses, not even an explicit nil
### GetOrgId

`func (o *MeShareAuthoringResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *MeShareAuthoringResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *MeShareAuthoringResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetPortals

`func (o *MeShareAuthoringResponse) GetPortals() []MeShareAuthoringPortal`

GetPortals returns the Portals field if non-nil, zero value otherwise.

### GetPortalsOk

`func (o *MeShareAuthoringResponse) GetPortalsOk() (*[]MeShareAuthoringPortal, bool)`

GetPortalsOk returns a tuple with the Portals field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPortals

`func (o *MeShareAuthoringResponse) SetPortals(v []MeShareAuthoringPortal)`

SetPortals sets Portals field to given value.


### GetRequireJustification

`func (o *MeShareAuthoringResponse) GetRequireJustification() bool`

GetRequireJustification returns the RequireJustification field if non-nil, zero value otherwise.

### GetRequireJustificationOk

`func (o *MeShareAuthoringResponse) GetRequireJustificationOk() (*bool, bool)`

GetRequireJustificationOk returns a tuple with the RequireJustification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequireJustification

`func (o *MeShareAuthoringResponse) SetRequireJustification(v bool)`

SetRequireJustification sets RequireJustification field to given value.


### GetUsesEditable

`func (o *MeShareAuthoringResponse) GetUsesEditable() bool`

GetUsesEditable returns the UsesEditable field if non-nil, zero value otherwise.

### GetUsesEditableOk

`func (o *MeShareAuthoringResponse) GetUsesEditableOk() (*bool, bool)`

GetUsesEditableOk returns a tuple with the UsesEditable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsesEditable

`func (o *MeShareAuthoringResponse) SetUsesEditable(v bool)`

SetUsesEditable sets UsesEditable field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
