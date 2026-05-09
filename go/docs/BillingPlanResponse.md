# BillingPlanResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**OrgId** | **string** |  |
**Subscription** | Pointer to **interface{}** |  | [optional]
**TierId** | Pointer to **NullableString** |  | [optional]

## Methods

### NewBillingPlanResponse

`func NewBillingPlanResponse(orgId string, ) *BillingPlanResponse`

NewBillingPlanResponse instantiates a new BillingPlanResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewBillingPlanResponseWithDefaults

`func NewBillingPlanResponseWithDefaults() *BillingPlanResponse`

NewBillingPlanResponseWithDefaults instantiates a new BillingPlanResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetOrgId

`func (o *BillingPlanResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *BillingPlanResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *BillingPlanResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetSubscription

`func (o *BillingPlanResponse) GetSubscription() interface{}`

GetSubscription returns the Subscription field if non-nil, zero value otherwise.

### GetSubscriptionOk

`func (o *BillingPlanResponse) GetSubscriptionOk() (*interface{}, bool)`

GetSubscriptionOk returns a tuple with the Subscription field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSubscription

`func (o *BillingPlanResponse) SetSubscription(v interface{})`

SetSubscription sets Subscription field to given value.

### HasSubscription

`func (o *BillingPlanResponse) HasSubscription() bool`

HasSubscription returns a boolean if a field has been set.

### SetSubscriptionNil

`func (o *BillingPlanResponse) SetSubscriptionNil(b bool)`

 SetSubscriptionNil sets the value for Subscription to be an explicit nil

### UnsetSubscription
`func (o *BillingPlanResponse) UnsetSubscription()`

UnsetSubscription ensures that no value is present for Subscription, not even an explicit nil
### GetTierId

`func (o *BillingPlanResponse) GetTierId() string`

GetTierId returns the TierId field if non-nil, zero value otherwise.

### GetTierIdOk

`func (o *BillingPlanResponse) GetTierIdOk() (*string, bool)`

GetTierIdOk returns a tuple with the TierId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTierId

`func (o *BillingPlanResponse) SetTierId(v string)`

SetTierId sets TierId field to given value.

### HasTierId

`func (o *BillingPlanResponse) HasTierId() bool`

HasTierId returns a boolean if a field has been set.

### SetTierIdNil

`func (o *BillingPlanResponse) SetTierIdNil(b bool)`

 SetTierIdNil sets the value for TierId to be an explicit nil

### UnsetTierId
`func (o *BillingPlanResponse) UnsetTierId()`

UnsetTierId ensures that no value is present for TierId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
