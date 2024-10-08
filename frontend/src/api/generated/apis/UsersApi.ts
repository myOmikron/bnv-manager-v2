/* tslint:disable */
/* eslint-disable */
/**
 * Unnamed API
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
  ApiErrorResponse,
  ChangeMeRequest,
  ChangePwRequest,
  FormResultForNullAndChangePwErrors,
  FullUser,
  SimpleUser,
} from '../models/index';

export interface ChangePasswordRequest {
    ChangePwRequest: ChangePwRequest;
}

export interface DeleteClubUserRequest {
    uuid: string;
}

export interface UpdateMeRequest {
    ChangeMeRequest: ChangeMeRequest;
}

/**
 * 
 */
export class UsersApi extends runtime.BaseAPI {

    /**
     * Change the password of the currently logged-in user  This may only be called by local users
     * Change the password of the currently logged-in user
     */
    async changePasswordRaw(requestParameters: ChangePasswordRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<FormResultForNullAndChangePwErrors>> {
        if (requestParameters['ChangePwRequest'] == null) {
            throw new runtime.RequiredError(
                'ChangePwRequest',
                'Required parameter "ChangePwRequest" was null or undefined when calling changePassword().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/frontend/v1/common/users/me/change-pw`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: requestParameters['ChangePwRequest'],
        }, initOverrides);

        return new runtime.JSONApiResponse(response);
    }

    /**
     * Change the password of the currently logged-in user  This may only be called by local users
     * Change the password of the currently logged-in user
     */
    async changePassword(requestParameters: ChangePasswordRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<FormResultForNullAndChangePwErrors> {
        const response = await this.changePasswordRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Retrieve the users of a club
     * Retrieve the users of a club
     */
    async deleteClubUserRaw(requestParameters: DeleteClubUserRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters['uuid'] == null) {
            throw new runtime.RequiredError(
                'uuid',
                'Required parameter "uuid" was null or undefined when calling deleteClubUser().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/frontend/v1/club-admin/users/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters['uuid']))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Retrieve the users of a club
     * Retrieve the users of a club
     */
    async deleteClubUser(requestParameters: DeleteClubUserRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.deleteClubUserRaw(requestParameters, initOverrides);
    }

    /**
     * Export all users of the club as csv
     * Export all users of the club as csv
     */
    async exportCsvCaRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/frontend/v1/club-admin/users/export/csv`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Export all users of the club as csv
     * Export all users of the club as csv
     */
    async exportCsvCa(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.exportCsvCaRaw(initOverrides);
    }

    /**
     * Export all users of the club as json
     * Export all users of the club as json
     */
    async exportJsonCaRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/frontend/v1/club-admin/users/export/json`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Export all users of the club as json
     * Export all users of the club as json
     */
    async exportJsonCa(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.exportJsonCaRaw(initOverrides);
    }

    /**
     * Retrieve the users of a club
     * Retrieve the users of a club
     */
    async getClubUsersClubAdminRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<SimpleUser>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/frontend/v1/club-admin/users`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response);
    }

    /**
     * Retrieve the users of a club
     * Retrieve the users of a club
     */
    async getClubUsersClubAdmin(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<SimpleUser>> {
        const response = await this.getClubUsersClubAdminRaw(initOverrides);
        return await response.value();
    }

    /**
     * Retrieve the currently logged-in user
     * Retrieve the currently logged-in user
     */
    async getMeRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<FullUser>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/frontend/v1/common/users/me`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response);
    }

    /**
     * Retrieve the currently logged-in user
     * Retrieve the currently logged-in user
     */
    async getMe(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<FullUser> {
        const response = await this.getMeRaw(initOverrides);
        return await response.value();
    }

    /**
     * Updates the current user information
     * Updates the current user information
     */
    async updateMeRaw(requestParameters: UpdateMeRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters['ChangeMeRequest'] == null) {
            throw new runtime.RequiredError(
                'ChangeMeRequest',
                'Required parameter "ChangeMeRequest" was null or undefined when calling updateMe().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/frontend/v1/common/users/me`,
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: requestParameters['ChangeMeRequest'],
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Updates the current user information
     * Updates the current user information
     */
    async updateMe(requestParameters: UpdateMeRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.updateMeRaw(requestParameters, initOverrides);
    }

}
