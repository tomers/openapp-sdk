# NotificationPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Channels** | [**[]NotificationChannel**](NotificationChannel.md) | Notification destinations. Only &#x60;audit&#x60; is currently supported. |

## Methods

### NewNotificationPolicyConfig

`func NewNotificationPolicyConfig(channels []NotificationChannel, ) *NotificationPolicyConfig`

NewNotificationPolicyConfig instantiates a new NotificationPolicyConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewNotificationPolicyConfigWithDefaults

`func NewNotificationPolicyConfigWithDefaults() *NotificationPolicyConfig`

NewNotificationPolicyConfigWithDefaults instantiates a new NotificationPolicyConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetChannels

`func (o *NotificationPolicyConfig) GetChannels() []NotificationChannel`

GetChannels returns the Channels field if non-nil, zero value otherwise.

### GetChannelsOk

`func (o *NotificationPolicyConfig) GetChannelsOk() (*[]NotificationChannel, bool)`

GetChannelsOk returns a tuple with the Channels field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChannels

`func (o *NotificationPolicyConfig) SetChannels(v []NotificationChannel)`

SetChannels sets Channels field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
