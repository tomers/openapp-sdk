# PatchLocationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**City** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**Country** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**FormattedAddress** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**Geocode** | Pointer to **bool** |  | [optional]
**GeocodingProvider** | Pointer to [**NullableGeocodingProviderId**](GeocodingProviderId.md) |  | [optional]
**Kind** | Pointer to [**NullableLocationKind**](LocationKind.md) |  | [optional]
**Lat** | Pointer to **NullableFloat64** |  | [optional]
**Lng** | Pointer to **NullableFloat64** |  | [optional]
**Locale** | Pointer to **NullableString** | Preferred locale (app code or BCP-47) for geocoding (see &#x60;CreateLocationRequest::locale&#x60;). | [optional]
**Notes** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**ProviderPlaceId** | Pointer to **NullableString** | Replaces the stored place reference. Both fields must be sent together (see &#x60;CreateLocationRequest::provider_place_id&#x60;). | [optional]

## Methods

### NewPatchLocationRequest

`func NewPatchLocationRequest() *PatchLocationRequest`

NewPatchLocationRequest instantiates a new PatchLocationRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPatchLocationRequestWithDefaults

`func NewPatchLocationRequestWithDefaults() *PatchLocationRequest`

NewPatchLocationRequestWithDefaults instantiates a new PatchLocationRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCity

`func (o *PatchLocationRequest) GetCity() LocalizedString`

GetCity returns the City field if non-nil, zero value otherwise.

### GetCityOk

`func (o *PatchLocationRequest) GetCityOk() (*LocalizedString, bool)`

GetCityOk returns a tuple with the City field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCity

`func (o *PatchLocationRequest) SetCity(v LocalizedString)`

SetCity sets City field to given value.

### HasCity

`func (o *PatchLocationRequest) HasCity() bool`

HasCity returns a boolean if a field has been set.

### SetCityNil

`func (o *PatchLocationRequest) SetCityNil(b bool)`

 SetCityNil sets the value for City to be an explicit nil

### UnsetCity
`func (o *PatchLocationRequest) UnsetCity()`

UnsetCity ensures that no value is present for City, not even an explicit nil
### GetCountry

`func (o *PatchLocationRequest) GetCountry() LocalizedString`

GetCountry returns the Country field if non-nil, zero value otherwise.

### GetCountryOk

`func (o *PatchLocationRequest) GetCountryOk() (*LocalizedString, bool)`

GetCountryOk returns a tuple with the Country field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCountry

`func (o *PatchLocationRequest) SetCountry(v LocalizedString)`

SetCountry sets Country field to given value.

### HasCountry

`func (o *PatchLocationRequest) HasCountry() bool`

HasCountry returns a boolean if a field has been set.

### SetCountryNil

`func (o *PatchLocationRequest) SetCountryNil(b bool)`

 SetCountryNil sets the value for Country to be an explicit nil

### UnsetCountry
`func (o *PatchLocationRequest) UnsetCountry()`

UnsetCountry ensures that no value is present for Country, not even an explicit nil
### GetFormattedAddress

`func (o *PatchLocationRequest) GetFormattedAddress() LocalizedString`

GetFormattedAddress returns the FormattedAddress field if non-nil, zero value otherwise.

### GetFormattedAddressOk

`func (o *PatchLocationRequest) GetFormattedAddressOk() (*LocalizedString, bool)`

GetFormattedAddressOk returns a tuple with the FormattedAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFormattedAddress

`func (o *PatchLocationRequest) SetFormattedAddress(v LocalizedString)`

SetFormattedAddress sets FormattedAddress field to given value.

### HasFormattedAddress

`func (o *PatchLocationRequest) HasFormattedAddress() bool`

HasFormattedAddress returns a boolean if a field has been set.

### SetFormattedAddressNil

`func (o *PatchLocationRequest) SetFormattedAddressNil(b bool)`

 SetFormattedAddressNil sets the value for FormattedAddress to be an explicit nil

### UnsetFormattedAddress
`func (o *PatchLocationRequest) UnsetFormattedAddress()`

UnsetFormattedAddress ensures that no value is present for FormattedAddress, not even an explicit nil
### GetGeocode

`func (o *PatchLocationRequest) GetGeocode() bool`

GetGeocode returns the Geocode field if non-nil, zero value otherwise.

### GetGeocodeOk

`func (o *PatchLocationRequest) GetGeocodeOk() (*bool, bool)`

GetGeocodeOk returns a tuple with the Geocode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGeocode

`func (o *PatchLocationRequest) SetGeocode(v bool)`

SetGeocode sets Geocode field to given value.

### HasGeocode

`func (o *PatchLocationRequest) HasGeocode() bool`

HasGeocode returns a boolean if a field has been set.

### GetGeocodingProvider

`func (o *PatchLocationRequest) GetGeocodingProvider() GeocodingProviderId`

GetGeocodingProvider returns the GeocodingProvider field if non-nil, zero value otherwise.

### GetGeocodingProviderOk

`func (o *PatchLocationRequest) GetGeocodingProviderOk() (*GeocodingProviderId, bool)`

GetGeocodingProviderOk returns a tuple with the GeocodingProvider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGeocodingProvider

`func (o *PatchLocationRequest) SetGeocodingProvider(v GeocodingProviderId)`

SetGeocodingProvider sets GeocodingProvider field to given value.

### HasGeocodingProvider

`func (o *PatchLocationRequest) HasGeocodingProvider() bool`

HasGeocodingProvider returns a boolean if a field has been set.

### SetGeocodingProviderNil

`func (o *PatchLocationRequest) SetGeocodingProviderNil(b bool)`

 SetGeocodingProviderNil sets the value for GeocodingProvider to be an explicit nil

### UnsetGeocodingProvider
`func (o *PatchLocationRequest) UnsetGeocodingProvider()`

UnsetGeocodingProvider ensures that no value is present for GeocodingProvider, not even an explicit nil
### GetKind

`func (o *PatchLocationRequest) GetKind() LocationKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *PatchLocationRequest) GetKindOk() (*LocationKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *PatchLocationRequest) SetKind(v LocationKind)`

SetKind sets Kind field to given value.

### HasKind

`func (o *PatchLocationRequest) HasKind() bool`

HasKind returns a boolean if a field has been set.

### SetKindNil

`func (o *PatchLocationRequest) SetKindNil(b bool)`

 SetKindNil sets the value for Kind to be an explicit nil

### UnsetKind
`func (o *PatchLocationRequest) UnsetKind()`

UnsetKind ensures that no value is present for Kind, not even an explicit nil
### GetLat

`func (o *PatchLocationRequest) GetLat() float64`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *PatchLocationRequest) GetLatOk() (*float64, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *PatchLocationRequest) SetLat(v float64)`

SetLat sets Lat field to given value.

### HasLat

`func (o *PatchLocationRequest) HasLat() bool`

HasLat returns a boolean if a field has been set.

### SetLatNil

`func (o *PatchLocationRequest) SetLatNil(b bool)`

 SetLatNil sets the value for Lat to be an explicit nil

### UnsetLat
`func (o *PatchLocationRequest) UnsetLat()`

UnsetLat ensures that no value is present for Lat, not even an explicit nil
### GetLng

`func (o *PatchLocationRequest) GetLng() float64`

GetLng returns the Lng field if non-nil, zero value otherwise.

### GetLngOk

`func (o *PatchLocationRequest) GetLngOk() (*float64, bool)`

GetLngOk returns a tuple with the Lng field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLng

`func (o *PatchLocationRequest) SetLng(v float64)`

SetLng sets Lng field to given value.

### HasLng

`func (o *PatchLocationRequest) HasLng() bool`

HasLng returns a boolean if a field has been set.

### SetLngNil

`func (o *PatchLocationRequest) SetLngNil(b bool)`

 SetLngNil sets the value for Lng to be an explicit nil

### UnsetLng
`func (o *PatchLocationRequest) UnsetLng()`

UnsetLng ensures that no value is present for Lng, not even an explicit nil
### GetLocale

`func (o *PatchLocationRequest) GetLocale() string`

GetLocale returns the Locale field if non-nil, zero value otherwise.

### GetLocaleOk

`func (o *PatchLocationRequest) GetLocaleOk() (*string, bool)`

GetLocaleOk returns a tuple with the Locale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocale

`func (o *PatchLocationRequest) SetLocale(v string)`

SetLocale sets Locale field to given value.

### HasLocale

`func (o *PatchLocationRequest) HasLocale() bool`

HasLocale returns a boolean if a field has been set.

### SetLocaleNil

`func (o *PatchLocationRequest) SetLocaleNil(b bool)`

 SetLocaleNil sets the value for Locale to be an explicit nil

### UnsetLocale
`func (o *PatchLocationRequest) UnsetLocale()`

UnsetLocale ensures that no value is present for Locale, not even an explicit nil
### GetNotes

`func (o *PatchLocationRequest) GetNotes() LocalizedString`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *PatchLocationRequest) GetNotesOk() (*LocalizedString, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *PatchLocationRequest) SetNotes(v LocalizedString)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *PatchLocationRequest) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *PatchLocationRequest) SetNotesNil(b bool)`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *PatchLocationRequest) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil
### GetProviderPlaceId

`func (o *PatchLocationRequest) GetProviderPlaceId() string`

GetProviderPlaceId returns the ProviderPlaceId field if non-nil, zero value otherwise.

### GetProviderPlaceIdOk

`func (o *PatchLocationRequest) GetProviderPlaceIdOk() (*string, bool)`

GetProviderPlaceIdOk returns a tuple with the ProviderPlaceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderPlaceId

`func (o *PatchLocationRequest) SetProviderPlaceId(v string)`

SetProviderPlaceId sets ProviderPlaceId field to given value.

### HasProviderPlaceId

`func (o *PatchLocationRequest) HasProviderPlaceId() bool`

HasProviderPlaceId returns a boolean if a field has been set.

### SetProviderPlaceIdNil

`func (o *PatchLocationRequest) SetProviderPlaceIdNil(b bool)`

 SetProviderPlaceIdNil sets the value for ProviderPlaceId to be an explicit nil

### UnsetProviderPlaceId
`func (o *PatchLocationRequest) UnsetProviderPlaceId()`

UnsetProviderPlaceId ensures that no value is present for ProviderPlaceId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
