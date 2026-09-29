# GroupProposalResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**GroupId** | **string** |  |
**OrgId** | **string** |  |
**PendingMembers** | **int32** | Members waiting for this org to admit them. |
**Status** | **string** | pending, active, or revoked. |

## Methods

### NewGroupProposalResponse

`func NewGroupProposalResponse(groupId string, orgId string, pendingMembers int32, status string, ) *GroupProposalResponse`

NewGroupProposalResponse instantiates a new GroupProposalResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewGroupProposalResponseWithDefaults

`func NewGroupProposalResponseWithDefaults() *GroupProposalResponse`

NewGroupProposalResponseWithDefaults instantiates a new GroupProposalResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetGroupId

`func (o *GroupProposalResponse) GetGroupId() string`

GetGroupId returns the GroupId field if non-nil, zero value otherwise.

### GetGroupIdOk

`func (o *GroupProposalResponse) GetGroupIdOk() (*string, bool)`

GetGroupIdOk returns a tuple with the GroupId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGroupId

`func (o *GroupProposalResponse) SetGroupId(v string)`

SetGroupId sets GroupId field to given value.


### GetOrgId

`func (o *GroupProposalResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *GroupProposalResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *GroupProposalResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetPendingMembers

`func (o *GroupProposalResponse) GetPendingMembers() int32`

GetPendingMembers returns the PendingMembers field if non-nil, zero value otherwise.

### GetPendingMembersOk

`func (o *GroupProposalResponse) GetPendingMembersOk() (*int32, bool)`

GetPendingMembersOk returns a tuple with the PendingMembers field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPendingMembers

`func (o *GroupProposalResponse) SetPendingMembers(v int32)`

SetPendingMembers sets PendingMembers field to given value.


### GetStatus

`func (o *GroupProposalResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *GroupProposalResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *GroupProposalResponse) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
