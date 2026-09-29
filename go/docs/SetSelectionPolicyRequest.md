# SetSelectionPolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Policy** | [**SelectionPolicy**](SelectionPolicy.md) | How excess resources are chosen for the lifecycle: &#x60;newest_first&#x60;, &#x60;oldest_first&#x60;, &#x60;least_recently_used&#x60;, or &#x60;manual&#x60;. |

## Methods

### NewSetSelectionPolicyRequest

`func NewSetSelectionPolicyRequest(policy SelectionPolicy, ) *SetSelectionPolicyRequest`

NewSetSelectionPolicyRequest instantiates a new SetSelectionPolicyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSetSelectionPolicyRequestWithDefaults

`func NewSetSelectionPolicyRequestWithDefaults() *SetSelectionPolicyRequest`

NewSetSelectionPolicyRequestWithDefaults instantiates a new SetSelectionPolicyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPolicy

`func (o *SetSelectionPolicyRequest) GetPolicy() SelectionPolicy`

GetPolicy returns the Policy field if non-nil, zero value otherwise.

### GetPolicyOk

`func (o *SetSelectionPolicyRequest) GetPolicyOk() (*SelectionPolicy, bool)`

GetPolicyOk returns a tuple with the Policy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicy

`func (o *SetSelectionPolicyRequest) SetPolicy(v SelectionPolicy)`

SetPolicy sets Policy field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
