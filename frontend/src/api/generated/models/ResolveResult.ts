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
 * The result of a resolver
 * @export
 * @interface ResolveResult
 */
export interface ResolveResult {
    /**
     * Ipv4 address
     * @type {string}
     * @memberof ResolveResult
     */
    ipv4?: string;
    /**
     * Ipv6 address
     * @type {string}
     * @memberof ResolveResult
     */
    ipv6?: string;
}

/**
 * Check if a given object implements the ResolveResult interface.
 */
export function instanceOfResolveResult(value: object): value is ResolveResult {
    return true;
}

export function ResolveResultFromJSON(json: any): ResolveResult {
    return ResolveResultFromJSONTyped(json, false);
}

export function ResolveResultFromJSONTyped(json: any, ignoreDiscriminator: boolean): ResolveResult {
    if (json == null) {
        return json;
    }
    return {
        
        'ipv4': json['ipv4'] == null ? undefined : json['ipv4'],
        'ipv6': json['ipv6'] == null ? undefined : json['ipv6'],
    };
}

export function ResolveResultToJSON(value?: ResolveResult | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'ipv4': value['ipv4'],
        'ipv6': value['ipv6'],
    };
}
