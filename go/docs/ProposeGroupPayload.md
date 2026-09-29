# ProposeGroupPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DisclosureAcknowledged** | **bool** |  |
**IntegrationId** | **string** | The site (integration) to propose the household to. |

## Methods

### NewProposeGroupPayload

`func NewProposeGroupPayload(disclosureAcknowledged bool, integrationId string, ) *ProposeGroupPayload`

NewProposeGroupPayload instantiates a new ProposeGroupPayload object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewProposeGroupPayloadWithDefaults

`func NewProposeGroupPayloadWithDefaults() *ProposeGroupPayload`

NewProposeGroupPayloadWithDefaults instantiates a new ProposeGroupPayload object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDisclosureAcknowledged

`func (o *ProposeGroupPayload) GetDisclosureAcknowledged() bool`

GetDisclosureAcknowledged returns the DisclosureAcknowledged field if non-nil, zero value otherwise.

### GetDisclosureAcknowledgedOk

`func (o *ProposeGroupPayload) GetDisclosureAcknowledgedOk() (*bool, bool)`

GetDisclosureAcknowledgedOk returns a tuple with the DisclosureAcknowledged field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisclosureAcknowledged

`func (o *ProposeGroupPayload) SetDisclosureAcknowledged(v bool)`

SetDisclosureAcknowledged sets DisclosureAcknowledged field to given value.


### GetIntegrationId

`func (o *ProposeGroupPayload) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *ProposeGroupPayload) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *ProposeGroupPayload) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
