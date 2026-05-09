# BillingEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExternalCustomerId** | Pointer to **NullableString** |  | [optional]
**ExternalSubscriptionId** | Pointer to **NullableString** |  | [optional]
**Kind** | [**BillingEventKind**](BillingEventKind.md) |  |
**Provider** | **string** |  |
**Raw** | **interface{}** |  |
**Status** | Pointer to **NullableString** |  | [optional]
**TierSlug** | Pointer to **NullableString** |  | [optional]

## Methods

### NewBillingEvent

`func NewBillingEvent(kind BillingEventKind, provider string, raw interface{}, ) *BillingEvent`

NewBillingEvent instantiates a new BillingEvent object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewBillingEventWithDefaults

`func NewBillingEventWithDefaults() *BillingEvent`

NewBillingEventWithDefaults instantiates a new BillingEvent object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExternalCustomerId

`func (o *BillingEvent) GetExternalCustomerId() string`

GetExternalCustomerId returns the ExternalCustomerId field if non-nil, zero value otherwise.

### GetExternalCustomerIdOk

`func (o *BillingEvent) GetExternalCustomerIdOk() (*string, bool)`

GetExternalCustomerIdOk returns a tuple with the ExternalCustomerId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalCustomerId

`func (o *BillingEvent) SetExternalCustomerId(v string)`

SetExternalCustomerId sets ExternalCustomerId field to given value.

### HasExternalCustomerId

`func (o *BillingEvent) HasExternalCustomerId() bool`

HasExternalCustomerId returns a boolean if a field has been set.

### SetExternalCustomerIdNil

`func (o *BillingEvent) SetExternalCustomerIdNil(b bool)`

 SetExternalCustomerIdNil sets the value for ExternalCustomerId to be an explicit nil

### UnsetExternalCustomerId
`func (o *BillingEvent) UnsetExternalCustomerId()`

UnsetExternalCustomerId ensures that no value is present for ExternalCustomerId, not even an explicit nil
### GetExternalSubscriptionId

`func (o *BillingEvent) GetExternalSubscriptionId() string`

GetExternalSubscriptionId returns the ExternalSubscriptionId field if non-nil, zero value otherwise.

### GetExternalSubscriptionIdOk

`func (o *BillingEvent) GetExternalSubscriptionIdOk() (*string, bool)`

GetExternalSubscriptionIdOk returns a tuple with the ExternalSubscriptionId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalSubscriptionId

`func (o *BillingEvent) SetExternalSubscriptionId(v string)`

SetExternalSubscriptionId sets ExternalSubscriptionId field to given value.

### HasExternalSubscriptionId

`func (o *BillingEvent) HasExternalSubscriptionId() bool`

HasExternalSubscriptionId returns a boolean if a field has been set.

### SetExternalSubscriptionIdNil

`func (o *BillingEvent) SetExternalSubscriptionIdNil(b bool)`

 SetExternalSubscriptionIdNil sets the value for ExternalSubscriptionId to be an explicit nil

### UnsetExternalSubscriptionId
`func (o *BillingEvent) UnsetExternalSubscriptionId()`

UnsetExternalSubscriptionId ensures that no value is present for ExternalSubscriptionId, not even an explicit nil
### GetKind

`func (o *BillingEvent) GetKind() BillingEventKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *BillingEvent) GetKindOk() (*BillingEventKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *BillingEvent) SetKind(v BillingEventKind)`

SetKind sets Kind field to given value.


### GetProvider

`func (o *BillingEvent) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *BillingEvent) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *BillingEvent) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetRaw

`func (o *BillingEvent) GetRaw() interface{}`

GetRaw returns the Raw field if non-nil, zero value otherwise.

### GetRawOk

`func (o *BillingEvent) GetRawOk() (*interface{}, bool)`

GetRawOk returns a tuple with the Raw field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRaw

`func (o *BillingEvent) SetRaw(v interface{})`

SetRaw sets Raw field to given value.


### SetRawNil

`func (o *BillingEvent) SetRawNil(b bool)`

 SetRawNil sets the value for Raw to be an explicit nil

### UnsetRaw
`func (o *BillingEvent) UnsetRaw()`

UnsetRaw ensures that no value is present for Raw, not even an explicit nil
### GetStatus

`func (o *BillingEvent) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *BillingEvent) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *BillingEvent) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *BillingEvent) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### SetStatusNil

`func (o *BillingEvent) SetStatusNil(b bool)`

 SetStatusNil sets the value for Status to be an explicit nil

### UnsetStatus
`func (o *BillingEvent) UnsetStatus()`

UnsetStatus ensures that no value is present for Status, not even an explicit nil
### GetTierSlug

`func (o *BillingEvent) GetTierSlug() string`

GetTierSlug returns the TierSlug field if non-nil, zero value otherwise.

### GetTierSlugOk

`func (o *BillingEvent) GetTierSlugOk() (*string, bool)`

GetTierSlugOk returns a tuple with the TierSlug field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTierSlug

`func (o *BillingEvent) SetTierSlug(v string)`

SetTierSlug sets TierSlug field to given value.

### HasTierSlug

`func (o *BillingEvent) HasTierSlug() bool`

HasTierSlug returns a boolean if a field has been set.

### SetTierSlugNil

`func (o *BillingEvent) SetTierSlugNil(b bool)`

 SetTierSlugNil sets the value for TierSlug to be an explicit nil

### UnsetTierSlug
`func (o *BillingEvent) UnsetTierSlug()`

UnsetTierSlug ensures that no value is present for TierSlug, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
