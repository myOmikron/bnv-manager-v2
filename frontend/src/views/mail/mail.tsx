import React from "react";
import HeadingLayout from "../../components/heading-layout";

/**
 * The properties for {@link Mail}
 */
export type MailProps = {};

/**
 * Mailbox configuration view
 */
export default function Mail(props: MailProps) {
    return <HeadingLayout heading={"Mail configuration"}></HeadingLayout>;
}
