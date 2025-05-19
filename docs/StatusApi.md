# \StatusApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_health_status**](StatusApi.md#get_health_status) | **GET** /status/health | Get system health status
[**get_status**](StatusApi.md#get_status) | **GET** /status | Get system status and configuration information
[**get_status_alerts**](StatusApi.md#get_status_alerts) | **GET** /status/alerts | Get system alerts



## get_health_status

> models::HealthResponse get_health_status()
Get system health status

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HealthResponse**](HealthResponse.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_status

> models::StatusResponse get_status()
Get system status and configuration information

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StatusResponse**](StatusResponse.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_status_alerts

> Vec<String> get_status_alerts()
Get system alerts

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

