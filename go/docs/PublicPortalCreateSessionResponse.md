# PublicPortalCreateSessionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CalleePeerId** | **string** |  |
**CalleesNotified** | **bool** | True when at least one callee had a push subscription so notification was sent (or attempted). |
**CallerPeerId** | **string** |  |
**CallerToken** | **string** |  |
**ExpiresAt** | **string** |  |
**Peerjs** | [**PeerJsConfig**](PeerJsConfig.md) |  |
**SessionId** | **string** |  |

## Methods

### NewPublicPortalCreateSessionResponse

`func NewPublicPortalCreateSessionResponse(calleePeerId string, calleesNotified bool, callerPeerId string, callerToken string, expiresAt string, peerjs PeerJsConfig, sessionId string, ) *PublicPortalCreateSessionResponse`

NewPublicPortalCreateSessionResponse instantiates a new PublicPortalCreateSessionResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicPortalCreateSessionResponseWithDefaults

`func NewPublicPortalCreateSessionResponseWithDefaults() *PublicPortalCreateSessionResponse`

NewPublicPortalCreateSessionResponseWithDefaults instantiates a new PublicPortalCreateSessionResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCalleePeerId

`func (o *PublicPortalCreateSessionResponse) GetCalleePeerId() string`

GetCalleePeerId returns the CalleePeerId field if non-nil, zero value otherwise.

### GetCalleePeerIdOk

`func (o *PublicPortalCreateSessionResponse) GetCalleePeerIdOk() (*string, bool)`

GetCalleePeerIdOk returns a tuple with the CalleePeerId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCalleePeerId

`func (o *PublicPortalCreateSessionResponse) SetCalleePeerId(v string)`

SetCalleePeerId sets CalleePeerId field to given value.


### GetCalleesNotified

`func (o *PublicPortalCreateSessionResponse) GetCalleesNotified() bool`

GetCalleesNotified returns the CalleesNotified field if non-nil, zero value otherwise.

### GetCalleesNotifiedOk

`func (o *PublicPortalCreateSessionResponse) GetCalleesNotifiedOk() (*bool, bool)`

GetCalleesNotifiedOk returns a tuple with the CalleesNotified field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCalleesNotified

`func (o *PublicPortalCreateSessionResponse) SetCalleesNotified(v bool)`

SetCalleesNotified sets CalleesNotified field to given value.


### GetCallerPeerId

`func (o *PublicPortalCreateSessionResponse) GetCallerPeerId() string`

GetCallerPeerId returns the CallerPeerId field if non-nil, zero value otherwise.

### GetCallerPeerIdOk

`func (o *PublicPortalCreateSessionResponse) GetCallerPeerIdOk() (*string, bool)`

GetCallerPeerIdOk returns a tuple with the CallerPeerId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCallerPeerId

`func (o *PublicPortalCreateSessionResponse) SetCallerPeerId(v string)`

SetCallerPeerId sets CallerPeerId field to given value.


### GetCallerToken

`func (o *PublicPortalCreateSessionResponse) GetCallerToken() string`

GetCallerToken returns the CallerToken field if non-nil, zero value otherwise.

### GetCallerTokenOk

`func (o *PublicPortalCreateSessionResponse) GetCallerTokenOk() (*string, bool)`

GetCallerTokenOk returns a tuple with the CallerToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCallerToken

`func (o *PublicPortalCreateSessionResponse) SetCallerToken(v string)`

SetCallerToken sets CallerToken field to given value.


### GetExpiresAt

`func (o *PublicPortalCreateSessionResponse) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *PublicPortalCreateSessionResponse) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *PublicPortalCreateSessionResponse) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.


### GetPeerjs

`func (o *PublicPortalCreateSessionResponse) GetPeerjs() PeerJsConfig`

GetPeerjs returns the Peerjs field if non-nil, zero value otherwise.

### GetPeerjsOk

`func (o *PublicPortalCreateSessionResponse) GetPeerjsOk() (*PeerJsConfig, bool)`

GetPeerjsOk returns a tuple with the Peerjs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeerjs

`func (o *PublicPortalCreateSessionResponse) SetPeerjs(v PeerJsConfig)`

SetPeerjs sets Peerjs field to given value.


### GetSessionId

`func (o *PublicPortalCreateSessionResponse) GetSessionId() string`

GetSessionId returns the SessionId field if non-nil, zero value otherwise.

### GetSessionIdOk

`func (o *PublicPortalCreateSessionResponse) GetSessionIdOk() (*string, bool)`

GetSessionIdOk returns a tuple with the SessionId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSessionId

`func (o *PublicPortalCreateSessionResponse) SetSessionId(v string)`

SetSessionId sets SessionId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
