/* tslint:disable */
/* eslint-disable */
/**
 * Frontend
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
/**
 * The request to login via LDAP
 * @export
 * @interface LdapLoginRequest
 */
export interface LdapLoginRequest {
    /**
     * Password
     * @type {string}
     * @memberof LdapLoginRequest
     */
    password: string;
    /**
     * Username
     * @type {string}
     * @memberof LdapLoginRequest
     */
    username: string;
}

/**
 * Check if a given object implements the LdapLoginRequest interface.
 */
export function instanceOfLdapLoginRequest(value: object): value is LdapLoginRequest {
    if (!('password' in value) || value['password'] === undefined) return false;
    if (!('username' in value) || value['username'] === undefined) return false;
    return true;
}

export function LdapLoginRequestFromJSON(json: any): LdapLoginRequest {
    return LdapLoginRequestFromJSONTyped(json, false);
}

export function LdapLoginRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): LdapLoginRequest {
    if (json == null) {
        return json;
    }
    return {
        
        'password': json['password'],
        'username': json['username'],
    };
}

export function LdapLoginRequestToJSON(value?: LdapLoginRequest | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'password': value['password'],
        'username': value['username'],
    };
}

