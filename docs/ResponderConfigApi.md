# \ResponderConfigApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_responder_configuration**](ResponderConfigApi.md#get_responder_configuration) | **GET** /responder/config/{responderConfigName} | Get a specific responder configuration
[**list_responder_configurations**](ResponderConfigApi.md#list_responder_configurations) | **GET** /responder/config | List all responder configurations for the user
[**update_responder_configuration**](ResponderConfigApi.md#update_responder_configuration) | **PUT** /responder/config/{responderConfigName} | Update or create a responder configuration



## get_responder_configuration

> models::BaseConfig get_responder_configuration(responder_config_name)
Get a specific responder configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**responder_config_name** | **String** | The name of the responder configuration. | [required] |

### Return type

[**models::BaseConfig**](BaseConfig.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_responder_configurations

> Vec<models::BaseConfig> list_responder_configurations()
List all responder configurations for the user

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::BaseConfig>**](BaseConfig.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_responder_configuration

> models::BaseConfig update_responder_configuration(responder_config_name, responder_config_update_request)
Update or create a responder configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**responder_config_name** | **String** | The name of the responder configuration. | [required] |
**responder_config_update_request** | [**ResponderConfigUpdateRequest**](ResponderConfigUpdateRequest.md) |  | [required] |

### Return type

[**models::BaseConfig**](BaseConfig.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

