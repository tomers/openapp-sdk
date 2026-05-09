# PublicSessionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BuildingDisplayName** | Pointer to **interface{}** |  | [optional]
**CallTargetDisplayName** | Pointer to **interface{}** |  | [optional]
**CallTargetLocationLine** | Pointer to **NullableString** | Single line for apartment + floor (e.g. \&quot;3 · Floor 2\&quot;). | [optional]
**CalleePeerId** | **string** |  |
**CalleesNotified** | Pointer to **NullableBool** | Whether at least one callee received a push notification. Caller uses this to skip peer-unavailable retries when false. | [optional]
**CallerPeerId** | **string** |  |
**ExpiresAt** | Pointer to **NullableString** |  | [optional]
**HasLights** | Pointer to **NullableBool** | Whether the portal has light devices configured. Callee-only. | [optional]
**Mode** | Pointer to **NullableString** | Call mode: \&quot;voice\&quot; or \&quot;video\&quot;. Video on by default when mode is \&quot;video\&quot;. | [optional]
**Peerjs** | [**PeerJsConfig**](PeerJsConfig.md) |  |
**PortalCameraDeviceIds** | Pointer to **[]string** |  | [optional]
**Role** | [**PublicSessionRole**](PublicSessionRole.md) |  |
**SessionId** | **string** |  |
**State** | **string** |  |

## Methods

### NewPublicSessionResponse

`func NewPublicSessionResponse(calleePeerId string, callerPeerId string, peerjs PeerJsConfig, role PublicSessionRole, sessionId string, state string, ) *PublicSessionResponse`

NewPublicSessionResponse instantiates a new PublicSessionResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicSessionResponseWithDefaults

`func NewPublicSessionResponseWithDefaults() *PublicSessionResponse`

NewPublicSessionResponseWithDefaults instantiates a new PublicSessionResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBuildingDisplayName

`func (o *PublicSessionResponse) GetBuildingDisplayName() interface{}`

GetBuildingDisplayName returns the BuildingDisplayName field if non-nil, zero value otherwise.

### GetBuildingDisplayNameOk

`func (o *PublicSessionResponse) GetBuildingDisplayNameOk() (*interface{}, bool)`

GetBuildingDisplayNameOk returns a tuple with the BuildingDisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBuildingDisplayName

`func (o *PublicSessionResponse) SetBuildingDisplayName(v interface{})`

SetBuildingDisplayName sets BuildingDisplayName field to given value.

### HasBuildingDisplayName

`func (o *PublicSessionResponse) HasBuildingDisplayName() bool`

HasBuildingDisplayName returns a boolean if a field has been set.

### SetBuildingDisplayNameNil

`func (o *PublicSessionResponse) SetBuildingDisplayNameNil(b bool)`

 SetBuildingDisplayNameNil sets the value for BuildingDisplayName to be an explicit nil

### UnsetBuildingDisplayName
`func (o *PublicSessionResponse) UnsetBuildingDisplayName()`

UnsetBuildingDisplayName ensures that no value is present for BuildingDisplayName, not even an explicit nil
### GetCallTargetDisplayName

`func (o *PublicSessionResponse) GetCallTargetDisplayName() interface{}`

GetCallTargetDisplayName returns the CallTargetDisplayName field if non-nil, zero value otherwise.

### GetCallTargetDisplayNameOk

`func (o *PublicSessionResponse) GetCallTargetDisplayNameOk() (*interface{}, bool)`

GetCallTargetDisplayNameOk returns a tuple with the CallTargetDisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCallTargetDisplayName

`func (o *PublicSessionResponse) SetCallTargetDisplayName(v interface{})`

SetCallTargetDisplayName sets CallTargetDisplayName field to given value.

### HasCallTargetDisplayName

`func (o *PublicSessionResponse) HasCallTargetDisplayName() bool`

HasCallTargetDisplayName returns a boolean if a field has been set.

### SetCallTargetDisplayNameNil

`func (o *PublicSessionResponse) SetCallTargetDisplayNameNil(b bool)`

 SetCallTargetDisplayNameNil sets the value for CallTargetDisplayName to be an explicit nil

### UnsetCallTargetDisplayName
`func (o *PublicSessionResponse) UnsetCallTargetDisplayName()`

UnsetCallTargetDisplayName ensures that no value is present for CallTargetDisplayName, not even an explicit nil
### GetCallTargetLocationLine

`func (o *PublicSessionResponse) GetCallTargetLocationLine() string`

GetCallTargetLocationLine returns the CallTargetLocationLine field if non-nil, zero value otherwise.

### GetCallTargetLocationLineOk

`func (o *PublicSessionResponse) GetCallTargetLocationLineOk() (*string, bool)`

GetCallTargetLocationLineOk returns a tuple with the CallTargetLocationLine field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCallTargetLocationLine

`func (o *PublicSessionResponse) SetCallTargetLocationLine(v string)`

SetCallTargetLocationLine sets CallTargetLocationLine field to given value.

### HasCallTargetLocationLine

`func (o *PublicSessionResponse) HasCallTargetLocationLine() bool`

HasCallTargetLocationLine returns a boolean if a field has been set.

### SetCallTargetLocationLineNil

`func (o *PublicSessionResponse) SetCallTargetLocationLineNil(b bool)`

 SetCallTargetLocationLineNil sets the value for CallTargetLocationLine to be an explicit nil

### UnsetCallTargetLocationLine
`func (o *PublicSessionResponse) UnsetCallTargetLocationLine()`

UnsetCallTargetLocationLine ensures that no value is present for CallTargetLocationLine, not even an explicit nil
### GetCalleePeerId

`func (o *PublicSessionResponse) GetCalleePeerId() string`

GetCalleePeerId returns the CalleePeerId field if non-nil, zero value otherwise.

### GetCalleePeerIdOk

`func (o *PublicSessionResponse) GetCalleePeerIdOk() (*string, bool)`

GetCalleePeerIdOk returns a tuple with the CalleePeerId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCalleePeerId

`func (o *PublicSessionResponse) SetCalleePeerId(v string)`

SetCalleePeerId sets CalleePeerId field to given value.


### GetCalleesNotified

`func (o *PublicSessionResponse) GetCalleesNotified() bool`

GetCalleesNotified returns the CalleesNotified field if non-nil, zero value otherwise.

### GetCalleesNotifiedOk

`func (o *PublicSessionResponse) GetCalleesNotifiedOk() (*bool, bool)`

GetCalleesNotifiedOk returns a tuple with the CalleesNotified field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCalleesNotified

`func (o *PublicSessionResponse) SetCalleesNotified(v bool)`

SetCalleesNotified sets CalleesNotified field to given value.

### HasCalleesNotified

`func (o *PublicSessionResponse) HasCalleesNotified() bool`

HasCalleesNotified returns a boolean if a field has been set.

### SetCalleesNotifiedNil

`func (o *PublicSessionResponse) SetCalleesNotifiedNil(b bool)`

 SetCalleesNotifiedNil sets the value for CalleesNotified to be an explicit nil

### UnsetCalleesNotified
`func (o *PublicSessionResponse) UnsetCalleesNotified()`

UnsetCalleesNotified ensures that no value is present for CalleesNotified, not even an explicit nil
### GetCallerPeerId

`func (o *PublicSessionResponse) GetCallerPeerId() string`

GetCallerPeerId returns the CallerPeerId field if non-nil, zero value otherwise.

### GetCallerPeerIdOk

`func (o *PublicSessionResponse) GetCallerPeerIdOk() (*string, bool)`

GetCallerPeerIdOk returns a tuple with the CallerPeerId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCallerPeerId

`func (o *PublicSessionResponse) SetCallerPeerId(v string)`

SetCallerPeerId sets CallerPeerId field to given value.


### GetExpiresAt

`func (o *PublicSessionResponse) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *PublicSessionResponse) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *PublicSessionResponse) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *PublicSessionResponse) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *PublicSessionResponse) SetExpiresAtNil(b bool)`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *PublicSessionResponse) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil
### GetHasLights

`func (o *PublicSessionResponse) GetHasLights() bool`

GetHasLights returns the HasLights field if non-nil, zero value otherwise.

### GetHasLightsOk

`func (o *PublicSessionResponse) GetHasLightsOk() (*bool, bool)`

GetHasLightsOk returns a tuple with the HasLights field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasLights

`func (o *PublicSessionResponse) SetHasLights(v bool)`

SetHasLights sets HasLights field to given value.

### HasHasLights

`func (o *PublicSessionResponse) HasHasLights() bool`

HasHasLights returns a boolean if a field has been set.

### SetHasLightsNil

`func (o *PublicSessionResponse) SetHasLightsNil(b bool)`

 SetHasLightsNil sets the value for HasLights to be an explicit nil

### UnsetHasLights
`func (o *PublicSessionResponse) UnsetHasLights()`

UnsetHasLights ensures that no value is present for HasLights, not even an explicit nil
### GetMode

`func (o *PublicSessionResponse) GetMode() string`

GetMode returns the Mode field if non-nil, zero value otherwise.

### GetModeOk

`func (o *PublicSessionResponse) GetModeOk() (*string, bool)`

GetModeOk returns a tuple with the Mode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMode

`func (o *PublicSessionResponse) SetMode(v string)`

SetMode sets Mode field to given value.

### HasMode

`func (o *PublicSessionResponse) HasMode() bool`

HasMode returns a boolean if a field has been set.

### SetModeNil

`func (o *PublicSessionResponse) SetModeNil(b bool)`

 SetModeNil sets the value for Mode to be an explicit nil

### UnsetMode
`func (o *PublicSessionResponse) UnsetMode()`

UnsetMode ensures that no value is present for Mode, not even an explicit nil
### GetPeerjs

`func (o *PublicSessionResponse) GetPeerjs() PeerJsConfig`

GetPeerjs returns the Peerjs field if non-nil, zero value otherwise.

### GetPeerjsOk

`func (o *PublicSessionResponse) GetPeerjsOk() (*PeerJsConfig, bool)`

GetPeerjsOk returns a tuple with the Peerjs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeerjs

`func (o *PublicSessionResponse) SetPeerjs(v PeerJsConfig)`

SetPeerjs sets Peerjs field to given value.


### GetPortalCameraDeviceIds

`func (o *PublicSessionResponse) GetPortalCameraDeviceIds() []string`

GetPortalCameraDeviceIds returns the PortalCameraDeviceIds field if non-nil, zero value otherwise.

### GetPortalCameraDeviceIdsOk

`func (o *PublicSessionResponse) GetPortalCameraDeviceIdsOk() (*[]string, bool)`

GetPortalCameraDeviceIdsOk returns a tuple with the PortalCameraDeviceIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPortalCameraDeviceIds

`func (o *PublicSessionResponse) SetPortalCameraDeviceIds(v []string)`

SetPortalCameraDeviceIds sets PortalCameraDeviceIds field to given value.

### HasPortalCameraDeviceIds

`func (o *PublicSessionResponse) HasPortalCameraDeviceIds() bool`

HasPortalCameraDeviceIds returns a boolean if a field has been set.

### SetPortalCameraDeviceIdsNil

`func (o *PublicSessionResponse) SetPortalCameraDeviceIdsNil(b bool)`

 SetPortalCameraDeviceIdsNil sets the value for PortalCameraDeviceIds to be an explicit nil

### UnsetPortalCameraDeviceIds
`func (o *PublicSessionResponse) UnsetPortalCameraDeviceIds()`

UnsetPortalCameraDeviceIds ensures that no value is present for PortalCameraDeviceIds, not even an explicit nil
### GetRole

`func (o *PublicSessionResponse) GetRole() PublicSessionRole`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *PublicSessionResponse) GetRoleOk() (*PublicSessionRole, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *PublicSessionResponse) SetRole(v PublicSessionRole)`

SetRole sets Role field to given value.


### GetSessionId

`func (o *PublicSessionResponse) GetSessionId() string`

GetSessionId returns the SessionId field if non-nil, zero value otherwise.

### GetSessionIdOk

`func (o *PublicSessionResponse) GetSessionIdOk() (*string, bool)`

GetSessionIdOk returns a tuple with the SessionId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSessionId

`func (o *PublicSessionResponse) SetSessionId(v string)`

SetSessionId sets SessionId field to given value.


### GetState

`func (o *PublicSessionResponse) GetState() string`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *PublicSessionResponse) GetStateOk() (*string, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *PublicSessionResponse) SetState(v string)`

SetState sets State field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
