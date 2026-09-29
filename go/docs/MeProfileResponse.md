# MeProfileResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Email** | Pointer to **NullableString** |  | [optional]
**EmailVerified** | **bool** |  |
**IdpLinks** | [**[]IdpLinkResponse**](IdpLinkResponse.md) |  |
**ImageSyncError** | Pointer to **NullableString** |  | [optional]
**ImageThumbUrl** | Pointer to **NullableString** |  | [optional]
**ImageUrl** | Pointer to **NullableString** |  | [optional]
**Locale** | Pointer to **NullableString** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**Phone** | Pointer to **NullableString** |  | [optional]
**PhoneVerified** | **bool** |  |
**ProfileSources** | [**ProfileSourcesResponse**](ProfileSourcesResponse.md) |  |

## Methods

### NewMeProfileResponse

`func NewMeProfileResponse(emailVerified bool, idpLinks []IdpLinkResponse, phoneVerified bool, profileSources ProfileSourcesResponse, ) *MeProfileResponse`

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

`func (o *MeProfileResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *MeProfileResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *MeProfileResponse) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *MeProfileResponse) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *MeProfileResponse) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *MeProfileResponse) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
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
