# CreateAuditExportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Format** | Pointer to **NullableString** | Output format: &#x60;jsonl&#x60; (default) or &#x60;csv&#x60;. | [optional]
**OccurredAfter** | Pointer to **NullableString** | RFC 3339 lower bound (inclusive). Defaults to the start of retention. | [optional]
**OccurredBefore** | Pointer to **NullableString** | RFC 3339 upper bound (inclusive). Defaults to now. | [optional]
**ZoneId** | Pointer to **NullableString** | Zone ULID: export only events whose zone snapshot (&#x60;details.zone_path&#x60;) contains it (the zone and its descendants). 400 when not a ULID. | [optional]

## Methods

### NewCreateAuditExportRequest

`func NewCreateAuditExportRequest() *CreateAuditExportRequest`

NewCreateAuditExportRequest instantiates a new CreateAuditExportRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateAuditExportRequestWithDefaults

`func NewCreateAuditExportRequestWithDefaults() *CreateAuditExportRequest`

NewCreateAuditExportRequestWithDefaults instantiates a new CreateAuditExportRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetFormat

`func (o *CreateAuditExportRequest) GetFormat() string`

GetFormat returns the Format field if non-nil, zero value otherwise.

### GetFormatOk

`func (o *CreateAuditExportRequest) GetFormatOk() (*string, bool)`

GetFormatOk returns a tuple with the Format field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFormat

`func (o *CreateAuditExportRequest) SetFormat(v string)`

SetFormat sets Format field to given value.

### HasFormat

`func (o *CreateAuditExportRequest) HasFormat() bool`

HasFormat returns a boolean if a field has been set.

### SetFormatNil

`func (o *CreateAuditExportRequest) SetFormatNil(b bool)`

 SetFormatNil sets the value for Format to be an explicit nil

### UnsetFormat
`func (o *CreateAuditExportRequest) UnsetFormat()`

UnsetFormat ensures that no value is present for Format, not even an explicit nil
### GetOccurredAfter

`func (o *CreateAuditExportRequest) GetOccurredAfter() string`

GetOccurredAfter returns the OccurredAfter field if non-nil, zero value otherwise.

### GetOccurredAfterOk

`func (o *CreateAuditExportRequest) GetOccurredAfterOk() (*string, bool)`

GetOccurredAfterOk returns a tuple with the OccurredAfter field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOccurredAfter

`func (o *CreateAuditExportRequest) SetOccurredAfter(v string)`

SetOccurredAfter sets OccurredAfter field to given value.

### HasOccurredAfter

`func (o *CreateAuditExportRequest) HasOccurredAfter() bool`

HasOccurredAfter returns a boolean if a field has been set.

### SetOccurredAfterNil

`func (o *CreateAuditExportRequest) SetOccurredAfterNil(b bool)`

 SetOccurredAfterNil sets the value for OccurredAfter to be an explicit nil

### UnsetOccurredAfter
`func (o *CreateAuditExportRequest) UnsetOccurredAfter()`

UnsetOccurredAfter ensures that no value is present for OccurredAfter, not even an explicit nil
### GetOccurredBefore

`func (o *CreateAuditExportRequest) GetOccurredBefore() string`

GetOccurredBefore returns the OccurredBefore field if non-nil, zero value otherwise.

### GetOccurredBeforeOk

`func (o *CreateAuditExportRequest) GetOccurredBeforeOk() (*string, bool)`

GetOccurredBeforeOk returns a tuple with the OccurredBefore field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOccurredBefore

`func (o *CreateAuditExportRequest) SetOccurredBefore(v string)`

SetOccurredBefore sets OccurredBefore field to given value.

### HasOccurredBefore

`func (o *CreateAuditExportRequest) HasOccurredBefore() bool`

HasOccurredBefore returns a boolean if a field has been set.

### SetOccurredBeforeNil

`func (o *CreateAuditExportRequest) SetOccurredBeforeNil(b bool)`

 SetOccurredBeforeNil sets the value for OccurredBefore to be an explicit nil

### UnsetOccurredBefore
`func (o *CreateAuditExportRequest) UnsetOccurredBefore()`

UnsetOccurredBefore ensures that no value is present for OccurredBefore, not even an explicit nil
### GetZoneId

`func (o *CreateAuditExportRequest) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *CreateAuditExportRequest) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *CreateAuditExportRequest) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.

### HasZoneId

`func (o *CreateAuditExportRequest) HasZoneId() bool`

HasZoneId returns a boolean if a field has been set.

### SetZoneIdNil

`func (o *CreateAuditExportRequest) SetZoneIdNil(b bool)`

 SetZoneIdNil sets the value for ZoneId to be an explicit nil

### UnsetZoneId
`func (o *CreateAuditExportRequest) UnsetZoneId()`

UnsetZoneId ensures that no value is present for ZoneId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
