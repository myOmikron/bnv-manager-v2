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
import type { SimpleWebsite } from './SimpleWebsite';
import {
    SimpleWebsiteFromJSON,
    SimpleWebsiteFromJSONTyped,
    SimpleWebsiteToJSON,
} from './SimpleWebsite';

/**
 * A list of websites
 * @export
 * @interface ListWebsites
 */
export interface ListWebsites {
    /**
     * The list of websites
     * @type {Array<SimpleWebsite>}
     * @memberof ListWebsites
     */
    websites: Array<SimpleWebsite>;
}

/**
 * Check if a given object implements the ListWebsites interface.
 */
export function instanceOfListWebsites(value: object): value is ListWebsites {
    if (!('websites' in value) || value['websites'] === undefined) return false;
    return true;
}

export function ListWebsitesFromJSON(json: any): ListWebsites {
    return ListWebsitesFromJSONTyped(json, false);
}

export function ListWebsitesFromJSONTyped(json: any, ignoreDiscriminator: boolean): ListWebsites {
    if (json == null) {
        return json;
    }
    return {
        
        'websites': ((json['websites'] as Array<any>).map(SimpleWebsiteFromJSON)),
    };
}

export function ListWebsitesToJSON(value?: ListWebsites | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'websites': ((value['websites'] as Array<any>).map(SimpleWebsiteToJSON)),
    };
}

