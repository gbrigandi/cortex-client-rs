# \AnalyzerConfigApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_analyzer_configuration**](AnalyzerConfigApi.md#get_analyzer_configuration) | **GET** /analyzer/config/{analyzerConfigName} | Get a specific analyzer configuration
[**list_analyzer_configurations**](AnalyzerConfigApi.md#list_analyzer_configurations) | **GET** /analyzer/config | List all analyzer configurations for the user
[**update_analyzer_configuration**](AnalyzerConfigApi.md#update_analyzer_configuration) | **PUT** /analyzer/config/{analyzerConfigName} | Update or create an analyzer configuration



## get_analyzer_configuration

> models::BaseConfig get_analyzer_configuration(analyzer_config_name)
Get a specific analyzer configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyzer_config_name** | **String** | The name of the analyzer configuration | [required] |

### Return type

[**models::BaseConfig**](BaseConfig.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_analyzer_configurations

> Vec<models::BaseConfig> list_analyzer_configurations()
List all analyzer configurations for the user

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


## update_analyzer_configuration

> models::BaseConfig update_analyzer_configuration(analyzer_config_name, analyzer_config_update_request)
Update or create an analyzer configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyzer_config_name** | **String** | The name of the analyzer configuration | [required] |
**analyzer_config_update_request** | [**AnalyzerConfigUpdateRequest**](AnalyzerConfigUpdateRequest.md) |  | [required] |

### Return type

[**models::BaseConfig**](BaseConfig.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

