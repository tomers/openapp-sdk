# MePushSubscriptionStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**HasAnyNotificationChannel** | **bool** | Convenience: &#x60;has_subscription || has_device_token&#x60;. |
**HasDeviceToken** | **bool** | True when at least one native device token (APNs / FCM) is registered. |
**HasSubscription** | **bool** | True when at least one Web Push subscription exists for this browser/device flow. |
**PushConfigured** | **bool** | True when VAPID is configured; false means Web Push call notifications cannot be sent. |

## Methods

### NewMePushSubscriptionStatusResponse

`func NewMePushSubscriptionStatusResponse(hasAnyNotificationChannel bool, hasDeviceToken bool, hasSubscription bool, pushConfigured bool, ) *MePushSubscriptionStatusResponse`

NewMePushSubscriptionStatusResponse instantiates a new MePushSubscriptionStatusResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMePushSubscriptionStatusResponseWithDefaults

`func NewMePushSubscriptionStatusResponseWithDefaults() *MePushSubscriptionStatusResponse`

NewMePushSubscriptionStatusResponseWithDefaults instantiates a new MePushSubscriptionStatusResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetHasAnyNotificationChannel

`func (o *MePushSubscriptionStatusResponse) GetHasAnyNotificationChannel() bool`

GetHasAnyNotificationChannel returns the HasAnyNotificationChannel field if non-nil, zero value otherwise.

### GetHasAnyNotificationChannelOk

`func (o *MePushSubscriptionStatusResponse) GetHasAnyNotificationChannelOk() (*bool, bool)`

GetHasAnyNotificationChannelOk returns a tuple with the HasAnyNotificationChannel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasAnyNotificationChannel

`func (o *MePushSubscriptionStatusResponse) SetHasAnyNotificationChannel(v bool)`

SetHasAnyNotificationChannel sets HasAnyNotificationChannel field to given value.


### GetHasDeviceToken

`func (o *MePushSubscriptionStatusResponse) GetHasDeviceToken() bool`

GetHasDeviceToken returns the HasDeviceToken field if non-nil, zero value otherwise.

### GetHasDeviceTokenOk

`func (o *MePushSubscriptionStatusResponse) GetHasDeviceTokenOk() (*bool, bool)`

GetHasDeviceTokenOk returns a tuple with the HasDeviceToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasDeviceToken

`func (o *MePushSubscriptionStatusResponse) SetHasDeviceToken(v bool)`

SetHasDeviceToken sets HasDeviceToken field to given value.


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
