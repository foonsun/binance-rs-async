use crate::client::*;
use crate::errors::*;
use crate::rest_model::*;

static SAPI_V1_ETP_INFO: &str = "/sapi/v1/blvt/tokenInfo";
static SAPI_V1_ETP_SUBSCRIBE: &str = "/sapi/v1/blvt/subscribe";
static SAPI_V1_ETP_REDEEM: &str = "/sapi/v1/blvt/redeem";
static SAPI_V1_ETP_USER_LIMIT: &str = "/sapi/v1/blvt/userLimit";
//TODO: add the records. 
// static SAPI_V1_ETP_SUBSCRIBE_RECORD: &str = "/sapi/v1/blvt/subscribe/record";
// static SAPI_V1_ETP_REDEEM_RECORD: &str = "/sapi/v1/blvt/redeem/record";

#[derive(Clone)]
pub struct ETP {
    pub client: Client,
    pub recv_window: u64,
}

impl ETP {
    /// Query ETP token info.
    pub async fn get_etp_info<S>(&self, token_name: S) -> Result<Vec<TokenInfo>>
    where
        S: Into<Option<String>>,
    {
        if let Some(token_name) = token_name.into() { 
            self.client
                .get_signed_p(SAPI_V1_ETP_INFO, Some(TokenNameQuery{ token_name: token_name.to_string() }))
                .await
        }
        else
        {
            self.client
                .get_signed_p(SAPI_V1_ETP_INFO, Some(""))
                .await
        }

    }
    ///Subscribe token
    pub async fn etp_subscribe<S, C>(&self, token_name: S, cost: C) -> Result<SubscribeResult>
    where
        S: Into<String>,
        C: Into<f64>,
    {
        let subscribe = Subscribe {
            token_name: token_name.into(),
            cost: cost.into(),
        };

        self.client
            .post_signed_p(SAPI_V1_ETP_SUBSCRIBE, subscribe, self.recv_window)
            .await
    }

    // Redeem token
    pub async fn etp_redeem<S, A>(&self, token_name: S, amount: A) -> Result<RedeemResult>
    where
        S: Into<String>,
        A: Into<f64>,
    {
        let redeem = Redeem {
            token_name: token_name.into(),
            amount: amount.into(),
        };

        self.client
            .post_signed_p(SAPI_V1_ETP_REDEEM, redeem, self.recv_window)
            .await

    }

    /// Query the ETP limit
    pub async fn get_etp_limit<S>(&self, token_name: S) -> Result<Vec<EtpLimit>>
    where
        S: Into<Option<String>>,
    {
        if let Some(token_name) = token_name.into() { 
            self.client
                .get_signed_p(SAPI_V1_ETP_USER_LIMIT, Some(TokenNameQuery{ token_name: token_name.to_string() }))
                .await
        }
        else
        {
            self.client
                .get_signed_p(SAPI_V1_ETP_USER_LIMIT, Some(""))
                .await
        }

    }

}

