# LocationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**City** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**Country** | Pointer to [**NullableLocationResponseCountry**](LocationResponseCountry.md) |  | [optional]
**FormattedAddress** | Pointer to [**NullableLocationResponseCountry**](LocationResponseCountry.md) |  | [optional]
**GeocodingProvider** | Pointer to [**NullableGeocodingProviderId**](GeocodingProviderId.md) |  | [optional]
**Id** | **NullableString** |  |
**ImageUrl** | Pointer to **NullableString** |  | [optional]
**Kind** | [**LocationKind**](LocationKind.md) |  |
**Lat** | Pointer to **NullableFloat64** |  | [optional]
**Lng** | Pointer to **NullableFloat64** |  | [optional]
**Notes** | Pointer to [**NullableLocationResponseCountry**](LocationResponseCountry.md) |  | [optional]
**ProviderPlaceId** | Pointer to **NullableString** | Opaque place id issued by &#x60;geocoding_provider&#x60;. Both fields are present together or not at all: a place id cannot be interpreted without knowing who issued it. | [optional]
**Source** | Pointer to [**NullableLocationSource**](LocationSource.md) |  | [optional]

## Methods

### NewLocationResponse

`func NewLocationResponse(id NullableString, kind LocationKind, ) *LocationResponse`

NewLocationResponse instantiates a new LocationResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewLocationResponseWithDefaults

`func NewLocationResponseWithDefaults() *LocationResponse`

NewLocationResponseWithDefaults instantiates a new LocationResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCity

`func (o *LocationResponse) GetCity() LocalizedString`

GetCity returns the City field if non-nil, zero value otherwise.

### GetCityOk

`func (o *LocationResponse) GetCityOk() (*LocalizedString, bool)`

GetCityOk returns a tuple with the City field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCity

`func (o *LocationResponse) SetCity(v LocalizedString)`

SetCity sets City field to given value.

### HasCity

`func (o *LocationResponse) HasCity() bool`

HasCity returns a boolean if a field has been set.

### SetCityNil

`func (o *LocationResponse) SetCityNil(b bool)`

 SetCityNil sets the value for City to be an explicit nil

### UnsetCity
`func (o *LocationResponse) UnsetCity()`

UnsetCity ensures that no value is present for City, not even an explicit nil
### GetCountry

`func (o *LocationResponse) GetCountry() LocationResponseCountry`

GetCountry returns the Country field if non-nil, zero value otherwise.

### GetCountryOk

`func (o *LocationResponse) GetCountryOk() (*LocationResponseCountry, bool)`

GetCountryOk returns a tuple with the Country field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCountry

`func (o *LocationResponse) SetCountry(v LocationResponseCountry)`

SetCountry sets Country field to given value.

### HasCountry

`func (o *LocationResponse) HasCountry() bool`

HasCountry returns a boolean if a field has been set.

### SetCountryNil

`func (o *LocationResponse) SetCountryNil(b bool)`

 SetCountryNil sets the value for Country to be an explicit nil

### UnsetCountry
`func (o *LocationResponse) UnsetCountry()`

UnsetCountry ensures that no value is present for Country, not even an explicit nil
### GetFormattedAddress

`func (o *LocationResponse) GetFormattedAddress() LocationResponseCountry`

GetFormattedAddress returns the FormattedAddress field if non-nil, zero value otherwise.

### GetFormattedAddressOk

`func (o *LocationResponse) GetFormattedAddressOk() (*LocationResponseCountry, bool)`

GetFormattedAddressOk returns a tuple with the FormattedAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFormattedAddress

`func (o *LocationResponse) SetFormattedAddress(v LocationResponseCountry)`

SetFormattedAddress sets FormattedAddress field to given value.

### HasFormattedAddress

`func (o *LocationResponse) HasFormattedAddress() bool`

HasFormattedAddress returns a boolean if a field has been set.

### SetFormattedAddressNil

`func (o *LocationResponse) SetFormattedAddressNil(b bool)`

 SetFormattedAddressNil sets the value for FormattedAddress to be an explicit nil

### UnsetFormattedAddress
`func (o *LocationResponse) UnsetFormattedAddress()`

UnsetFormattedAddress ensures that no value is present for FormattedAddress, not even an explicit nil
### GetGeocodingProvider

`func (o *LocationResponse) GetGeocodingProvider() GeocodingProviderId`

GetGeocodingProvider returns the GeocodingProvider field if non-nil, zero value otherwise.

### GetGeocodingProviderOk

`func (o *LocationResponse) GetGeocodingProviderOk() (*GeocodingProviderId, bool)`

GetGeocodingProviderOk returns a tuple with the GeocodingProvider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGeocodingProvider

`func (o *LocationResponse) SetGeocodingProvider(v GeocodingProviderId)`

SetGeocodingProvider sets GeocodingProvider field to given value.

### HasGeocodingProvider

`func (o *LocationResponse) HasGeocodingProvider() bool`

HasGeocodingProvider returns a boolean if a field has been set.

### SetGeocodingProviderNil

`func (o *LocationResponse) SetGeocodingProviderNil(b bool)`

 SetGeocodingProviderNil sets the value for GeocodingProvider to be an explicit nil

### UnsetGeocodingProvider
`func (o *LocationResponse) UnsetGeocodingProvider()`

UnsetGeocodingProvider ensures that no value is present for GeocodingProvider, not even an explicit nil
### GetId

`func (o *LocationResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *LocationResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *LocationResponse) SetId(v string)`

SetId sets Id field to given value.


### SetIdNil

`func (o *LocationResponse) SetIdNil(b bool)`

 SetIdNil sets the value for Id to be an explicit nil

### UnsetId
`func (o *LocationResponse) UnsetId()`

UnsetId ensures that no value is present for Id, not even an explicit nil
### GetImageUrl

`func (o *LocationResponse) GetImageUrl() string`

GetImageUrl returns the ImageUrl field if non-nil, zero value otherwise.

### GetImageUrlOk

`func (o *LocationResponse) GetImageUrlOk() (*string, bool)`

GetImageUrlOk returns a tuple with the ImageUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageUrl

`func (o *LocationResponse) SetImageUrl(v string)`

SetImageUrl sets ImageUrl field to given value.

### HasImageUrl

`func (o *LocationResponse) HasImageUrl() bool`

HasImageUrl returns a boolean if a field has been set.

### SetImageUrlNil

`func (o *LocationResponse) SetImageUrlNil(b bool)`

 SetImageUrlNil sets the value for ImageUrl to be an explicit nil

### UnsetImageUrl
`func (o *LocationResponse) UnsetImageUrl()`

UnsetImageUrl ensures that no value is present for ImageUrl, not even an explicit nil
### GetKind

`func (o *LocationResponse) GetKind() LocationKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *LocationResponse) GetKindOk() (*LocationKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *LocationResponse) SetKind(v LocationKind)`

SetKind sets Kind field to given value.


### GetLat

`func (o *LocationResponse) GetLat() float64`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *LocationResponse) GetLatOk() (*float64, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *LocationResponse) SetLat(v float64)`

SetLat sets Lat field to given value.

### HasLat

`func (o *LocationResponse) HasLat() bool`

HasLat returns a boolean if a field has been set.

### SetLatNil

`func (o *LocationResponse) SetLatNil(b bool)`

 SetLatNil sets the value for Lat to be an explicit nil

### UnsetLat
`func (o *LocationResponse) UnsetLat()`

UnsetLat ensures that no value is present for Lat, not even an explicit nil
### GetLng

`func (o *LocationResponse) GetLng() float64`

GetLng returns the Lng field if non-nil, zero value otherwise.

### GetLngOk

`func (o *LocationResponse) GetLngOk() (*float64, bool)`

GetLngOk returns a tuple with the Lng field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLng

`func (o *LocationResponse) SetLng(v float64)`

SetLng sets Lng field to given value.

### HasLng

`func (o *LocationResponse) HasLng() bool`

HasLng returns a boolean if a field has been set.

### SetLngNil

`func (o *LocationResponse) SetLngNil(b bool)`

 SetLngNil sets the value for Lng to be an explicit nil

### UnsetLng
`func (o *LocationResponse) UnsetLng()`

UnsetLng ensures that no value is present for Lng, not even an explicit nil
### GetNotes

`func (o *LocationResponse) GetNotes() LocationResponseCountry`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *LocationResponse) GetNotesOk() (*LocationResponseCountry, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *LocationResponse) SetNotes(v LocationResponseCountry)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *LocationResponse) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *LocationResponse) SetNotesNil(b bool)`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *LocationResponse) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil
### GetProviderPlaceId

`func (o *LocationResponse) GetProviderPlaceId() string`

GetProviderPlaceId returns the ProviderPlaceId field if non-nil, zero value otherwise.

### GetProviderPlaceIdOk

`func (o *LocationResponse) GetProviderPlaceIdOk() (*string, bool)`

GetProviderPlaceIdOk returns a tuple with the ProviderPlaceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderPlaceId

`func (o *LocationResponse) SetProviderPlaceId(v string)`

SetProviderPlaceId sets ProviderPlaceId field to given value.

### HasProviderPlaceId

`func (o *LocationResponse) HasProviderPlaceId() bool`

HasProviderPlaceId returns a boolean if a field has been set.

### SetProviderPlaceIdNil

`func (o *LocationResponse) SetProviderPlaceIdNil(b bool)`

 SetProviderPlaceIdNil sets the value for ProviderPlaceId to be an explicit nil

### UnsetProviderPlaceId
`func (o *LocationResponse) UnsetProviderPlaceId()`

UnsetProviderPlaceId ensures that no value is present for ProviderPlaceId, not even an explicit nil
### GetSource

`func (o *LocationResponse) GetSource() LocationSource`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *LocationResponse) GetSourceOk() (*LocationSource, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *LocationResponse) SetSource(v LocationSource)`

SetSource sets Source field to given value.

### HasSource

`func (o *LocationResponse) HasSource() bool`

HasSource returns a boolean if a field has been set.

### SetSourceNil

`func (o *LocationResponse) SetSourceNil(b bool)`

 SetSourceNil sets the value for Source to be an explicit nil

### UnsetSource
`func (o *LocationResponse) UnsetSource()`

UnsetSource ensures that no value is present for Source, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
