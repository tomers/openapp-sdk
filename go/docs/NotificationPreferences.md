# NotificationPreferences

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Push** | **bool** | Send notices as push notifications to the user&#39;s registered browsers and devices. |
**Sms** | **bool** | Send notices to the phone on file as text messages when WhatsApp did not deliver them. |
**Whatsapp** | **bool** | Send notices to the phone on file over WhatsApp. |

## Methods

### NewNotificationPreferences

`func NewNotificationPreferences(push bool, sms bool, whatsapp bool, ) *NotificationPreferences`

NewNotificationPreferences instantiates a new NotificationPreferences object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewNotificationPreferencesWithDefaults

`func NewNotificationPreferencesWithDefaults() *NotificationPreferences`

NewNotificationPreferencesWithDefaults instantiates a new NotificationPreferences object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPush

`func (o *NotificationPreferences) GetPush() bool`

GetPush returns the Push field if non-nil, zero value otherwise.

### GetPushOk

`func (o *NotificationPreferences) GetPushOk() (*bool, bool)`

GetPushOk returns a tuple with the Push field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPush

`func (o *NotificationPreferences) SetPush(v bool)`

SetPush sets Push field to given value.


### GetSms

`func (o *NotificationPreferences) GetSms() bool`

GetSms returns the Sms field if non-nil, zero value otherwise.

### GetSmsOk

`func (o *NotificationPreferences) GetSmsOk() (*bool, bool)`

GetSmsOk returns a tuple with the Sms field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSms

`func (o *NotificationPreferences) SetSms(v bool)`

SetSms sets Sms field to given value.


### GetWhatsapp

`func (o *NotificationPreferences) GetWhatsapp() bool`

GetWhatsapp returns the Whatsapp field if non-nil, zero value otherwise.

### GetWhatsappOk

`func (o *NotificationPreferences) GetWhatsappOk() (*bool, bool)`

GetWhatsappOk returns a tuple with the Whatsapp field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWhatsapp

`func (o *NotificationPreferences) SetWhatsapp(v bool)`

SetWhatsapp sets Whatsapp field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
