# MeWorkspaceEnsureResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Created** | **bool** | True when this call had to provision the workspace; false when an existing admin org was reused. |
**WorkspaceId** | **string** | Org id of the workspace the user is admin of. |

## Methods

### NewMeWorkspaceEnsureResponse

`func NewMeWorkspaceEnsureResponse(created bool, workspaceId string, ) *MeWorkspaceEnsureResponse`

NewMeWorkspaceEnsureResponse instantiates a new MeWorkspaceEnsureResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMeWorkspaceEnsureResponseWithDefaults

`func NewMeWorkspaceEnsureResponseWithDefaults() *MeWorkspaceEnsureResponse`

NewMeWorkspaceEnsureResponseWithDefaults instantiates a new MeWorkspaceEnsureResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreated

`func (o *MeWorkspaceEnsureResponse) GetCreated() bool`

GetCreated returns the Created field if non-nil, zero value otherwise.

### GetCreatedOk

`func (o *MeWorkspaceEnsureResponse) GetCreatedOk() (*bool, bool)`

GetCreatedOk returns a tuple with the Created field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreated

`func (o *MeWorkspaceEnsureResponse) SetCreated(v bool)`

SetCreated sets Created field to given value.


### GetWorkspaceId

`func (o *MeWorkspaceEnsureResponse) GetWorkspaceId() string`

GetWorkspaceId returns the WorkspaceId field if non-nil, zero value otherwise.

### GetWorkspaceIdOk

`func (o *MeWorkspaceEnsureResponse) GetWorkspaceIdOk() (*string, bool)`

GetWorkspaceIdOk returns a tuple with the WorkspaceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWorkspaceId

`func (o *MeWorkspaceEnsureResponse) SetWorkspaceId(v string)`

SetWorkspaceId sets WorkspaceId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
