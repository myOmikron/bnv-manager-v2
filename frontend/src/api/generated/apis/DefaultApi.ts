/* tslint:disable */
/* eslint-disable */
/**
 * Unnamed Galvyn API
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import type {
  AcceptInviteRequest,
  ApiErrorResponse,
  FormResultForNullAndLoginResponse,
  FullInvite,
  LoginRequest,
  Me,
} from '../models/index';

export interface AcceptInviteOperationRequest {
    uuid: string;
    AcceptInviteRequest?: AcceptInviteRequest;
}

export interface GetInviteRequest {
    uuid: string;
}

export interface LoginOperationRequest {
    LoginRequest?: LoginRequest;
}

/**
 * 
 */
export class DefaultApi extends runtime.BaseAPI {

    /**
     */
    async acceptInviteRaw(requestParameters: AcceptInviteOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters['uuid'] == null) {
            throw new runtime.RequiredError(
                'uuid',
                'Required parameter "uuid" was null or undefined when calling acceptInvite().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/frontend/invites/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters['uuid']))),
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: requestParameters['AcceptInviteRequest'],
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     */
    async acceptInvite(requestParameters: AcceptInviteOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.acceptInviteRaw(requestParameters, initOverrides);
    }

    /**
     */
    async getInviteRaw(requestParameters: GetInviteRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<FullInvite>> {
        if (requestParameters['uuid'] == null) {
            throw new runtime.RequiredError(
                'uuid',
                'Required parameter "uuid" was null or undefined when calling getInvite().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/frontend/invites/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters['uuid']))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response);
    }

    /**
     */
    async getInvite(requestParameters: GetInviteRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<FullInvite> {
        const response = await this.getInviteRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async getMeRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Me>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/frontend/common/users/me`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response);
    }

    /**
     */
    async getMe(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Me> {
        const response = await this.getMeRaw(initOverrides);
        return await response.value();
    }

    /**
     */
    async loginRaw(requestParameters: LoginOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<FormResultForNullAndLoginResponse>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/frontend/auth/login`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: requestParameters['LoginRequest'],
        }, initOverrides);

        return new runtime.JSONApiResponse(response);
    }

    /**
     */
    async login(requestParameters: LoginOperationRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<FormResultForNullAndLoginResponse> {
        const response = await this.loginRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async logoutRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/frontend/auth/logout`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     */
    async logout(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.logoutRaw(initOverrides);
    }

    /**
     */
    async openapiRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/frontend/openapi.json`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     */
    async openapi(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.openapiRaw(initOverrides);
    }

}
