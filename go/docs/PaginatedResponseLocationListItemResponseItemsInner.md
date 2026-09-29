# PaginatedResponseLocationListItemResponseItemsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**City** | Pointer to [**LocalizedString**](LocalizedString.md) |  | [optional]
**Country** | Pointer to [**NullableLocationResponseCountry**](LocationResponseCountry.md) |  | [optional]
**FormattedAddress** | Pointer to [**NullableLocationResponseCountry**](LocationResponseCountry.md) |  | [optional]
**GeocodingProvider** | Pointer to [**GeocodingProviderId**](GeocodingProviderId.md) |  | [optional]
**Id** | **string** |  |
**ImageUrl** | Pointer to **string** |  | [optional]
**Kind** | [**LocationKind**](LocationKind.md) |  |
**Lat** | Pointer to **float64** |  | [optional]
**Lng** | Pointer to **float64** |  | [optional]
**Notes** | Pointer to [**NullableLocationResponseCountry**](LocationResponseCountry.md) |  | [optional]
**ProviderPlaceId** | Pointer to **string** | Opaque place id issued by &#x60;geocoding_provider&#x60;. Both fields are present together or not at all: a place id cannot be interpreted without knowing who issued it. | [optional]
**Source** | Pointer to [**LocationSource**](LocationSource.md) |  | [optional]
**IsBoundHere** | Pointer to **NullableBool** |  | [optional]
**IsRelevant** | Pointer to **NullableBool** |  | [optional]
**UsageCount** | Pointer to **NullableInt64** |  | [optional]

## Methods

### NewPaginatedResponseLocationListItemResponseItemsInner

`func NewPaginatedResponseLocationListItemResponseItemsInner(id string, kind LocationKind, ) *PaginatedResponseLocationListItemResponseItemsInner`

NewPaginatedResponseLocationListItemResponseItemsInner instantiates a new PaginatedResponseLocationListItemResponseItemsInner object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseLocationListItemResponseItemsInnerWithDefaults

`func NewPaginatedResponseLocationListItemResponseItemsInnerWithDefaults() *PaginatedResponseLocationListItemResponseItemsInner`

NewPaginatedResponseLocationListItemResponseItemsInnerWithDefaults instantiates a new PaginatedResponseLocationListItemResponseItemsInner object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCity

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetCity() LocalizedString`

GetCity returns the City field if non-nil, zero value otherwise.

### GetCityOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetCityOk() (*LocalizedString, bool)`

GetCityOk returns a tuple with the City field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCity

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetCity(v LocalizedString)`

SetCity sets City field to given value.

### HasCity

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasCity() bool`

HasCity returns a boolean if a field has been set.

### GetCountry

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetCountry() LocationResponseCountry`

GetCountry returns the Country field if non-nil, zero value otherwise.

### GetCountryOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetCountryOk() (*LocationResponseCountry, bool)`

GetCountryOk returns a tuple with the Country field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCountry

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetCountry(v LocationResponseCountry)`

SetCountry sets Country field to given value.

### HasCountry

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasCountry() bool`

HasCountry returns a boolean if a field has been set.

### SetCountryNil

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetCountryNil(b bool)`

 SetCountryNil sets the value for Country to be an explicit nil

### UnsetCountry
`func (o *PaginatedResponseLocationListItemResponseItemsInner) UnsetCountry()`

UnsetCountry ensures that no value is present for Country, not even an explicit nil
### GetFormattedAddress

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetFormattedAddress() LocationResponseCountry`

GetFormattedAddress returns the FormattedAddress field if non-nil, zero value otherwise.

### GetFormattedAddressOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetFormattedAddressOk() (*LocationResponseCountry, bool)`

GetFormattedAddressOk returns a tuple with the FormattedAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFormattedAddress

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetFormattedAddress(v LocationResponseCountry)`

SetFormattedAddress sets FormattedAddress field to given value.

### HasFormattedAddress

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasFormattedAddress() bool`

HasFormattedAddress returns a boolean if a field has been set.

### SetFormattedAddressNil

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetFormattedAddressNil(b bool)`

 SetFormattedAddressNil sets the value for FormattedAddress to be an explicit nil

### UnsetFormattedAddress
`func (o *PaginatedResponseLocationListItemResponseItemsInner) UnsetFormattedAddress()`

UnsetFormattedAddress ensures that no value is present for FormattedAddress, not even an explicit nil
### GetGeocodingProvider

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetGeocodingProvider() GeocodingProviderId`

GetGeocodingProvider returns the GeocodingProvider field if non-nil, zero value otherwise.

### GetGeocodingProviderOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetGeocodingProviderOk() (*GeocodingProviderId, bool)`

GetGeocodingProviderOk returns a tuple with the GeocodingProvider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGeocodingProvider

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetGeocodingProvider(v GeocodingProviderId)`

SetGeocodingProvider sets GeocodingProvider field to given value.

### HasGeocodingProvider

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasGeocodingProvider() bool`

HasGeocodingProvider returns a boolean if a field has been set.

### GetId

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetId(v string)`

SetId sets Id field to given value.


### GetImageUrl

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetImageUrl() string`

GetImageUrl returns the ImageUrl field if non-nil, zero value otherwise.

### GetImageUrlOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetImageUrlOk() (*string, bool)`

GetImageUrlOk returns a tuple with the ImageUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageUrl

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetImageUrl(v string)`

SetImageUrl sets ImageUrl field to given value.

### HasImageUrl

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasImageUrl() bool`

HasImageUrl returns a boolean if a field has been set.

### GetKind

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetKind() LocationKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetKindOk() (*LocationKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetKind(v LocationKind)`

SetKind sets Kind field to given value.


### GetLat

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetLat() float64`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetLatOk() (*float64, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetLat(v float64)`

SetLat sets Lat field to given value.

### HasLat

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasLat() bool`

HasLat returns a boolean if a field has been set.

### GetLng

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetLng() float64`

GetLng returns the Lng field if non-nil, zero value otherwise.

### GetLngOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetLngOk() (*float64, bool)`

GetLngOk returns a tuple with the Lng field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLng

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetLng(v float64)`

SetLng sets Lng field to given value.

### HasLng

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasLng() bool`

HasLng returns a boolean if a field has been set.

### GetNotes

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetNotes() LocationResponseCountry`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetNotesOk() (*LocationResponseCountry, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetNotes(v LocationResponseCountry)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetNotesNil(b bool)`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *PaginatedResponseLocationListItemResponseItemsInner) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil
### GetProviderPlaceId

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetProviderPlaceId() string`

GetProviderPlaceId returns the ProviderPlaceId field if non-nil, zero value otherwise.

### GetProviderPlaceIdOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetProviderPlaceIdOk() (*string, bool)`

GetProviderPlaceIdOk returns a tuple with the ProviderPlaceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderPlaceId

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetProviderPlaceId(v string)`

SetProviderPlaceId sets ProviderPlaceId field to given value.

### HasProviderPlaceId

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasProviderPlaceId() bool`

HasProviderPlaceId returns a boolean if a field has been set.

### GetSource

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetSource() LocationSource`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetSourceOk() (*LocationSource, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetSource(v LocationSource)`

SetSource sets Source field to given value.

### HasSource

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasSource() bool`

HasSource returns a boolean if a field has been set.

### GetIsBoundHere

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetIsBoundHere() bool`

GetIsBoundHere returns the IsBoundHere field if non-nil, zero value otherwise.

### GetIsBoundHereOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetIsBoundHereOk() (*bool, bool)`

GetIsBoundHereOk returns a tuple with the IsBoundHere field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsBoundHere

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetIsBoundHere(v bool)`

SetIsBoundHere sets IsBoundHere field to given value.

### HasIsBoundHere

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasIsBoundHere() bool`

HasIsBoundHere returns a boolean if a field has been set.

### SetIsBoundHereNil

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetIsBoundHereNil(b bool)`

 SetIsBoundHereNil sets the value for IsBoundHere to be an explicit nil

### UnsetIsBoundHere
`func (o *PaginatedResponseLocationListItemResponseItemsInner) UnsetIsBoundHere()`

UnsetIsBoundHere ensures that no value is present for IsBoundHere, not even an explicit nil
### GetIsRelevant

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetIsRelevant() bool`

GetIsRelevant returns the IsRelevant field if non-nil, zero value otherwise.

### GetIsRelevantOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetIsRelevantOk() (*bool, bool)`

GetIsRelevantOk returns a tuple with the IsRelevant field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsRelevant

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetIsRelevant(v bool)`

SetIsRelevant sets IsRelevant field to given value.

### HasIsRelevant

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasIsRelevant() bool`

HasIsRelevant returns a boolean if a field has been set.

### SetIsRelevantNil

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetIsRelevantNil(b bool)`

 SetIsRelevantNil sets the value for IsRelevant to be an explicit nil

### UnsetIsRelevant
`func (o *PaginatedResponseLocationListItemResponseItemsInner) UnsetIsRelevant()`

UnsetIsRelevant ensures that no value is present for IsRelevant, not even an explicit nil
### GetUsageCount

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetUsageCount() int64`

GetUsageCount returns the UsageCount field if non-nil, zero value otherwise.

### GetUsageCountOk

`func (o *PaginatedResponseLocationListItemResponseItemsInner) GetUsageCountOk() (*int64, bool)`

GetUsageCountOk returns a tuple with the UsageCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsageCount

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetUsageCount(v int64)`

SetUsageCount sets UsageCount field to given value.

### HasUsageCount

`func (o *PaginatedResponseLocationListItemResponseItemsInner) HasUsageCount() bool`

HasUsageCount returns a boolean if a field has been set.

### SetUsageCountNil

`func (o *PaginatedResponseLocationListItemResponseItemsInner) SetUsageCountNil(b bool)`

 SetUsageCountNil sets the value for UsageCount to be an explicit nil

### UnsetUsageCount
`func (o *PaginatedResponseLocationListItemResponseItemsInner) UnsetUsageCount()`

UnsetUsageCount ensures that no value is present for UsageCount, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
