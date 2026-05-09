# BillingWebhookResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Events** | [**[]BillingEvent**](BillingEvent.md) |  |
**Provider** | **string** |  |

## Methods

### NewBillingWebhookResponse

`func NewBillingWebhookResponse(events []BillingEvent, provider string, ) *BillingWebhookResponse`

NewBillingWebhookResponse instantiates a new BillingWebhookResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewBillingWebhookResponseWithDefaults

`func NewBillingWebhookResponseWithDefaults() *BillingWebhookResponse`

NewBillingWebhookResponseWithDefaults instantiates a new BillingWebhookResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEvents

`func (o *BillingWebhookResponse) GetEvents() []BillingEvent`

GetEvents returns the Events field if non-nil, zero value otherwise.

### GetEventsOk

`func (o *BillingWebhookResponse) GetEventsOk() (*[]BillingEvent, bool)`

GetEventsOk returns a tuple with the Events field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEvents

`func (o *BillingWebhookResponse) SetEvents(v []BillingEvent)`

SetEvents sets Events field to given value.


### GetProvider

`func (o *BillingWebhookResponse) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *BillingWebhookResponse) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *BillingWebhookResponse) SetProvider(v string)`

SetProvider sets Provider field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
