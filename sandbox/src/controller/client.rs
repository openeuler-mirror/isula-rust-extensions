// Copyright (c) 2024 Huawei Technologies Co.,Ltd. All rights reserved.
//
// isula-rust-extensions is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//         http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

pub mod sandbox;
pub mod cri;

use sandbox::containerd::services::sandbox::v1::controller_client::ControllerClient;
use sandbox::containerd::services::sandbox::v1::ControllerCreateRequest;
use sandbox::containerd::services::sandbox::v1::ControllerCreateResponse;
use sandbox::containerd::services::sandbox::v1::ControllerStartRequest;
use sandbox::containerd::services::sandbox::v1::ControllerStartResponse;
use sandbox::containerd::services::sandbox::v1::ControllerPlatformRequest;
use sandbox::containerd::services::sandbox::v1::ControllerPlatformResponse;
use sandbox::containerd::services::sandbox::v1::ControllerStopRequest;
use sandbox::containerd::services::sandbox::v1::ControllerWaitRequest;
use sandbox::containerd::services::sandbox::v1::ControllerWaitResponse;
use sandbox::containerd::services::sandbox::v1::ControllerStatusRequest;
use sandbox::containerd::services::sandbox::v1::ControllerStatusResponse;
use sandbox::containerd::services::sandbox::v1::ControllerShutdownRequest;
use sandbox::containerd::services::sandbox::v1::ControllerMetricsRequest;
use sandbox::containerd::services::sandbox::v1::ControllerMetricsResponse;
use sandbox::containerd::services::sandbox::v1::ControllerUpdateRequest;
use sandbox::containerd::services::sandbox::v1::ControllerUpdateResponse;

use tonic::transport::Channel;

pub struct Client {
    pub client: ControllerClient<Channel>,
}

pub async fn connect(
    path: impl AsRef<std::path::Path>,
) -> Result<tonic::transport::Channel, tonic::transport::Error> {
    use std::convert::TryFrom;

    use tokio::net::UnixStream;
    use tonic::transport::Endpoint;

    let path = path.as_ref().to_path_buf();

    let channel = Endpoint::try_from("https://[::]")
        .unwrap()
        .connect_with_connector(tower::service_fn(move |_| {
            UnixStream::connect(path.clone())
        }))
        .await?;

    Ok(channel)
}

impl Client {
    pub async fn new(address: String) -> Result<Client, Box<dyn std::error::Error>> {
        let channel = connect(address).await?;
        let client = ControllerClient::new(channel);
        Ok(Client { client })
    }

    pub async fn create(
        &mut self,
        request: ControllerCreateRequest) -> Result<ControllerCreateResponse, tonic::Status> {
        let response = self.client.create(request).await?;
        Ok(response.into_inner())
    }

    pub async fn start(
        &mut self,
        request: ControllerStartRequest) -> Result<ControllerStartResponse, tonic::Status> {
        let response = self.client.start(request).await?;
        Ok(response.into_inner())
    }

    pub async fn platform(
        &mut self,
        request: ControllerPlatformRequest) -> Result<ControllerPlatformResponse, tonic::Status> {
        let response = self.client.platform(request).await?;
        Ok(response.into_inner())
    }

    pub async fn stop(
        &mut self,
        request: ControllerStopRequest) -> Result<(), tonic::Status> {
        self.client.stop(request).await?;
        Ok(())
    }

    pub async fn wait(
        &mut self,
        request: ControllerWaitRequest) -> Result<ControllerWaitResponse, tonic::Status> {
        let response = self.client.wait(request).await?;
        Ok(response.into_inner())
    }

    pub async fn status(
        &mut self,
        request: ControllerStatusRequest) -> Result<ControllerStatusResponse, tonic::Status> {
        let response = self.client.status(request).await?;
        Ok(response.into_inner())
    }

    pub async fn shutdown(
        &mut self,
        request: ControllerShutdownRequest) -> Result<(), tonic::Status> {
        self.client.shutdown(request).await?;
        Ok(())
    }

    pub async fn metrics(
        &mut self,
        request: ControllerMetricsRequest) -> Result<ControllerMetricsResponse, tonic::Status> {
        let response = self.client.metrics(request).await?;
        Ok(response.into_inner())
    }

    pub async fn update(
        &mut self,
        request: ControllerUpdateRequest) -> Result<ControllerUpdateResponse, tonic::Status> {
        let response = self.client.update(request).await?;
        Ok(response.into_inner())
    }
}