# MePushSubscriptionStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**HasSubscription** | **bool** |  |
**PushConfigured** | **bool** | True when VAPID is configured; false means call notifications cannot be sent. |

## Methods

### NewMePushSubscriptionStatusResponse

`func NewMePushSubscriptionStatusResponse(hasSubscription bool, pushConfigured bool, ) *MePushSubscriptionStatusResponse`

NewMePushSubscriptionStatusResponse instantiates a new MePushSubscriptionStatusResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMePushSubscriptionStatusResponseWithDefaults

`func NewMePushSubscriptionStatusResponseWithDefaults() *MePushSubscriptionStatusResponse`

NewMePushSubscriptionStatusResponseWithDefaults instantiates a new MePushSubscriptionStatusResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetHasSubscription

`func (o *MePushSubscriptionStatusResponse) GetHasSubscription() bool`

GetHasSubscription returns the HasSubscription field if non-nil, zero value otherwise.

### GetHasSubscriptionOk

`func (o *MePushSubscriptionStatusResponse) GetHasSubscriptionOk() (*bool, bool)`

GetHasSubscriptionOk returns a tuple with the HasSubscription field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasSubscription

`func (o *MePushSubscriptionStatusResponse) SetHasSubscription(v bool)`

SetHasSubscription sets HasSubscription field to given value.


### GetPushConfigured

`func (o *MePushSubscriptionStatusResponse) GetPushConfigured() bool`

GetPushConfigured returns the PushConfigured field if non-nil, zero value otherwise.

### GetPushConfiguredOk

`func (o *MePushSubscriptionStatusResponse) GetPushConfiguredOk() (*bool, bool)`

GetPushConfiguredOk returns a tuple with the PushConfigured field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPushConfigured

`func (o *MePushSubscriptionStatusResponse) SetPushConfigured(v bool)`

SetPushConfigured sets PushConfigured field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
