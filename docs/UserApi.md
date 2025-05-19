# \UserApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_user_password**](UserApi.md#change_user_password) | **POST** /user/{userId}/password/change | Change the current user's password
[**create_user**](UserApi.md#create_user) | **POST** /user | Create a new user
[**delete_user**](UserApi.md#delete_user) | **DELETE** /user/{userId} | Delete a user (marks as Locked)
[**find_users**](UserApi.md#find_users) | **POST** /user/_search | Find/Search users (SuperAdmin only)
[**find_users_for_organization**](UserApi.md#find_users_for_organization) | **POST** /user/organization/{organizationId}/_search | Find/Search users within a specific organization
[**get_current_user**](UserApi.md#get_current_user) | **GET** /user/current | Get the current logged-in user's details
[**get_user_api_key**](UserApi.md#get_user_api_key) | **GET** /user/{userId}/key | Get a user's API key
[**get_user_by_id**](UserApi.md#get_user_by_id) | **GET** /user/{userId} | Get a specific user by ID
[**remove_user_api_key**](UserApi.md#remove_user_api_key) | **DELETE** /user/{userId}/key | Remove a user's API key
[**renew_user_api_key**](UserApi.md#renew_user_api_key) | **POST** /user/{userId}/key/renew | Renew a user's API key
[**set_user_password**](UserApi.md#set_user_password) | **POST** /user/{userId}/password/set | Set a user's password (admin)
[**update_user**](UserApi.md#update_user) | **PUT** /user/{userId} | Update a user



## change_user_password

> change_user_password(user_id, password_change_request)
Change the current user's password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Must be the ID of the currently authenticated user. | [required] |
**password_change_request** | [**PasswordChangeRequest**](PasswordChangeRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> models::User create_user(user_create_request)
Create a new user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_create_request** | [**UserCreateRequest**](UserCreateRequest.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(user_id)
Delete a user (marks as Locked)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_users

> models::FindUsers200Response find_users(analyzer_find_request)
Find/Search users (SuperAdmin only)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyzer_find_request** | Option<[**AnalyzerFindRequest**](AnalyzerFindRequest.md)> |  |  |

### Return type

[**models::FindUsers200Response**](findUsers_200_response.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_users_for_organization

> models::FindUsers200Response find_users_for_organization(organization_id, analyzer_find_request)
Find/Search users within a specific organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |
**analyzer_find_request** | Option<[**AnalyzerFindRequest**](AnalyzerFindRequest.md)> |  |  |

### Return type

[**models::FindUsers200Response**](findUsers_200_response.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> models::User get_current_user()
Get the current logged-in user's details

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::User**](User.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_api_key

> String get_user_api_key(user_id)
Get a user's API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

**String**

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_id

> models::User get_user_by_id(user_id)
Get a specific user by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_api_key

> remove_user_api_key(user_id)
Remove a user's API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## renew_user_api_key

> String renew_user_api_key(user_id)
Renew a user's API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

**String**

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_password

> set_user_password(user_id, password_set_request)
Set a user's password (admin)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**password_set_request** | [**PasswordSetRequest**](PasswordSetRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::User update_user(user_id, user_update_request)
Update a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**user_update_request** | [**UserUpdateRequest**](UserUpdateRequest.md) |  | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

