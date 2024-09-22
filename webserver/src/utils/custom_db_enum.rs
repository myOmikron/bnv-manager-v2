//! Better [`DbEnum`](rorm::DbEnum)™

use std::str::FromStr;

use futures::future::BoxFuture;
use rorm::db::transaction::Transaction;
use strum::IntoEnumIterator;

/// Wrapper around `Option<T>` to be able to implement `FieldType` on it.
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CustomDbOption<T>(pub Option<T>);

/// Slice of all custom db enum's `migrate_db` functions
///
/// They are collected automatically from all over the codebase and executed in the `migrate` cli command.
#[linkme::distributed_slice]
pub static MIGRATE_DB: [fn(&mut Transaction) -> BoxFuture<'_, Result<(), rorm::Error>>] = [..];

/// Custom alternative to [`DbEnum`](rorm::DbEnum).
///
/// This macro implements `FieldType` for a basic enum, just like `DbEnum`.
/// But instead of using a custom db type (like `DbEnum`), this macro creates
/// a single-column model which stores the variants.
///
/// It also adds to methods `check_db` and `migrate_db` to manage this special model.
#[macro_export]
macro_rules! custom_db_enum {
    ($Enum:ident, $name:literal) => {
        const _: () = {
            $crate::utils::custom_db_enum::test_strum_derives::<$Enum>();
            impl ::rorm::fields::traits::FieldType for $Enum {
                type Columns<T> = [T; 1];

                fn into_values(self) -> Self::Columns<::rorm::conditions::Value<'static>> {
                    [::rorm::conditions::Value::String(
                        ::std::borrow::Cow::Borrowed(self.into()),
                    )]
                }

                fn as_values(&self) -> Self::Columns<::rorm::conditions::Value<'static>> {
                    [::rorm::conditions::Value::String(
                        ::std::borrow::Cow::Borrowed(self.into()),
                    )]
                }

                fn get_imr<F: ::rorm::internal::field::Field<Type = Self>>() -> Self::Columns<rorm::imr::Field>
                {
                    use ::rorm::internal::hmr::AsImr;
                    [::rorm::internal::imr::Field {
                        name: F::NAME.to_string(),
                        db_type: <::rorm::internal::hmr::db_type::VarChar as ::rorm::internal::hmr::db_type::DbType>::IMR,
                        annotations: F::EFFECTIVE_ANNOTATIONS
                            .unwrap_or_else(::rorm::internal::hmr::annotations::Annotations::empty)
                            .as_imr(),
                        source_defined_at: F::SOURCE.map(|s| s.as_imr()),
                    }]
                }

                type Decoder = __EnumDecoder;
                type AnnotationsModifier<F: ::rorm::internal::field::Field<Type = Self>> =
                    ::rorm::internal::field::modifier::MergeAnnotations<Self>;
                type CheckModifier<F: ::rorm::internal::field::Field<Type = Self>> =
                    ::rorm::internal::field::modifier::SingleColumnCheck<
                        ::rorm::internal::hmr::db_type::VarChar,
                    >;
                type ColumnsFromName<F: ::rorm::internal::field::Field<Type = Self>> =
                    ::rorm::internal::field::modifier::SingleColumnFromName;
            }
            impl ::rorm::internal::field::as_db_type::AsDbType for $Enum {
                type Primitive = ::std::string::String;
                type DbType = ::rorm::internal::hmr::db_type::VarChar;
                const IMPLICIT: ::std::option::Option<::rorm::internal::hmr::annotations::Annotations> =
                    ::std::option::Option::Some(::rorm::internal::hmr::annotations::Annotations {
                        max_length: ::std::option::Option::Some(::rorm::internal::hmr::annotations::MaxLength(255)),
                        foreign: ::std::option::Option::Some(::rorm::internal::hmr::annotations::ForeignKey {
                            table_name: $name,
                            column_name: "identifier",
                        }),
                        on_update: ::std::option::Option::Some(::rorm::imr::ReferentialAction::Cascade),
                        on_delete: ::std::option::Option::Some(::rorm::imr::ReferentialAction::Restrict),
                        ..::rorm::internal::hmr::annotations::Annotations::empty()
                    });

                fn from_primitive(_: Self::Primitive) -> Self {
                    panic!(
                        "Called <{} as AsDbType>::from_primitive, this is a bug!",
                        ::std::any::type_name::<Self>()
                    )
                }
            }
            ::rorm::impl_FieldEq!(
                impl<'rhs> FieldEq<'rhs, Self> for $Enum {
                    |kind: Self| ::rorm::conditions::Value::String(
                        ::std::borrow::Cow::Borrowed(kind.into()),
                    )
                }
            );
            ::rorm::new_converting_decoder! {
                pub __EnumDecoder,
                |string: ::std::string::String| -> $Enum {
                    string.parse()
                        .map_err(|_|
                            ::rorm::Error::DecodeError(
                                ::std::format!(
                                    "Invalid value {string} for enum {}",
                                    ::std::any::type_name::<$Enum>(),
                                )
                            )
                        )
                }
            }

            impl ::rorm::fields::traits::FieldType for $crate::utils::custom_db_enum::CustomDbOption<$Enum> {
                type Columns<T> = [T; 1];

                fn into_values(self) -> Self::Columns<::rorm::conditions::Value<'static>> {
                    [match self {
                        Self(::std::option::Option::Some(value)) => ::rorm::conditions::Value::String(
                            ::std::borrow::Cow::Borrowed(value.into()),
                        ),
                        Self(::std::option::Option::None) => ::rorm::conditions::Value::Null(
                            ::rorm::db::sql::value::NullType::String,
                        ),
                    }]
                }

                fn as_values(&self) -> Self::Columns<::rorm::conditions::Value<'static>> {
                    [match self {
                        Self(::std::option::Option::Some(value)) => ::rorm::conditions::Value::String(
                            ::std::borrow::Cow::Borrowed(value.into()),
                        ),
                        Self(::std::option::Option::None) => ::rorm::conditions::Value::Null(
                            ::rorm::db::sql::value::NullType::String,
                        ),
                    }]
                }

                fn get_imr<F: ::rorm::internal::field::Field<Type = Self>>() -> Self::Columns<rorm::imr::Field>
                {
                    use ::rorm::internal::hmr::AsImr;
                    [::rorm::internal::imr::Field {
                        name: F::NAME.to_string(),
                        db_type: <::rorm::internal::hmr::db_type::VarChar as ::rorm::internal::hmr::db_type::DbType>::IMR,
                        annotations: F::EFFECTIVE_ANNOTATIONS
                            .unwrap_or_else(::rorm::internal::hmr::annotations::Annotations::empty)
                            .as_imr(),
                        source_defined_at: F::SOURCE.map(|s| s.as_imr()),
                    }]
                }

                type Decoder = __OptionEnumDecoder;
                type AnnotationsModifier<F: ::rorm::internal::field::Field<Type = Self>> =
                    ::rorm::internal::field::modifier::MergeAnnotations<Self>;
                type CheckModifier<F: ::rorm::internal::field::Field<Type = Self>> =
                    ::rorm::internal::field::modifier::SingleColumnCheck<
                        ::rorm::internal::hmr::db_type::VarChar,
                    >;
                type ColumnsFromName<F: ::rorm::internal::field::Field<Type = Self>> =
                    ::rorm::internal::field::modifier::SingleColumnFromName;
            }
            impl ::rorm::internal::field::as_db_type::AsDbType for $crate::utils::custom_db_enum::CustomDbOption<$Enum> {
                type Primitive = ::std::option::Option<::std::string::String>;
                type DbType = ::rorm::internal::hmr::db_type::VarChar;
                const IMPLICIT: ::std::option::Option<::rorm::internal::hmr::annotations::Annotations> =
                    ::std::option::Option::Some(::rorm::internal::hmr::annotations::Annotations {
                        nullable: true,
                        max_length: ::std::option::Option::Some(::rorm::internal::hmr::annotations::MaxLength(255)),
                        foreign: ::std::option::Option::Some(::rorm::internal::hmr::annotations::ForeignKey {
                            table_name: $name,
                            column_name: "identifier",
                        }),
                        on_update: ::std::option::Option::Some(::rorm::imr::ReferentialAction::Cascade),
                        on_delete: ::std::option::Option::Some(::rorm::imr::ReferentialAction::Restrict),
                        ..::rorm::internal::hmr::annotations::Annotations::empty()
                    });

                fn from_primitive(_: Self::Primitive) -> Self {
                    panic!(
                        "Called <{} as AsDbType>::from_primitive, this is a bug!",
                        ::std::any::type_name::<Self>()
                    )
                }
            }
            ::rorm::impl_FieldEq!(
                impl<'rhs> FieldEq<'rhs, $Enum> for $crate::utils::custom_db_enum::CustomDbOption<$Enum> {
                    |kind: $Enum| ::rorm::conditions::Value::String(
                        ::std::borrow::Cow::Borrowed(kind.into()),
                    )
                }
            );
            ::rorm::new_converting_decoder! {
                pub __OptionEnumDecoder,
                |option: ::std::option::Option<::std::string::String>| -> $crate::utils::custom_db_enum::CustomDbOption<$Enum> {
                    option
                        .map(|string| string.parse()
                            .map_err(|_|
                                ::rorm::Error::DecodeError(
                                    ::std::format!(
                                        "Invalid value {string} for enum {}",
                                        ::std::any::type_name::<$Enum>(),
                                    )
                                )
                            )
                        )
                        .transpose()
                        .map($crate::utils::custom_db_enum::CustomDbOption)
                }
            }

            #[doc = concat!("The [`Model`](rorm::Model) storing [`", $name, "`]'s values in the database")]
            #[derive(rorm::Model)]
            #[rorm(rename = $name)]
            pub struct __EnumModel {
                #[doc = concat!("The string values of [`", $name, "`]")]
                #[rorm(primary_key, max_length = 255)]
                pub identifier: ::std::string::String,
            }
            impl $Enum {
                /// Checks whether the database rows match the rust variants
                #[::tracing::instrument(skip_all)]
                pub async fn check_db(executor: impl ::rorm::db::Executor<'_>) -> ::std::result::Result<::std::result::Result<(), ()>, ::rorm::Error> {
                    let db_state: ::std::vec::Vec<__EnumModel> = ::rorm::query!(executor, __EnumModel).all().await?;
                    let db_state: ::std::collections::HashSet<::std::string::String> =
                        db_state.into_iter().map(|__EnumModel { identifier }| identifier).collect();

                    let mut result = Ok(());

                    for variant in &db_state {
                        if variant.parse::<Self>().is_err() {
                            ::tracing::error!("Variant {variant:?} exists in the database, but not in the code");
                            result = Err(());
                        }
                    }

                    for variant in <Self as ::strum::IntoEnumIterator>::iter() {
                        let variant = <&'static str as From<Self>>::from(variant);
                        if !db_state.contains(variant) {
                            ::tracing::error!("Variant {variant:?} exists in the code, but not in the database");
                            result = Err(());
                        }
                    }

                    Ok(result)
                }

                /// Creates and deletes database rows to match the rust variants
                ///
                /// This function doesn't check whether rows to be deleted are still in use.
                /// In this case, a database error will be returned.
                #[::tracing::instrument(skip_all)]
                pub async fn migrate_db(executor: impl ::rorm::db::Executor<'_>) -> ::std::result::Result<(), ::rorm::Error> {
                    let mut guard = executor.ensure_transaction().await?;

                    let db_state: ::std::vec::Vec<__EnumModel> = ::rorm::query!(guard.get_transaction(), __EnumModel).all().await?;
                    let db_state: ::std::collections::HashSet<::std::string::String> =
                        db_state.into_iter().map(|__EnumModel { identifier }| identifier).collect();

                    let mut to_delete: ::std::vec::Vec<String> = ::std::vec::Vec::new();
                    let mut to_insert: ::std::vec::Vec<String> = ::std::vec::Vec::new();

                    for variant in &db_state {
                        if variant.parse::<Self>().is_err() {
                            to_delete.push(variant.clone());
                        }
                    }

                    for variant in <Self as ::strum::IntoEnumIterator>::iter() {
                        let variant = <&'static str as From<Self>>::from(variant);
                        if !db_state.contains(variant) {
                            to_insert.push(variant.to_string());
                        }
                    }

                    if !to_delete.is_empty() {
                        ::rorm::delete!(guard.get_transaction(), __EnumModel)
                            .condition(::rorm::conditions::collections::DynamicCollection::or(
                                to_delete.into_iter()
                                    .map(|identifier| ::rorm::FieldAccess::equals(<__EnumModel as ::rorm::Model>::F.identifier, identifier))
                                    .collect()
                            ))
                            .await?;
                    }
                    if !to_insert.is_empty() {
                        ::rorm::insert!(guard.get_transaction(), __EnumModel)
                            .return_nothing()
                            .bulk(
                                to_insert.into_iter()
                                    .map(|identifier| __EnumModel { identifier })
                            )
                            .await?;
                    }

                    guard.commit().await?;
                    Ok(())
                }
            }

            #[linkme::distributed_slice($crate::utils::custom_db_enum::MIGRATE_DB)]
            pub static MIGRATE_DB: fn(&mut ::rorm::db::transaction::Transaction) -> ::futures::future::BoxFuture<'_, Result<(), rorm::Error>> = |tx| Box::pin($Enum::migrate_db(tx));
        };
    }
}

/// Tests whether the enum passed to [`custom_db_enum!`](crate::custom_db_enum) has the required derives from [`strum`].
#[allow(private_bounds)]
#[allow(dead_code)]
pub const fn test_strum_derives<E: StrumDerives>() {}

/// Helper trait to modify [`test_strum_derives`]'s error message
#[diagnostic::on_unimplemented(
    message = "missing required strum derives",
    label = "missing required strum derives",
    note = "please add `#[derive(strum::EnumString, strum::IntoStaticStr, strum::EnumIter)]` to your enum"
)]
#[allow(dead_code)]
trait StrumDerives {}
impl<E: FromStr + Into<&'static str> + IntoEnumIterator> StrumDerives for E {}
