# AuditEventsPage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Items** | [**[]AuditRecord**](AuditRecord.md) |  |
**NextCursor** | Pointer to **NullableString** | Cursor for the next (older) page, or null when the last page was returned. | [optional]

## Methods

### NewAuditEventsPage

`func NewAuditEventsPage(items []AuditRecord, ) *AuditEventsPage`

NewAuditEventsPage instantiates a new AuditEventsPage object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAuditEventsPageWithDefaults

`func NewAuditEventsPageWithDefaults() *AuditEventsPage`

NewAuditEventsPageWithDefaults instantiates a new AuditEventsPage object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetItems

`func (o *AuditEventsPage) GetItems() []AuditRecord`

GetItems returns the Items field if non-nil, zero value otherwise.

### GetItemsOk

`func (o *AuditEventsPage) GetItemsOk() (*[]AuditRecord, bool)`

GetItemsOk returns a tuple with the Items field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetItems

`func (o *AuditEventsPage) SetItems(v []AuditRecord)`

SetItems sets Items field to given value.


### GetNextCursor

`func (o *AuditEventsPage) GetNextCursor() string`

GetNextCursor returns the NextCursor field if non-nil, zero value otherwise.

### GetNextCursorOk

`func (o *AuditEventsPage) GetNextCursorOk() (*string, bool)`

GetNextCursorOk returns a tuple with the NextCursor field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextCursor

`func (o *AuditEventsPage) SetNextCursor(v string)`

SetNextCursor sets NextCursor field to given value.

### HasNextCursor

`func (o *AuditEventsPage) HasNextCursor() bool`

HasNextCursor returns a boolean if a field has been set.

### SetNextCursorNil

`func (o *AuditEventsPage) SetNextCursorNil(b bool)`

 SetNextCursorNil sets the value for NextCursor to be an explicit nil

### UnsetNextCursor
`func (o *AuditEventsPage) UnsetNextCursor()`

UnsetNextCursor ensures that no value is present for NextCursor, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
