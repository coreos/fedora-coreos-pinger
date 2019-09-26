// Copyright 2019 Red Hat, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! aws ec2 metadata fetcher
//!

use std::collections::HashMap;

#[cfg(test)]
use mockito;

use crate::errors::*;
use crate::providers::MetadataProvider;
use crate::retry;

#[cfg(test)]
mod mock_tests;

#[cfg(not(feature = "cl-legacy"))]
static ENV_PREFIX: &str = "AWS";
#[cfg(feature = "cl-legacy")]
static ENV_PREFIX: &str = "EC2";

#[derive(Clone, Debug)]
pub struct AwsProvider {
    client: retry::Client,
}

impl AwsProvider {
    pub fn try_new() -> Result<AwsProvider> {
        let client = retry::Client::try_new()?.return_on_404(true);

        Ok(AwsProvider { client })
    }

    #[cfg(test)]
    fn endpoint_for(key: &str) -> String {
        let url = mockito::server_url();
        format!("{}/{}", url, key)
    }

    #[cfg(not(test))]
    fn endpoint_for(key: &str) -> String {
        const URL: &str = "http://169.254.169.254/2009-04-04";
        format!("{}/{}", URL, key)
    }
}

impl MetadataProvider for AwsProvider {
    fn get_attributes(&self) -> Result<HashMap<String, String>> {
        let mut out = HashMap::with_capacity(6);

        let add_value = |map: &mut HashMap<_, _>, key: &str, name| -> Result<()> {
            let value = self
                .client
                .get(retry::Raw, AwsProvider::endpoint_for(name))
                .send()?;

            if let Some(value) = value {
                map.insert(key.to_string(), value);
            }

            Ok(())
        };

        add_value(
            &mut out,
            &format!("{}_INSTANCE_ID", ENV_PREFIX),
            "meta-data/instance-id",
        )?;

        Ok(out)
    }
}
