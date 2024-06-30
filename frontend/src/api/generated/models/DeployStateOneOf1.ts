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
 * There are pending changes
 * @export
 * @interface DeployStateOneOf1
 */
export interface DeployStateOneOf1 {
    /**
     * 
     * @type {string}
     * @memberof DeployStateOneOf1
     */
    type: DeployStateOneOf1TypeEnum;
}


/**
 * @export
 */
export const DeployStateOneOf1TypeEnum = {
    PendingChanges: 'PendingChanges'
} as const;
export type DeployStateOneOf1TypeEnum = typeof DeployStateOneOf1TypeEnum[keyof typeof DeployStateOneOf1TypeEnum];


/**
 * Check if a given object implements the DeployStateOneOf1 interface.
 */
export function instanceOfDeployStateOneOf1(value: object): value is DeployStateOneOf1 {
    if (!('type' in value) || value['type'] === undefined) return false;
    return true;
}

export function DeployStateOneOf1FromJSON(json: any): DeployStateOneOf1 {
    return DeployStateOneOf1FromJSONTyped(json, false);
}

export function DeployStateOneOf1FromJSONTyped(json: any, ignoreDiscriminator: boolean): DeployStateOneOf1 {
    if (json == null) {
        return json;
    }
    return {
        
        'type': json['type'],
    };
}

export function DeployStateOneOf1ToJSON(value?: DeployStateOneOf1 | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'type': value['type'],
    };
}
