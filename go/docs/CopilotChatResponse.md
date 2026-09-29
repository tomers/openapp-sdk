# CopilotChatResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CorrelationId** | **string** |  |
**ProposedActions** | [**[]CopilotProposedAction**](CopilotProposedAction.md) |  |
**Replies** | **[]string** |  |

## Methods

### NewCopilotChatResponse

`func NewCopilotChatResponse(correlationId string, proposedActions []CopilotProposedAction, replies []string, ) *CopilotChatResponse`

NewCopilotChatResponse instantiates a new CopilotChatResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCopilotChatResponseWithDefaults

`func NewCopilotChatResponseWithDefaults() *CopilotChatResponse`

NewCopilotChatResponseWithDefaults instantiates a new CopilotChatResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCorrelationId

`func (o *CopilotChatResponse) GetCorrelationId() string`

GetCorrelationId returns the CorrelationId field if non-nil, zero value otherwise.

### GetCorrelationIdOk

`func (o *CopilotChatResponse) GetCorrelationIdOk() (*string, bool)`

GetCorrelationIdOk returns a tuple with the CorrelationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCorrelationId

`func (o *CopilotChatResponse) SetCorrelationId(v string)`

SetCorrelationId sets CorrelationId field to given value.


### GetProposedActions

`func (o *CopilotChatResponse) GetProposedActions() []CopilotProposedAction`

GetProposedActions returns the ProposedActions field if non-nil, zero value otherwise.

### GetProposedActionsOk

`func (o *CopilotChatResponse) GetProposedActionsOk() (*[]CopilotProposedAction, bool)`

GetProposedActionsOk returns a tuple with the ProposedActions field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProposedActions

`func (o *CopilotChatResponse) SetProposedActions(v []CopilotProposedAction)`

SetProposedActions sets ProposedActions field to given value.


### GetReplies

`func (o *CopilotChatResponse) GetReplies() []string`

GetReplies returns the Replies field if non-nil, zero value otherwise.

### GetRepliesOk

`func (o *CopilotChatResponse) GetRepliesOk() (*[]string, bool)`

GetRepliesOk returns a tuple with the Replies field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReplies

`func (o *CopilotChatResponse) SetReplies(v []string)`

SetReplies sets Replies field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
