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
 * 
 * @export
 * @interface WebconfUpdateResultOneOf1
 */
export interface WebconfUpdateResultOneOf1 {
    /**
     * 
     * @type {string}
     * @memberof WebconfUpdateResultOneOf1
     */
    res: WebconfUpdateResultOneOf1ResEnum;
}


/**
 * @export
 */
export const WebconfUpdateResultOneOf1ResEnum = {
    Fail: 'Fail'
} as const;
export type WebconfUpdateResultOneOf1ResEnum = typeof WebconfUpdateResultOneOf1ResEnum[keyof typeof WebconfUpdateResultOneOf1ResEnum];


/**
 * Check if a given object implements the WebconfUpdateResultOneOf1 interface.
 */
export function instanceOfWebconfUpdateResultOneOf1(value: object): value is WebconfUpdateResultOneOf1 {
    if (!('res' in value) || value['res'] === undefined) return false;
    return true;
}

export function WebconfUpdateResultOneOf1FromJSON(json: any): WebconfUpdateResultOneOf1 {
    return WebconfUpdateResultOneOf1FromJSONTyped(json, false);
}

export function WebconfUpdateResultOneOf1FromJSONTyped(json: any, ignoreDiscriminator: boolean): WebconfUpdateResultOneOf1 {
    if (json == null) {
        return json;
    }
    return {
        
        'res': json['res'],
    };
}

export function WebconfUpdateResultOneOf1ToJSON(value?: WebconfUpdateResultOneOf1 | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'res': value['res'],
    };
}
