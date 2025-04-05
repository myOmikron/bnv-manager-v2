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
  AdminUser,
  ApiErrorResponse,
  CreateClubRequest,
  FormResultForNullAndLoginResponse,
  FormResultForSingleUuidAndCreateClubResponseError,
  FullInvite,
  LoginRequest,
  Me,
  SimpleClub,
} from '../models/index';

export interface AcceptInviteOperationRequest {
    uuid: string;
    AcceptInviteRequest?: AcceptInviteRequest;
}

export interface AdminGetClubRequest {
    uuid: string;
}

export interface CreateClubOperationRequest {
    CreateClubRequest?: CreateClubRequest;
}

export interface DeleteClubRequest {
    uuid: string;
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
    async adminGetClubRaw(requestParameters: AdminGetClubRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<SimpleClub>> {
        if (requestParameters['uuid'] == null) {
            throw new runtime.RequiredError(
                'uuid',
                'Required parameter "uuid" was null or undefined when calling adminGetClub().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/frontend/admin/clubs/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters['uuid']))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response);
    }

    /**
     */
    async adminGetClub(requestParameters: AdminGetClubRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<SimpleClub> {
        const response = await this.adminGetClubRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async adminGetClubsRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<SimpleClub>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/frontend/admin/clubs`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response);
    }

    /**
     */
    async adminGetClubs(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<SimpleClub>> {
        const response = await this.adminGetClubsRaw(initOverrides);
        return await response.value();
    }

    /**
     */
    async createClubRaw(requestParameters: CreateClubOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<FormResultForSingleUuidAndCreateClubResponseError>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/frontend/admin/clubs`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: requestParameters['CreateClubRequest'],
        }, initOverrides);

        return new runtime.JSONApiResponse(response);
    }

    /**
     */
    async createClub(requestParameters: CreateClubOperationRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<FormResultForSingleUuidAndCreateClubResponseError> {
        const response = await this.createClubRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async deleteClubRaw(requestParameters: DeleteClubRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters['uuid'] == null) {
            throw new runtime.RequiredError(
                'uuid',
                'Required parameter "uuid" was null or undefined when calling deleteClub().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/frontend/admin/clubs/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters['uuid']))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     */
    async deleteClub(requestParameters: DeleteClubRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.deleteClubRaw(requestParameters, initOverrides);
    }

    /**
     */
    async getAdminsRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<AdminUser>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/frontend/admin/users/admins`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response);
    }

    /**
     */
    async getAdmins(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<AdminUser>> {
        const response = await this.getAdminsRaw(initOverrides);
        return await response.value();
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
