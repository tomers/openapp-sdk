# CreateLocationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**City** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**Country** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**FormattedAddress** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**Geocode** | Pointer to **bool** |  | [optional]
**GeocodingProvider** | Pointer to [**NullableGeocodingProviderId**](GeocodingProviderId.md) |  | [optional]
**Kind** | [**LocationKind**](LocationKind.md) |  |
**Lat** | Pointer to **NullableFloat64** |  | [optional]
**Lng** | Pointer to **NullableFloat64** |  | [optional]
**Locale** | Pointer to **NullableString** | Preferred locale (app code or BCP-47) for geocoding: controls the language of geocoded text and the locale key it is tagged under. Defaults to &#x60;en&#x60; when omitted. | [optional]
**Notes** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**ProviderPlaceId** | Pointer to **NullableString** | Opaque place id issued by &#x60;geocoding_provider&#x60;. Both fields must be sent together: a place id cannot be resolved without naming its issuer. | [optional]

## Methods

### NewCreateLocationRequest

`func NewCreateLocationRequest(kind LocationKind, ) *CreateLocationRequest`

NewCreateLocationRequest instantiates a new CreateLocationRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateLocationRequestWithDefaults

`func NewCreateLocationRequestWithDefaults() *CreateLocationRequest`

NewCreateLocationRequestWithDefaults instantiates a new CreateLocationRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCity

`func (o *CreateLocationRequest) GetCity() LocalizedString`

GetCity returns the City field if non-nil, zero value otherwise.

### GetCityOk

`func (o *CreateLocationRequest) GetCityOk() (*LocalizedString, bool)`

GetCityOk returns a tuple with the City field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCity

`func (o *CreateLocationRequest) SetCity(v LocalizedString)`

SetCity sets City field to given value.

### HasCity

`func (o *CreateLocationRequest) HasCity() bool`

HasCity returns a boolean if a field has been set.

### SetCityNil

`func (o *CreateLocationRequest) SetCityNil(b bool)`

 SetCityNil sets the value for City to be an explicit nil

### UnsetCity
`func (o *CreateLocationRequest) UnsetCity()`

UnsetCity ensures that no value is present for City, not even an explicit nil
### GetCountry

`func (o *CreateLocationRequest) GetCountry() LocalizedString`

GetCountry returns the Country field if non-nil, zero value otherwise.

### GetCountryOk

`func (o *CreateLocationRequest) GetCountryOk() (*LocalizedString, bool)`

GetCountryOk returns a tuple with the Country field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCountry

`func (o *CreateLocationRequest) SetCountry(v LocalizedString)`

SetCountry sets Country field to given value.

### HasCountry

`func (o *CreateLocationRequest) HasCountry() bool`

HasCountry returns a boolean if a field has been set.

### SetCountryNil

`func (o *CreateLocationRequest) SetCountryNil(b bool)`

 SetCountryNil sets the value for Country to be an explicit nil

### UnsetCountry
`func (o *CreateLocationRequest) UnsetCountry()`

UnsetCountry ensures that no value is present for Country, not even an explicit nil
### GetFormattedAddress

`func (o *CreateLocationRequest) GetFormattedAddress() LocalizedString`

GetFormattedAddress returns the FormattedAddress field if non-nil, zero value otherwise.

### GetFormattedAddressOk

`func (o *CreateLocationRequest) GetFormattedAddressOk() (*LocalizedString, bool)`

GetFormattedAddressOk returns a tuple with the FormattedAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFormattedAddress

`func (o *CreateLocationRequest) SetFormattedAddress(v LocalizedString)`

SetFormattedAddress sets FormattedAddress field to given value.

### HasFormattedAddress

`func (o *CreateLocationRequest) HasFormattedAddress() bool`

HasFormattedAddress returns a boolean if a field has been set.

### SetFormattedAddressNil

`func (o *CreateLocationRequest) SetFormattedAddressNil(b bool)`

 SetFormattedAddressNil sets the value for FormattedAddress to be an explicit nil

### UnsetFormattedAddress
`func (o *CreateLocationRequest) UnsetFormattedAddress()`

UnsetFormattedAddress ensures that no value is present for FormattedAddress, not even an explicit nil
### GetGeocode

`func (o *CreateLocationRequest) GetGeocode() bool`

GetGeocode returns the Geocode field if non-nil, zero value otherwise.

### GetGeocodeOk

`func (o *CreateLocationRequest) GetGeocodeOk() (*bool, bool)`

GetGeocodeOk returns a tuple with the Geocode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGeocode

`func (o *CreateLocationRequest) SetGeocode(v bool)`

SetGeocode sets Geocode field to given value.

### HasGeocode

`func (o *CreateLocationRequest) HasGeocode() bool`

HasGeocode returns a boolean if a field has been set.

### GetGeocodingProvider

`func (o *CreateLocationRequest) GetGeocodingProvider() GeocodingProviderId`

GetGeocodingProvider returns the GeocodingProvider field if non-nil, zero value otherwise.

### GetGeocodingProviderOk

`func (o *CreateLocationRequest) GetGeocodingProviderOk() (*GeocodingProviderId, bool)`

GetGeocodingProviderOk returns a tuple with the GeocodingProvider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGeocodingProvider

`func (o *CreateLocationRequest) SetGeocodingProvider(v GeocodingProviderId)`

SetGeocodingProvider sets GeocodingProvider field to given value.

### HasGeocodingProvider

`func (o *CreateLocationRequest) HasGeocodingProvider() bool`

HasGeocodingProvider returns a boolean if a field has been set.

### SetGeocodingProviderNil

`func (o *CreateLocationRequest) SetGeocodingProviderNil(b bool)`

 SetGeocodingProviderNil sets the value for GeocodingProvider to be an explicit nil

### UnsetGeocodingProvider
`func (o *CreateLocationRequest) UnsetGeocodingProvider()`

UnsetGeocodingProvider ensures that no value is present for GeocodingProvider, not even an explicit nil
### GetKind

`func (o *CreateLocationRequest) GetKind() LocationKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *CreateLocationRequest) GetKindOk() (*LocationKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *CreateLocationRequest) SetKind(v LocationKind)`

SetKind sets Kind field to given value.


### GetLat

`func (o *CreateLocationRequest) GetLat() float64`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *CreateLocationRequest) GetLatOk() (*float64, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *CreateLocationRequest) SetLat(v float64)`

SetLat sets Lat field to given value.

### HasLat

`func (o *CreateLocationRequest) HasLat() bool`

HasLat returns a boolean if a field has been set.

### SetLatNil

`func (o *CreateLocationRequest) SetLatNil(b bool)`

 SetLatNil sets the value for Lat to be an explicit nil

### UnsetLat
`func (o *CreateLocationRequest) UnsetLat()`

UnsetLat ensures that no value is present for Lat, not even an explicit nil
### GetLng

`func (o *CreateLocationRequest) GetLng() float64`

GetLng returns the Lng field if non-nil, zero value otherwise.

### GetLngOk

`func (o *CreateLocationRequest) GetLngOk() (*float64, bool)`

GetLngOk returns a tuple with the Lng field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLng

`func (o *CreateLocationRequest) SetLng(v float64)`

SetLng sets Lng field to given value.

### HasLng

`func (o *CreateLocationRequest) HasLng() bool`

HasLng returns a boolean if a field has been set.

### SetLngNil

`func (o *CreateLocationRequest) SetLngNil(b bool)`

 SetLngNil sets the value for Lng to be an explicit nil

### UnsetLng
`func (o *CreateLocationRequest) UnsetLng()`

UnsetLng ensures that no value is present for Lng, not even an explicit nil
### GetLocale

`func (o *CreateLocationRequest) GetLocale() string`

GetLocale returns the Locale field if non-nil, zero value otherwise.

### GetLocaleOk

`func (o *CreateLocationRequest) GetLocaleOk() (*string, bool)`

GetLocaleOk returns a tuple with the Locale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocale

`func (o *CreateLocationRequest) SetLocale(v string)`

SetLocale sets Locale field to given value.

### HasLocale

`func (o *CreateLocationRequest) HasLocale() bool`

HasLocale returns a boolean if a field has been set.

### SetLocaleNil

`func (o *CreateLocationRequest) SetLocaleNil(b bool)`

 SetLocaleNil sets the value for Locale to be an explicit nil

### UnsetLocale
`func (o *CreateLocationRequest) UnsetLocale()`

UnsetLocale ensures that no value is present for Locale, not even an explicit nil
### GetNotes

`func (o *CreateLocationRequest) GetNotes() LocalizedString`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *CreateLocationRequest) GetNotesOk() (*LocalizedString, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *CreateLocationRequest) SetNotes(v LocalizedString)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *CreateLocationRequest) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *CreateLocationRequest) SetNotesNil(b bool)`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *CreateLocationRequest) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil
### GetProviderPlaceId

`func (o *CreateLocationRequest) GetProviderPlaceId() string`

GetProviderPlaceId returns the ProviderPlaceId field if non-nil, zero value otherwise.

### GetProviderPlaceIdOk

`func (o *CreateLocationRequest) GetProviderPlaceIdOk() (*string, bool)`

GetProviderPlaceIdOk returns a tuple with the ProviderPlaceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderPlaceId

`func (o *CreateLocationRequest) SetProviderPlaceId(v string)`

SetProviderPlaceId sets ProviderPlaceId field to given value.

### HasProviderPlaceId

`func (o *CreateLocationRequest) HasProviderPlaceId() bool`

HasProviderPlaceId returns a boolean if a field has been set.

### SetProviderPlaceIdNil

`func (o *CreateLocationRequest) SetProviderPlaceIdNil(b bool)`

 SetProviderPlaceIdNil sets the value for ProviderPlaceId to be an explicit nil

### UnsetProviderPlaceId
`func (o *CreateLocationRequest) UnsetProviderPlaceId()`

UnsetProviderPlaceId ensures that no value is present for ProviderPlaceId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
