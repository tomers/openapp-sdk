# PatchMeProfileRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Locale** | Pointer to **NullableString** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**ProfileSources** | Pointer to [**NullableProfileSourcePatch**](ProfileSourcePatch.md) |  | [optional]

## Methods

### NewPatchMeProfileRequest

`func NewPatchMeProfileRequest() *PatchMeProfileRequest`

NewPatchMeProfileRequest instantiates a new PatchMeProfileRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPatchMeProfileRequestWithDefaults

`func NewPatchMeProfileRequestWithDefaults() *PatchMeProfileRequest`

NewPatchMeProfileRequestWithDefaults instantiates a new PatchMeProfileRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLocale

`func (o *PatchMeProfileRequest) GetLocale() string`

GetLocale returns the Locale field if non-nil, zero value otherwise.

### GetLocaleOk

`func (o *PatchMeProfileRequest) GetLocaleOk() (*string, bool)`

GetLocaleOk returns a tuple with the Locale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocale

`func (o *PatchMeProfileRequest) SetLocale(v string)`

SetLocale sets Locale field to given value.

### HasLocale

`func (o *PatchMeProfileRequest) HasLocale() bool`

HasLocale returns a boolean if a field has been set.

### SetLocaleNil

`func (o *PatchMeProfileRequest) SetLocaleNil(b bool)`

 SetLocaleNil sets the value for Locale to be an explicit nil

### UnsetLocale
`func (o *PatchMeProfileRequest) UnsetLocale()`

UnsetLocale ensures that no value is present for Locale, not even an explicit nil
### GetName

`func (o *PatchMeProfileRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PatchMeProfileRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PatchMeProfileRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *PatchMeProfileRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *PatchMeProfileRequest) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *PatchMeProfileRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetProfileSources

`func (o *PatchMeProfileRequest) GetProfileSources() ProfileSourcePatch`

GetProfileSources returns the ProfileSources field if non-nil, zero value otherwise.

### GetProfileSourcesOk

`func (o *PatchMeProfileRequest) GetProfileSourcesOk() (*ProfileSourcePatch, bool)`

GetProfileSourcesOk returns a tuple with the ProfileSources field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProfileSources

`func (o *PatchMeProfileRequest) SetProfileSources(v ProfileSourcePatch)`

SetProfileSources sets ProfileSources field to given value.

### HasProfileSources

`func (o *PatchMeProfileRequest) HasProfileSources() bool`

HasProfileSources returns a boolean if a field has been set.

### SetProfileSourcesNil

`func (o *PatchMeProfileRequest) SetProfileSourcesNil(b bool)`

 SetProfileSourcesNil sets the value for ProfileSources to be an explicit nil

### UnsetProfileSources
`func (o *PatchMeProfileRequest) UnsetProfileSources()`

UnsetProfileSources ensures that no value is present for ProfileSources, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
