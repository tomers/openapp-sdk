# MeProfileResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Email** | Pointer to **NullableString** |  | [optional]
**EmailVerified** | **bool** |  |
**IdpImageThumbUrl** | Pointer to **NullableString** |  | [optional]
**IdpImageUrl** | Pointer to **NullableString** | OpenApp-hosted preview of the last imported identity-provider portrait. | [optional]
**IdpLinks** | [**[]IdpLinkResponse**](IdpLinkResponse.md) |  |
**ImageSyncError** | Pointer to **NullableString** |  | [optional]
**ImageThumbUrl** | Pointer to **NullableString** |  | [optional]
**ImageUrl** | Pointer to **NullableString** | Portrait others see, from the selected photo choice (&#x60;idp&#x60;, &#x60;openapp&#x60;, or &#x60;none&#x60;). | [optional]
**Locale** | Pointer to **NullableString** | Explicit preferred locale: a supported locale code, or &#x60;null&#x60; to use the app language. Messages use it ahead of &#x60;observed_locale&#x60;. | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) | Display name as a locale-to-string map. May be empty when the account has no name yet. |
**NameConfirmationPending** | **bool** | True while the account still owes a name confirmation after accepting an invitation: the name is the inviter&#39;s spelling, or there is none. Clears once a name is saved through &#x60;PATCH /me/profile&#x60;, or when an identity provider supplies the name. |
**ObservedLocale** | Pointer to **NullableString** | Last app/device language reported through &#x60;PUT /me/locale/observed&#x60;. Read-only here. | [optional]
**OpenappImageThumbUrl** | Pointer to **NullableString** |  | [optional]
**OpenappImageUrl** | Pointer to **NullableString** | OpenApp-hosted preview of the optional account override portrait. | [optional]
**Phone** | Pointer to **NullableString** |  | [optional]
**PhoneVerified** | **bool** |  |
**ProfileSources** | [**ProfileSourcesResponse**](ProfileSourcesResponse.md) |  |

## Methods

### NewMeProfileResponse

`func NewMeProfileResponse(emailVerified bool, idpLinks []IdpLinkResponse, name LocalizedString, nameConfirmationPending bool, phoneVerified bool, profileSources ProfileSourcesResponse, ) *MeProfileResponse`

NewMeProfileResponse instantiates a new MeProfileResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMeProfileResponseWithDefaults

`func NewMeProfileResponseWithDefaults() *MeProfileResponse`

NewMeProfileResponseWithDefaults instantiates a new MeProfileResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEmail

`func (o *MeProfileResponse) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *MeProfileResponse) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *MeProfileResponse) SetEmail(v string)`

SetEmail sets Email field to given value.

### HasEmail

`func (o *MeProfileResponse) HasEmail() bool`

HasEmail returns a boolean if a field has been set.

### SetEmailNil

`func (o *MeProfileResponse) SetEmailNil(b bool)`

 SetEmailNil sets the value for Email to be an explicit nil

### UnsetEmail
`func (o *MeProfileResponse) UnsetEmail()`

UnsetEmail ensures that no value is present for Email, not even an explicit nil
### GetEmailVerified

`func (o *MeProfileResponse) GetEmailVerified() bool`

GetEmailVerified returns the EmailVerified field if non-nil, zero value otherwise.

### GetEmailVerifiedOk

`func (o *MeProfileResponse) GetEmailVerifiedOk() (*bool, bool)`

GetEmailVerifiedOk returns a tuple with the EmailVerified field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmailVerified

`func (o *MeProfileResponse) SetEmailVerified(v bool)`

SetEmailVerified sets EmailVerified field to given value.


### GetIdpImageThumbUrl

`func (o *MeProfileResponse) GetIdpImageThumbUrl() string`

GetIdpImageThumbUrl returns the IdpImageThumbUrl field if non-nil, zero value otherwise.

### GetIdpImageThumbUrlOk

`func (o *MeProfileResponse) GetIdpImageThumbUrlOk() (*string, bool)`

GetIdpImageThumbUrlOk returns a tuple with the IdpImageThumbUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIdpImageThumbUrl

`func (o *MeProfileResponse) SetIdpImageThumbUrl(v string)`

SetIdpImageThumbUrl sets IdpImageThumbUrl field to given value.

### HasIdpImageThumbUrl

`func (o *MeProfileResponse) HasIdpImageThumbUrl() bool`

HasIdpImageThumbUrl returns a boolean if a field has been set.

### SetIdpImageThumbUrlNil

`func (o *MeProfileResponse) SetIdpImageThumbUrlNil(b bool)`

 SetIdpImageThumbUrlNil sets the value for IdpImageThumbUrl to be an explicit nil

### UnsetIdpImageThumbUrl
`func (o *MeProfileResponse) UnsetIdpImageThumbUrl()`

UnsetIdpImageThumbUrl ensures that no value is present for IdpImageThumbUrl, not even an explicit nil
### GetIdpImageUrl

`func (o *MeProfileResponse) GetIdpImageUrl() string`

GetIdpImageUrl returns the IdpImageUrl field if non-nil, zero value otherwise.

### GetIdpImageUrlOk

`func (o *MeProfileResponse) GetIdpImageUrlOk() (*string, bool)`

GetIdpImageUrlOk returns a tuple with the IdpImageUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIdpImageUrl

`func (o *MeProfileResponse) SetIdpImageUrl(v string)`

SetIdpImageUrl sets IdpImageUrl field to given value.

### HasIdpImageUrl

`func (o *MeProfileResponse) HasIdpImageUrl() bool`

HasIdpImageUrl returns a boolean if a field has been set.

### SetIdpImageUrlNil

`func (o *MeProfileResponse) SetIdpImageUrlNil(b bool)`

 SetIdpImageUrlNil sets the value for IdpImageUrl to be an explicit nil

### UnsetIdpImageUrl
`func (o *MeProfileResponse) UnsetIdpImageUrl()`

UnsetIdpImageUrl ensures that no value is present for IdpImageUrl, not even an explicit nil
### GetIdpLinks

`func (o *MeProfileResponse) GetIdpLinks() []IdpLinkResponse`

GetIdpLinks returns the IdpLinks field if non-nil, zero value otherwise.

### GetIdpLinksOk

`func (o *MeProfileResponse) GetIdpLinksOk() (*[]IdpLinkResponse, bool)`

GetIdpLinksOk returns a tuple with the IdpLinks field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIdpLinks

`func (o *MeProfileResponse) SetIdpLinks(v []IdpLinkResponse)`

SetIdpLinks sets IdpLinks field to given value.


### GetImageSyncError

`func (o *MeProfileResponse) GetImageSyncError() string`

GetImageSyncError returns the ImageSyncError field if non-nil, zero value otherwise.

### GetImageSyncErrorOk

`func (o *MeProfileResponse) GetImageSyncErrorOk() (*string, bool)`

GetImageSyncErrorOk returns a tuple with the ImageSyncError field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageSyncError

`func (o *MeProfileResponse) SetImageSyncError(v string)`

SetImageSyncError sets ImageSyncError field to given value.

### HasImageSyncError

`func (o *MeProfileResponse) HasImageSyncError() bool`

HasImageSyncError returns a boolean if a field has been set.

### SetImageSyncErrorNil

`func (o *MeProfileResponse) SetImageSyncErrorNil(b bool)`

 SetImageSyncErrorNil sets the value for ImageSyncError to be an explicit nil

### UnsetImageSyncError
`func (o *MeProfileResponse) UnsetImageSyncError()`

UnsetImageSyncError ensures that no value is present for ImageSyncError, not even an explicit nil
### GetImageThumbUrl

`func (o *MeProfileResponse) GetImageThumbUrl() string`

GetImageThumbUrl returns the ImageThumbUrl field if non-nil, zero value otherwise.

### GetImageThumbUrlOk

`func (o *MeProfileResponse) GetImageThumbUrlOk() (*string, bool)`

GetImageThumbUrlOk returns a tuple with the ImageThumbUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageThumbUrl

`func (o *MeProfileResponse) SetImageThumbUrl(v string)`

SetImageThumbUrl sets ImageThumbUrl field to given value.

### HasImageThumbUrl

`func (o *MeProfileResponse) HasImageThumbUrl() bool`

HasImageThumbUrl returns a boolean if a field has been set.

### SetImageThumbUrlNil

`func (o *MeProfileResponse) SetImageThumbUrlNil(b bool)`

 SetImageThumbUrlNil sets the value for ImageThumbUrl to be an explicit nil

### UnsetImageThumbUrl
`func (o *MeProfileResponse) UnsetImageThumbUrl()`

UnsetImageThumbUrl ensures that no value is present for ImageThumbUrl, not even an explicit nil
### GetImageUrl

`func (o *MeProfileResponse) GetImageUrl() string`

GetImageUrl returns the ImageUrl field if non-nil, zero value otherwise.

### GetImageUrlOk

`func (o *MeProfileResponse) GetImageUrlOk() (*string, bool)`

GetImageUrlOk returns a tuple with the ImageUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageUrl

`func (o *MeProfileResponse) SetImageUrl(v string)`

SetImageUrl sets ImageUrl field to given value.

### HasImageUrl

`func (o *MeProfileResponse) HasImageUrl() bool`

HasImageUrl returns a boolean if a field has been set.

### SetImageUrlNil

`func (o *MeProfileResponse) SetImageUrlNil(b bool)`

 SetImageUrlNil sets the value for ImageUrl to be an explicit nil

### UnsetImageUrl
`func (o *MeProfileResponse) UnsetImageUrl()`

UnsetImageUrl ensures that no value is present for ImageUrl, not even an explicit nil
### GetLocale

`func (o *MeProfileResponse) GetLocale() string`

GetLocale returns the Locale field if non-nil, zero value otherwise.

### GetLocaleOk

`func (o *MeProfileResponse) GetLocaleOk() (*string, bool)`

GetLocaleOk returns a tuple with the Locale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocale

`func (o *MeProfileResponse) SetLocale(v string)`

SetLocale sets Locale field to given value.

### HasLocale

`func (o *MeProfileResponse) HasLocale() bool`

HasLocale returns a boolean if a field has been set.

### SetLocaleNil

`func (o *MeProfileResponse) SetLocaleNil(b bool)`

 SetLocaleNil sets the value for Locale to be an explicit nil

### UnsetLocale
`func (o *MeProfileResponse) UnsetLocale()`

UnsetLocale ensures that no value is present for Locale, not even an explicit nil
### GetName

`func (o *MeProfileResponse) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *MeProfileResponse) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *MeProfileResponse) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetNameConfirmationPending

`func (o *MeProfileResponse) GetNameConfirmationPending() bool`

GetNameConfirmationPending returns the NameConfirmationPending field if non-nil, zero value otherwise.

### GetNameConfirmationPendingOk

`func (o *MeProfileResponse) GetNameConfirmationPendingOk() (*bool, bool)`

GetNameConfirmationPendingOk returns a tuple with the NameConfirmationPending field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNameConfirmationPending

`func (o *MeProfileResponse) SetNameConfirmationPending(v bool)`

SetNameConfirmationPending sets NameConfirmationPending field to given value.


### GetObservedLocale

`func (o *MeProfileResponse) GetObservedLocale() string`

GetObservedLocale returns the ObservedLocale field if non-nil, zero value otherwise.

### GetObservedLocaleOk

`func (o *MeProfileResponse) GetObservedLocaleOk() (*string, bool)`

GetObservedLocaleOk returns a tuple with the ObservedLocale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetObservedLocale

`func (o *MeProfileResponse) SetObservedLocale(v string)`

SetObservedLocale sets ObservedLocale field to given value.

### HasObservedLocale

`func (o *MeProfileResponse) HasObservedLocale() bool`

HasObservedLocale returns a boolean if a field has been set.

### SetObservedLocaleNil

`func (o *MeProfileResponse) SetObservedLocaleNil(b bool)`

 SetObservedLocaleNil sets the value for ObservedLocale to be an explicit nil

### UnsetObservedLocale
`func (o *MeProfileResponse) UnsetObservedLocale()`

UnsetObservedLocale ensures that no value is present for ObservedLocale, not even an explicit nil
### GetOpenappImageThumbUrl

`func (o *MeProfileResponse) GetOpenappImageThumbUrl() string`

GetOpenappImageThumbUrl returns the OpenappImageThumbUrl field if non-nil, zero value otherwise.

### GetOpenappImageThumbUrlOk

`func (o *MeProfileResponse) GetOpenappImageThumbUrlOk() (*string, bool)`

GetOpenappImageThumbUrlOk returns a tuple with the OpenappImageThumbUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpenappImageThumbUrl

`func (o *MeProfileResponse) SetOpenappImageThumbUrl(v string)`

SetOpenappImageThumbUrl sets OpenappImageThumbUrl field to given value.

### HasOpenappImageThumbUrl

`func (o *MeProfileResponse) HasOpenappImageThumbUrl() bool`

HasOpenappImageThumbUrl returns a boolean if a field has been set.

### SetOpenappImageThumbUrlNil

`func (o *MeProfileResponse) SetOpenappImageThumbUrlNil(b bool)`

 SetOpenappImageThumbUrlNil sets the value for OpenappImageThumbUrl to be an explicit nil

### UnsetOpenappImageThumbUrl
`func (o *MeProfileResponse) UnsetOpenappImageThumbUrl()`

UnsetOpenappImageThumbUrl ensures that no value is present for OpenappImageThumbUrl, not even an explicit nil
### GetOpenappImageUrl

`func (o *MeProfileResponse) GetOpenappImageUrl() string`

GetOpenappImageUrl returns the OpenappImageUrl field if non-nil, zero value otherwise.

### GetOpenappImageUrlOk

`func (o *MeProfileResponse) GetOpenappImageUrlOk() (*string, bool)`

GetOpenappImageUrlOk returns a tuple with the OpenappImageUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpenappImageUrl

`func (o *MeProfileResponse) SetOpenappImageUrl(v string)`

SetOpenappImageUrl sets OpenappImageUrl field to given value.

### HasOpenappImageUrl

`func (o *MeProfileResponse) HasOpenappImageUrl() bool`

HasOpenappImageUrl returns a boolean if a field has been set.

### SetOpenappImageUrlNil

`func (o *MeProfileResponse) SetOpenappImageUrlNil(b bool)`

 SetOpenappImageUrlNil sets the value for OpenappImageUrl to be an explicit nil

### UnsetOpenappImageUrl
`func (o *MeProfileResponse) UnsetOpenappImageUrl()`

UnsetOpenappImageUrl ensures that no value is present for OpenappImageUrl, not even an explicit nil
### GetPhone

`func (o *MeProfileResponse) GetPhone() string`

GetPhone returns the Phone field if non-nil, zero value otherwise.

### GetPhoneOk

`func (o *MeProfileResponse) GetPhoneOk() (*string, bool)`

GetPhoneOk returns a tuple with the Phone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhone

`func (o *MeProfileResponse) SetPhone(v string)`

SetPhone sets Phone field to given value.

### HasPhone

`func (o *MeProfileResponse) HasPhone() bool`

HasPhone returns a boolean if a field has been set.

### SetPhoneNil

`func (o *MeProfileResponse) SetPhoneNil(b bool)`

 SetPhoneNil sets the value for Phone to be an explicit nil

### UnsetPhone
`func (o *MeProfileResponse) UnsetPhone()`

UnsetPhone ensures that no value is present for Phone, not even an explicit nil
### GetPhoneVerified

`func (o *MeProfileResponse) GetPhoneVerified() bool`

GetPhoneVerified returns the PhoneVerified field if non-nil, zero value otherwise.

### GetPhoneVerifiedOk

`func (o *MeProfileResponse) GetPhoneVerifiedOk() (*bool, bool)`

GetPhoneVerifiedOk returns a tuple with the PhoneVerified field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhoneVerified

`func (o *MeProfileResponse) SetPhoneVerified(v bool)`

SetPhoneVerified sets PhoneVerified field to given value.


### GetProfileSources

`func (o *MeProfileResponse) GetProfileSources() ProfileSourcesResponse`

GetProfileSources returns the ProfileSources field if non-nil, zero value otherwise.

### GetProfileSourcesOk

`func (o *MeProfileResponse) GetProfileSourcesOk() (*ProfileSourcesResponse, bool)`

GetProfileSourcesOk returns a tuple with the ProfileSources field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProfileSources

`func (o *MeProfileResponse) SetProfileSources(v ProfileSourcesResponse)`

SetProfileSources sets ProfileSources field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
