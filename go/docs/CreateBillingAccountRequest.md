# CreateBillingAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Name** | [**LocalizedString**](LocalizedString.md) |  |

## Methods

### NewCreateBillingAccountRequest

`func NewCreateBillingAccountRequest(name LocalizedString, ) *CreateBillingAccountRequest`

NewCreateBillingAccountRequest instantiates a new CreateBillingAccountRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateBillingAccountRequestWithDefaults

`func NewCreateBillingAccountRequestWithDefaults() *CreateBillingAccountRequest`

NewCreateBillingAccountRequestWithDefaults instantiates a new CreateBillingAccountRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetName

`func (o *CreateBillingAccountRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateBillingAccountRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateBillingAccountRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
