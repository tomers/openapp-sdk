# ApprovalRequestResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ApproveCount** | Pointer to **NullableInt32** |  | [optional]
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**Id** | **string** |  |
**Note** | Pointer to **NullableString** |  | [optional]
**OrgId** | **string** |  |
**PolicyId** | **string** |  |
**RequesterUserId** | **string** |  |
**RequiredCount** | Pointer to **NullableInt32** |  | [optional]
**Status** | Pointer to **NullableString** |  | [optional]
**SubjectKind** | Pointer to **NullableString** | What the request is about, e.g. &#x60;invite&#x60;. | [optional]
**SubjectLabel** | Pointer to **NullableString** | Human-readable subject (invite name), not the raw kind/ref. | [optional]
**SubjectRef** | Pointer to **NullableString** | Id of the subject (e.g. the disabled invite link awaiting approval). | [optional]

## Methods

### NewApprovalRequestResponse

`func NewApprovalRequestResponse(id string, orgId string, policyId string, requesterUserId string, ) *ApprovalRequestResponse`

NewApprovalRequestResponse instantiates a new ApprovalRequestResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApprovalRequestResponseWithDefaults

`func NewApprovalRequestResponseWithDefaults() *ApprovalRequestResponse`

NewApprovalRequestResponseWithDefaults instantiates a new ApprovalRequestResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetApproveCount

`func (o *ApprovalRequestResponse) GetApproveCount() int32`

GetApproveCount returns the ApproveCount field if non-nil, zero value otherwise.

### GetApproveCountOk

`func (o *ApprovalRequestResponse) GetApproveCountOk() (*int32, bool)`

GetApproveCountOk returns a tuple with the ApproveCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetApproveCount

`func (o *ApprovalRequestResponse) SetApproveCount(v int32)`

SetApproveCount sets ApproveCount field to given value.

### HasApproveCount

`func (o *ApprovalRequestResponse) HasApproveCount() bool`

HasApproveCount returns a boolean if a field has been set.

### SetApproveCountNil

`func (o *ApprovalRequestResponse) SetApproveCountNil(b bool)`

 SetApproveCountNil sets the value for ApproveCount to be an explicit nil

### UnsetApproveCount
`func (o *ApprovalRequestResponse) UnsetApproveCount()`

UnsetApproveCount ensures that no value is present for ApproveCount, not even an explicit nil
### GetCreatedAt

`func (o *ApprovalRequestResponse) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ApprovalRequestResponse) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ApprovalRequestResponse) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *ApprovalRequestResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *ApprovalRequestResponse) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *ApprovalRequestResponse) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetId

`func (o *ApprovalRequestResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApprovalRequestResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApprovalRequestResponse) SetId(v string)`

SetId sets Id field to given value.


### GetNote

`func (o *ApprovalRequestResponse) GetNote() string`

GetNote returns the Note field if non-nil, zero value otherwise.

### GetNoteOk

`func (o *ApprovalRequestResponse) GetNoteOk() (*string, bool)`

GetNoteOk returns a tuple with the Note field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNote

`func (o *ApprovalRequestResponse) SetNote(v string)`

SetNote sets Note field to given value.

### HasNote

`func (o *ApprovalRequestResponse) HasNote() bool`

HasNote returns a boolean if a field has been set.

### SetNoteNil

`func (o *ApprovalRequestResponse) SetNoteNil(b bool)`

 SetNoteNil sets the value for Note to be an explicit nil

### UnsetNote
`func (o *ApprovalRequestResponse) UnsetNote()`

UnsetNote ensures that no value is present for Note, not even an explicit nil
### GetOrgId

`func (o *ApprovalRequestResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *ApprovalRequestResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *ApprovalRequestResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetPolicyId

`func (o *ApprovalRequestResponse) GetPolicyId() string`

GetPolicyId returns the PolicyId field if non-nil, zero value otherwise.

### GetPolicyIdOk

`func (o *ApprovalRequestResponse) GetPolicyIdOk() (*string, bool)`

GetPolicyIdOk returns a tuple with the PolicyId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyId

`func (o *ApprovalRequestResponse) SetPolicyId(v string)`

SetPolicyId sets PolicyId field to given value.


### GetRequesterUserId

`func (o *ApprovalRequestResponse) GetRequesterUserId() string`

GetRequesterUserId returns the RequesterUserId field if non-nil, zero value otherwise.

### GetRequesterUserIdOk

`func (o *ApprovalRequestResponse) GetRequesterUserIdOk() (*string, bool)`

GetRequesterUserIdOk returns a tuple with the RequesterUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequesterUserId

`func (o *ApprovalRequestResponse) SetRequesterUserId(v string)`

SetRequesterUserId sets RequesterUserId field to given value.


### GetRequiredCount

`func (o *ApprovalRequestResponse) GetRequiredCount() int32`

GetRequiredCount returns the RequiredCount field if non-nil, zero value otherwise.

### GetRequiredCountOk

`func (o *ApprovalRequestResponse) GetRequiredCountOk() (*int32, bool)`

GetRequiredCountOk returns a tuple with the RequiredCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequiredCount

`func (o *ApprovalRequestResponse) SetRequiredCount(v int32)`

SetRequiredCount sets RequiredCount field to given value.

### HasRequiredCount

`func (o *ApprovalRequestResponse) HasRequiredCount() bool`

HasRequiredCount returns a boolean if a field has been set.

### SetRequiredCountNil

`func (o *ApprovalRequestResponse) SetRequiredCountNil(b bool)`

 SetRequiredCountNil sets the value for RequiredCount to be an explicit nil

### UnsetRequiredCount
`func (o *ApprovalRequestResponse) UnsetRequiredCount()`

UnsetRequiredCount ensures that no value is present for RequiredCount, not even an explicit nil
### GetStatus

`func (o *ApprovalRequestResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *ApprovalRequestResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *ApprovalRequestResponse) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *ApprovalRequestResponse) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### SetStatusNil

`func (o *ApprovalRequestResponse) SetStatusNil(b bool)`

 SetStatusNil sets the value for Status to be an explicit nil

### UnsetStatus
`func (o *ApprovalRequestResponse) UnsetStatus()`

UnsetStatus ensures that no value is present for Status, not even an explicit nil
### GetSubjectKind

`func (o *ApprovalRequestResponse) GetSubjectKind() string`

GetSubjectKind returns the SubjectKind field if non-nil, zero value otherwise.

### GetSubjectKindOk

`func (o *ApprovalRequestResponse) GetSubjectKindOk() (*string, bool)`

GetSubjectKindOk returns a tuple with the SubjectKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSubjectKind

`func (o *ApprovalRequestResponse) SetSubjectKind(v string)`

SetSubjectKind sets SubjectKind field to given value.

### HasSubjectKind

`func (o *ApprovalRequestResponse) HasSubjectKind() bool`

HasSubjectKind returns a boolean if a field has been set.

### SetSubjectKindNil

`func (o *ApprovalRequestResponse) SetSubjectKindNil(b bool)`

 SetSubjectKindNil sets the value for SubjectKind to be an explicit nil

### UnsetSubjectKind
`func (o *ApprovalRequestResponse) UnsetSubjectKind()`

UnsetSubjectKind ensures that no value is present for SubjectKind, not even an explicit nil
### GetSubjectLabel

`func (o *ApprovalRequestResponse) GetSubjectLabel() string`

GetSubjectLabel returns the SubjectLabel field if non-nil, zero value otherwise.

### GetSubjectLabelOk

`func (o *ApprovalRequestResponse) GetSubjectLabelOk() (*string, bool)`

GetSubjectLabelOk returns a tuple with the SubjectLabel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSubjectLabel

`func (o *ApprovalRequestResponse) SetSubjectLabel(v string)`

SetSubjectLabel sets SubjectLabel field to given value.

### HasSubjectLabel

`func (o *ApprovalRequestResponse) HasSubjectLabel() bool`

HasSubjectLabel returns a boolean if a field has been set.

### SetSubjectLabelNil

`func (o *ApprovalRequestResponse) SetSubjectLabelNil(b bool)`

 SetSubjectLabelNil sets the value for SubjectLabel to be an explicit nil

### UnsetSubjectLabel
`func (o *ApprovalRequestResponse) UnsetSubjectLabel()`

UnsetSubjectLabel ensures that no value is present for SubjectLabel, not even an explicit nil
### GetSubjectRef

`func (o *ApprovalRequestResponse) GetSubjectRef() string`

GetSubjectRef returns the SubjectRef field if non-nil, zero value otherwise.

### GetSubjectRefOk

`func (o *ApprovalRequestResponse) GetSubjectRefOk() (*string, bool)`

GetSubjectRefOk returns a tuple with the SubjectRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSubjectRef

`func (o *ApprovalRequestResponse) SetSubjectRef(v string)`

SetSubjectRef sets SubjectRef field to given value.

### HasSubjectRef

`func (o *ApprovalRequestResponse) HasSubjectRef() bool`

HasSubjectRef returns a boolean if a field has been set.

### SetSubjectRefNil

`func (o *ApprovalRequestResponse) SetSubjectRefNil(b bool)`

 SetSubjectRefNil sets the value for SubjectRef to be an explicit nil

### UnsetSubjectRef
`func (o *ApprovalRequestResponse) UnsetSubjectRef()`

UnsetSubjectRef ensures that no value is present for SubjectRef, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
