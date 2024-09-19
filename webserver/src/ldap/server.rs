//! The server module

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use futures_util::SinkExt;
use futures_util::StreamExt;
use ldap3_proto::DisconnectionNotice;
use ldap3_proto::LdapCodec;
use ldap3_proto::LdapMsg;
use ldap3_proto::LdapResultCode;
use ldap3_proto::ServerOps;
use ldap3_proto::SimpleBindRequest;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_util::codec::FramedRead;
use tokio_util::codec::FramedWrite;
use tracing::debug;
use tracing::info;

use crate::config::Config;
use crate::global::GLOBAL;
use crate::models::User;
use crate::utils::hashing;
use crate::utils::hashing::VerifyPwError;

/// Execute the bind operation
pub async fn do_bind(sbr: &SimpleBindRequest) -> Result<LdapMsg, LdapMsg> {
    let mut parts = sbr.dn.split("=");
    let dn = parts
        .next()
        .ok_or(sbr.gen_error(LdapResultCode::Other, "Invalid dn".to_string()))?;
    let username = parts
        .next()
        .ok_or(sbr.gen_error(LdapResultCode::Other, "Invalid dn".to_string()))?;

    if dn.to_lowercase() != "dn" || username.is_empty() {
        return Err(sbr.gen_error(LdapResultCode::Other, "Invalid dn".to_string()));
    }

    let db = &GLOBAL.db;

    // Search user
    let user = query!(db, User)
        .condition(User::F.username.equals(username))
        .optional()
        .await
        .map_err(|_e| sbr.gen_error(LdapResultCode::Other, "Internal Server Error".to_string()))?
        .ok_or(sbr.gen_invalid_cred())?;

    // Verify password
    hashing::verify_pw(&sbr.pw, &user.password).map_err(|x| match x {
        VerifyPwError::Hash(_) => {
            sbr.gen_error(LdapResultCode::Other, "Internal Server Error".to_string())
        }
        VerifyPwError::Mismatch => sbr.gen_invalid_cred(),
    })?;

    Ok(sbr.gen_success())
}

/// Handle the client connection
pub async fn handle_client(socket: TcpStream, _addr: SocketAddr) {
    // Configure the codec etc.
    let (r, w) = tokio::io::split(socket);
    let mut reqs = FramedRead::new(r, LdapCodec::default());
    let mut resp = FramedWrite::new(w, LdapCodec::default());

    while let Some(req) = reqs.next().await {
        let Ok(Ok(server_ops)) = req.map(ServerOps::try_from) else {
            let _err = resp
                .send(DisconnectionNotice::gen(
                    LdapResultCode::Other,
                    "Internal Server Error",
                ))
                .await;
            let _err = resp.flush().await;
            return;
        };

        let ServerOps::SimpleBind(sbr) = server_ops else {
            return;
        };

        let msg = do_bind(&sbr).await.unwrap_or_else(|x| x);

        if resp.send(msg).await.is_err() {
            return;
        }
        if resp.flush().await.is_err() {
            return;
        }
    }
}

/// Start the LDAP listener
pub async fn start_server(config: Arc<Config>) -> Result<JoinHandle<()>, io::Error> {
    let addr = SocketAddr::new(config.ldap.listen_address, config.ldap.listen_port);

    info!("Start to listen on ldap://{}", addr);
    let listener = TcpListener::bind(&addr).await?;

    Ok(tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    debug!("Handle ldap client");
                    tokio::spawn(handle_client(socket, addr));
                }
                Err(err) => {
                    debug!("accept error = {:?}", err);
                }
            }
        }
    }))
}
