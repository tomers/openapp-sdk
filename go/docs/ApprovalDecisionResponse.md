# ApprovalDecisionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ApproveCount** | Pointer to **NullableInt32** |  | [optional]
**Id** | **string** |  |
**RequiredCount** | Pointer to **NullableInt32** |  | [optional]
**Status** | **string** |  |

## Methods

### NewApprovalDecisionResponse

`func NewApprovalDecisionResponse(id string, status string, ) *ApprovalDecisionResponse`

NewApprovalDecisionResponse instantiates a new ApprovalDecisionResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApprovalDecisionResponseWithDefaults

`func NewApprovalDecisionResponseWithDefaults() *ApprovalDecisionResponse`

NewApprovalDecisionResponseWithDefaults instantiates a new ApprovalDecisionResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetApproveCount

`func (o *ApprovalDecisionResponse) GetApproveCount() int32`

GetApproveCount returns the ApproveCount field if non-nil, zero value otherwise.

### GetApproveCountOk

`func (o *ApprovalDecisionResponse) GetApproveCountOk() (*int32, bool)`

GetApproveCountOk returns a tuple with the ApproveCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetApproveCount

`func (o *ApprovalDecisionResponse) SetApproveCount(v int32)`

SetApproveCount sets ApproveCount field to given value.

### HasApproveCount

`func (o *ApprovalDecisionResponse) HasApproveCount() bool`

HasApproveCount returns a boolean if a field has been set.

### SetApproveCountNil

`func (o *ApprovalDecisionResponse) SetApproveCountNil(b bool)`

 SetApproveCountNil sets the value for ApproveCount to be an explicit nil

### UnsetApproveCount
`func (o *ApprovalDecisionResponse) UnsetApproveCount()`

UnsetApproveCount ensures that no value is present for ApproveCount, not even an explicit nil
### GetId

`func (o *ApprovalDecisionResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApprovalDecisionResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApprovalDecisionResponse) SetId(v string)`

SetId sets Id field to given value.


### GetRequiredCount

`func (o *ApprovalDecisionResponse) GetRequiredCount() int32`

GetRequiredCount returns the RequiredCount field if non-nil, zero value otherwise.

### GetRequiredCountOk

`func (o *ApprovalDecisionResponse) GetRequiredCountOk() (*int32, bool)`

GetRequiredCountOk returns a tuple with the RequiredCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequiredCount

`func (o *ApprovalDecisionResponse) SetRequiredCount(v int32)`

SetRequiredCount sets RequiredCount field to given value.

### HasRequiredCount

`func (o *ApprovalDecisionResponse) HasRequiredCount() bool`

HasRequiredCount returns a boolean if a field has been set.

### SetRequiredCountNil

`func (o *ApprovalDecisionResponse) SetRequiredCountNil(b bool)`

 SetRequiredCountNil sets the value for RequiredCount to be an explicit nil

### UnsetRequiredCount
`func (o *ApprovalDecisionResponse) UnsetRequiredCount()`

UnsetRequiredCount ensures that no value is present for RequiredCount, not even an explicit nil
### GetStatus

`func (o *ApprovalDecisionResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *ApprovalDecisionResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *ApprovalDecisionResponse) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
