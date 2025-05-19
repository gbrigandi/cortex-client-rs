# \MispApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_misp_modules**](MispApi.md#list_misp_modules) | **POST** /misp/modules | List available MISP modules (Cortex analyzers)
[**query_misp_module**](MispApi.md#query_misp_module) | **POST** /misp/query | Query a MISP module (Cortex analyzer)



## list_misp_modules

> models::ListMispModules200Response list_misp_modules(request_body)
List available MISP modules (Cortex analyzers)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  |  |

### Return type

[**models::ListMispModules200Response**](listMispModules_200_response.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_misp_module

> models::MispQueryResponse query_misp_module(misp_query_request)
Query a MISP module (Cortex analyzer)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**misp_query_request** | [**MispQueryRequest**](MispQueryRequest.md) |  | [required] |

### Return type

[**models::MispQueryResponse**](MispQueryResponse.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

