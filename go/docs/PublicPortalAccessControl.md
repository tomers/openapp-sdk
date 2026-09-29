# PublicPortalAccessControl

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**City** | Pointer to **interface{}** |  | [optional]
**Country** | Pointer to **interface{}** |  | [optional]
**FormattedAddress** | Pointer to **interface{}** |  | [optional]
**GeocodingProvider** | Pointer to [**NullableGeocodingProviderId**](GeocodingProviderId.md) |  | [optional]
**ImageThumbUrl** | Pointer to **NullableString** | Presigned URL for the &#x60;thumb&#x60; rendition of the same photo, for small renders. Absent when the source is already thumb-sized; fall back to &#x60;image_url&#x60;. | [optional]
**ImageUrl** | Pointer to **NullableString** | Presigned URL for the integration or location photo. | [optional]
**Lat** | Pointer to **NullableFloat64** |  | [optional]
**Lng** | Pointer to **NullableFloat64** |  | [optional]
**Message** | Pointer to **interface{}** |  | [optional]
**Name** | **interface{}** |  |
**Notes** | Pointer to **interface{}** |  | [optional]
**ProviderPlaceId** | Pointer to **NullableString** | Opaque place id for the address, when it was resolved by a maps provider, together with the provider that issued it. Clients that recognize the provider can link to the place itself (e.g. Google Maps URLs &#x60;query_place_id&#x60;) instead of a bare coordinate; the rest fall back to the address text. Both fields are present together or not at all. | [optional]
**Status** | Pointer to **NullableString** | Directory status when this summary describes the visitor directory: one of &#x60;ready&#x60;, &#x60;empty&#x60;, &#x60;not_configured&#x60;, &#x60;disabled&#x60;. Absent for the access-control summary. | [optional]

## Methods

### NewPublicPortalAccessControl

`func NewPublicPortalAccessControl(name interface{}, ) *PublicPortalAccessControl`

NewPublicPortalAccessControl instantiates a new PublicPortalAccessControl object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicPortalAccessControlWithDefaults

`func NewPublicPortalAccessControlWithDefaults() *PublicPortalAccessControl`

NewPublicPortalAccessControlWithDefaults instantiates a new PublicPortalAccessControl object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCity

`func (o *PublicPortalAccessControl) GetCity() interface{}`

GetCity returns the City field if non-nil, zero value otherwise.

### GetCityOk

`func (o *PublicPortalAccessControl) GetCityOk() (*interface{}, bool)`

GetCityOk returns a tuple with the City field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCity

`func (o *PublicPortalAccessControl) SetCity(v interface{})`

SetCity sets City field to given value.

### HasCity

`func (o *PublicPortalAccessControl) HasCity() bool`

HasCity returns a boolean if a field has been set.

### SetCityNil

`func (o *PublicPortalAccessControl) SetCityNil(b bool)`

 SetCityNil sets the value for City to be an explicit nil

### UnsetCity
`func (o *PublicPortalAccessControl) UnsetCity()`

UnsetCity ensures that no value is present for City, not even an explicit nil
### GetCountry

`func (o *PublicPortalAccessControl) GetCountry() interface{}`

GetCountry returns the Country field if non-nil, zero value otherwise.

### GetCountryOk

`func (o *PublicPortalAccessControl) GetCountryOk() (*interface{}, bool)`

GetCountryOk returns a tuple with the Country field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCountry

`func (o *PublicPortalAccessControl) SetCountry(v interface{})`

SetCountry sets Country field to given value.

### HasCountry

`func (o *PublicPortalAccessControl) HasCountry() bool`

HasCountry returns a boolean if a field has been set.

### SetCountryNil

`func (o *PublicPortalAccessControl) SetCountryNil(b bool)`

 SetCountryNil sets the value for Country to be an explicit nil

### UnsetCountry
`func (o *PublicPortalAccessControl) UnsetCountry()`

UnsetCountry ensures that no value is present for Country, not even an explicit nil
### GetFormattedAddress

`func (o *PublicPortalAccessControl) GetFormattedAddress() interface{}`

GetFormattedAddress returns the FormattedAddress field if non-nil, zero value otherwise.

### GetFormattedAddressOk

`func (o *PublicPortalAccessControl) GetFormattedAddressOk() (*interface{}, bool)`

GetFormattedAddressOk returns a tuple with the FormattedAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFormattedAddress

`func (o *PublicPortalAccessControl) SetFormattedAddress(v interface{})`

SetFormattedAddress sets FormattedAddress field to given value.

### HasFormattedAddress

`func (o *PublicPortalAccessControl) HasFormattedAddress() bool`

HasFormattedAddress returns a boolean if a field has been set.

### SetFormattedAddressNil

`func (o *PublicPortalAccessControl) SetFormattedAddressNil(b bool)`

 SetFormattedAddressNil sets the value for FormattedAddress to be an explicit nil

### UnsetFormattedAddress
`func (o *PublicPortalAccessControl) UnsetFormattedAddress()`

UnsetFormattedAddress ensures that no value is present for FormattedAddress, not even an explicit nil
### GetGeocodingProvider

`func (o *PublicPortalAccessControl) GetGeocodingProvider() GeocodingProviderId`

GetGeocodingProvider returns the GeocodingProvider field if non-nil, zero value otherwise.

### GetGeocodingProviderOk

`func (o *PublicPortalAccessControl) GetGeocodingProviderOk() (*GeocodingProviderId, bool)`

GetGeocodingProviderOk returns a tuple with the GeocodingProvider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGeocodingProvider

`func (o *PublicPortalAccessControl) SetGeocodingProvider(v GeocodingProviderId)`

SetGeocodingProvider sets GeocodingProvider field to given value.

### HasGeocodingProvider

`func (o *PublicPortalAccessControl) HasGeocodingProvider() bool`

HasGeocodingProvider returns a boolean if a field has been set.

### SetGeocodingProviderNil

`func (o *PublicPortalAccessControl) SetGeocodingProviderNil(b bool)`

 SetGeocodingProviderNil sets the value for GeocodingProvider to be an explicit nil

### UnsetGeocodingProvider
`func (o *PublicPortalAccessControl) UnsetGeocodingProvider()`

UnsetGeocodingProvider ensures that no value is present for GeocodingProvider, not even an explicit nil
### GetImageThumbUrl

`func (o *PublicPortalAccessControl) GetImageThumbUrl() string`

GetImageThumbUrl returns the ImageThumbUrl field if non-nil, zero value otherwise.

### GetImageThumbUrlOk

`func (o *PublicPortalAccessControl) GetImageThumbUrlOk() (*string, bool)`

GetImageThumbUrlOk returns a tuple with the ImageThumbUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageThumbUrl

`func (o *PublicPortalAccessControl) SetImageThumbUrl(v string)`

SetImageThumbUrl sets ImageThumbUrl field to given value.

### HasImageThumbUrl

`func (o *PublicPortalAccessControl) HasImageThumbUrl() bool`

HasImageThumbUrl returns a boolean if a field has been set.

### SetImageThumbUrlNil

`func (o *PublicPortalAccessControl) SetImageThumbUrlNil(b bool)`

 SetImageThumbUrlNil sets the value for ImageThumbUrl to be an explicit nil

### UnsetImageThumbUrl
`func (o *PublicPortalAccessControl) UnsetImageThumbUrl()`

UnsetImageThumbUrl ensures that no value is present for ImageThumbUrl, not even an explicit nil
### GetImageUrl

`func (o *PublicPortalAccessControl) GetImageUrl() string`

GetImageUrl returns the ImageUrl field if non-nil, zero value otherwise.

### GetImageUrlOk

`func (o *PublicPortalAccessControl) GetImageUrlOk() (*string, bool)`

GetImageUrlOk returns a tuple with the ImageUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageUrl

`func (o *PublicPortalAccessControl) SetImageUrl(v string)`

SetImageUrl sets ImageUrl field to given value.

### HasImageUrl

`func (o *PublicPortalAccessControl) HasImageUrl() bool`

HasImageUrl returns a boolean if a field has been set.

### SetImageUrlNil

`func (o *PublicPortalAccessControl) SetImageUrlNil(b bool)`

 SetImageUrlNil sets the value for ImageUrl to be an explicit nil

### UnsetImageUrl
`func (o *PublicPortalAccessControl) UnsetImageUrl()`

UnsetImageUrl ensures that no value is present for ImageUrl, not even an explicit nil
### GetLat

`func (o *PublicPortalAccessControl) GetLat() float64`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *PublicPortalAccessControl) GetLatOk() (*float64, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *PublicPortalAccessControl) SetLat(v float64)`

SetLat sets Lat field to given value.

### HasLat

`func (o *PublicPortalAccessControl) HasLat() bool`

HasLat returns a boolean if a field has been set.

### SetLatNil

`func (o *PublicPortalAccessControl) SetLatNil(b bool)`

 SetLatNil sets the value for Lat to be an explicit nil

### UnsetLat
`func (o *PublicPortalAccessControl) UnsetLat()`

UnsetLat ensures that no value is present for Lat, not even an explicit nil
### GetLng

`func (o *PublicPortalAccessControl) GetLng() float64`

GetLng returns the Lng field if non-nil, zero value otherwise.

### GetLngOk

`func (o *PublicPortalAccessControl) GetLngOk() (*float64, bool)`

GetLngOk returns a tuple with the Lng field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLng

`func (o *PublicPortalAccessControl) SetLng(v float64)`

SetLng sets Lng field to given value.

### HasLng

`func (o *PublicPortalAccessControl) HasLng() bool`

HasLng returns a boolean if a field has been set.

### SetLngNil

`func (o *PublicPortalAccessControl) SetLngNil(b bool)`

 SetLngNil sets the value for Lng to be an explicit nil

### UnsetLng
`func (o *PublicPortalAccessControl) UnsetLng()`

UnsetLng ensures that no value is present for Lng, not even an explicit nil
### GetMessage

`func (o *PublicPortalAccessControl) GetMessage() interface{}`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *PublicPortalAccessControl) GetMessageOk() (*interface{}, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *PublicPortalAccessControl) SetMessage(v interface{})`

SetMessage sets Message field to given value.

### HasMessage

`func (o *PublicPortalAccessControl) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### SetMessageNil

`func (o *PublicPortalAccessControl) SetMessageNil(b bool)`

 SetMessageNil sets the value for Message to be an explicit nil

### UnsetMessage
`func (o *PublicPortalAccessControl) UnsetMessage()`

UnsetMessage ensures that no value is present for Message, not even an explicit nil
### GetName

`func (o *PublicPortalAccessControl) GetName() interface{}`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PublicPortalAccessControl) GetNameOk() (*interface{}, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PublicPortalAccessControl) SetName(v interface{})`

SetName sets Name field to given value.


### SetNameNil

`func (o *PublicPortalAccessControl) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *PublicPortalAccessControl) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetNotes

`func (o *PublicPortalAccessControl) GetNotes() interface{}`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *PublicPortalAccessControl) GetNotesOk() (*interface{}, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *PublicPortalAccessControl) SetNotes(v interface{})`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *PublicPortalAccessControl) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *PublicPortalAccessControl) SetNotesNil(b bool)`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *PublicPortalAccessControl) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil
### GetProviderPlaceId

`func (o *PublicPortalAccessControl) GetProviderPlaceId() string`

GetProviderPlaceId returns the ProviderPlaceId field if non-nil, zero value otherwise.

### GetProviderPlaceIdOk

`func (o *PublicPortalAccessControl) GetProviderPlaceIdOk() (*string, bool)`

GetProviderPlaceIdOk returns a tuple with the ProviderPlaceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderPlaceId

`func (o *PublicPortalAccessControl) SetProviderPlaceId(v string)`

SetProviderPlaceId sets ProviderPlaceId field to given value.

### HasProviderPlaceId

`func (o *PublicPortalAccessControl) HasProviderPlaceId() bool`

HasProviderPlaceId returns a boolean if a field has been set.

### SetProviderPlaceIdNil

`func (o *PublicPortalAccessControl) SetProviderPlaceIdNil(b bool)`

 SetProviderPlaceIdNil sets the value for ProviderPlaceId to be an explicit nil

### UnsetProviderPlaceId
`func (o *PublicPortalAccessControl) UnsetProviderPlaceId()`

UnsetProviderPlaceId ensures that no value is present for ProviderPlaceId, not even an explicit nil
### GetStatus

`func (o *PublicPortalAccessControl) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *PublicPortalAccessControl) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *PublicPortalAccessControl) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *PublicPortalAccessControl) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### SetStatusNil

`func (o *PublicPortalAccessControl) SetStatusNil(b bool)`

 SetStatusNil sets the value for Status to be an explicit nil

### UnsetStatus
`func (o *PublicPortalAccessControl) UnsetStatus()`

UnsetStatus ensures that no value is present for Status, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
