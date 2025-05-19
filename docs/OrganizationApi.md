# \OrganizationApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization**](OrganizationApi.md#create_organization) | **POST** /organization | Create a new organization
[**delete_organization**](OrganizationApi.md#delete_organization) | **DELETE** /organization/{organizationId} | Delete an organization
[**find_organizations**](OrganizationApi.md#find_organizations) | **POST** /organization/_search | Find/Search organizations
[**get_organization_by_id**](OrganizationApi.md#get_organization_by_id) | **GET** /organization/{organizationId} | Get a specific organization by ID
[**get_organization_stats**](OrganizationApi.md#get_organization_stats) | **POST** /organization/_stats | Get statistics for organizations
[**update_organization**](OrganizationApi.md#update_organization) | **PUT** /organization/{organizationId} | Update an organization



## create_organization

> models::Organization create_organization(organization_create_request)
Create a new organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_create_request** | [**OrganizationCreateRequest**](OrganizationCreateRequest.md) |  | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization

> delete_organization(organization_id)
Delete an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_organizations

> models::FindOrganizations200Response find_organizations(organization_find_request)
Find/Search organizations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_find_request** | Option<[**OrganizationFindRequest**](OrganizationFindRequest.md)> |  |  |

### Return type

[**models::FindOrganizations200Response**](findOrganizations_200_response.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_by_id

> models::Organization get_organization_by_id(organization_id, nstats)
Get a specific organization by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**nstats** | Option<**bool**> | Whether to include statistics. |  |[default to false]

### Return type

[**models::Organization**](Organization.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_stats

> std::collections::HashMap<String, serde_json::Value> get_organization_stats(organization_stats_request)
Get statistics for organizations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_stats_request** | [**OrganizationStatsRequest**](OrganizationStatsRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization

> models::Organization update_organization(organization_id, organization_update_request)
Update an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**organization_update_request** | [**OrganizationUpdateRequest**](OrganizationUpdateRequest.md) |  | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

