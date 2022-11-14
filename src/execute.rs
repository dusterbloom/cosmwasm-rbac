use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response};
use cosmwasm_schema::{cw_serde};

use crate::{errors::RbacError, Role};

#[cw_serde]

pub enum RbacExecuteMsg {
    Grant { address: Addr },
    Revoke { address: Addr },
    Transfer { to: Addr },
}

impl<'a> Role<'a> {
    pub fn handle_execute(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        msg: RbacExecuteMsg,
    ) -> Result<Response, RbacError> {
        match msg {
            RbacExecuteMsg::Grant { address } => {
                self.grant(deps.storage, address.clone())?;

                Ok(Response::new().add_attributes(vec![
                    ("action", "grant"),
                    ("role", self.name),
                    ("address", address.as_str()),
                ]))
            }
            RbacExecuteMsg::Revoke { address } => {
                self.revoke(deps.storage, address.clone())?;

                Ok(Response::new().add_attributes(vec![
                    ("action", "revoke"),
                    ("role", self.name),
                    ("address", address.as_str()),
                ]))
            }
            RbacExecuteMsg::Transfer { to } => {
                self.check(deps.storage, &info.sender)?;
                self.revoke(deps.storage, info.sender.clone())?;
                self.grant(deps.storage, to.clone())?;

                Ok(Response::new()
                    .add_attributes(vec![
                        ("action", "revoke"),
                        ("role", self.name),
                        ("address", info.sender.as_str()),
                    ])
                    .add_attributes(vec![
                        ("action", "grant"),
                        ("role", self.name),
                        ("address", to.as_str()),
                    ]))
            }
        }
    }
}
