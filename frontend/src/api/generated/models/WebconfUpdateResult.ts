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

import type { WebconfUpdateResultOneOf } from './WebconfUpdateResultOneOf';
import {
    instanceOfWebconfUpdateResultOneOf,
    WebconfUpdateResultOneOfFromJSON,
    WebconfUpdateResultOneOfFromJSONTyped,
    WebconfUpdateResultOneOfToJSON,
} from './WebconfUpdateResultOneOf';
import type { WebconfUpdateResultOneOf1 } from './WebconfUpdateResultOneOf1';
import {
    instanceOfWebconfUpdateResultOneOf1,
    WebconfUpdateResultOneOf1FromJSON,
    WebconfUpdateResultOneOf1FromJSONTyped,
    WebconfUpdateResultOneOf1ToJSON,
} from './WebconfUpdateResultOneOf1';

/**
 * @type WebconfUpdateResult
 * The result of a webconf update request
 * @export
 */
export type WebconfUpdateResult = WebconfUpdateResultOneOf | WebconfUpdateResultOneOf1;

export function WebconfUpdateResultFromJSON(json: any): WebconfUpdateResult {
    return WebconfUpdateResultFromJSONTyped(json, false);
}

export function WebconfUpdateResultFromJSONTyped(json: any, ignoreDiscriminator: boolean): WebconfUpdateResult {
    if (json == null) {
        return json;
    }
    if (instanceOfWebconfUpdateResultOneOf(json)) {
        return WebconfUpdateResultOneOfFromJSONTyped(json, true);
    }
    if (instanceOfWebconfUpdateResultOneOf1(json)) {
        return WebconfUpdateResultOneOf1FromJSONTyped(json, true);
    }
}

export function WebconfUpdateResultToJSON(value?: WebconfUpdateResult | null): any {
    if (value == null) {
        return value;
    }

    if (instanceOfWebconfUpdateResultOneOf(value)) {
        return WebconfUpdateResultOneOfToJSON(value as WebconfUpdateResultOneOf);
    }
    if (instanceOfWebconfUpdateResultOneOf1(value)) {
        return WebconfUpdateResultOneOf1ToJSON(value as WebconfUpdateResultOneOf1);
    }

    return {};
}
