# AcceptEulaResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Message** | **string** |  |
**UserId** | **string** |  |
**WorkspaceId** | Pointer to **NullableString** | Personal Workspace org id auto-created on self-signup. Absent for existing users being linked to Kratos credentials. | [optional]

## Methods

### NewAcceptEulaResponse

`func NewAcceptEulaResponse(message string, userId string, ) *AcceptEulaResponse`

NewAcceptEulaResponse instantiates a new AcceptEulaResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAcceptEulaResponseWithDefaults

`func NewAcceptEulaResponseWithDefaults() *AcceptEulaResponse`

NewAcceptEulaResponseWithDefaults instantiates a new AcceptEulaResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMessage

`func (o *AcceptEulaResponse) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *AcceptEulaResponse) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *AcceptEulaResponse) SetMessage(v string)`

SetMessage sets Message field to given value.


### GetUserId

`func (o *AcceptEulaResponse) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *AcceptEulaResponse) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *AcceptEulaResponse) SetUserId(v string)`

SetUserId sets UserId field to given value.


### GetWorkspaceId

`func (o *AcceptEulaResponse) GetWorkspaceId() string`

GetWorkspaceId returns the WorkspaceId field if non-nil, zero value otherwise.

### GetWorkspaceIdOk

`func (o *AcceptEulaResponse) GetWorkspaceIdOk() (*string, bool)`

GetWorkspaceIdOk returns a tuple with the WorkspaceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWorkspaceId

`func (o *AcceptEulaResponse) SetWorkspaceId(v string)`

SetWorkspaceId sets WorkspaceId field to given value.

### HasWorkspaceId

`func (o *AcceptEulaResponse) HasWorkspaceId() bool`

HasWorkspaceId returns a boolean if a field has been set.

### SetWorkspaceIdNil

`func (o *AcceptEulaResponse) SetWorkspaceIdNil(b bool)`

 SetWorkspaceIdNil sets the value for WorkspaceId to be an explicit nil

### UnsetWorkspaceId
`func (o *AcceptEulaResponse) UnsetWorkspaceId()`

UnsetWorkspaceId ensures that no value is present for WorkspaceId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
