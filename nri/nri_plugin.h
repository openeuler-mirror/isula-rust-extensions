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

#ifndef LIB_NRI_PLUGIN_H
#define LIB_NRI_PLUGIN_H

#include <isula_libutils/nri_create_container_request.h>
#include <isula_libutils/nri_create_container_response.h>
#include <isula_libutils/nri_configure_request.h>
#include <isula_libutils/nri_configure_response.h>
#include <isula_libutils/nri_register_plugin_request.h>
#include <isula_libutils/nri_state_change_event.h>
#include <isula_libutils/nri_stop_container_request.h>
#include <isula_libutils/nri_stop_container_response.h>
#include <isula_libutils/nri_synchronize_request.h>
#include <isula_libutils/nri_synchronize_response.h>
#include <isula_libutils/nri_update_container_request.h>
#include <isula_libutils/nri_update_container_response.h>
#include <isula_libutils/nri_update_containers_request.h>
#include <isula_libutils/nri_update_containers_response.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef int (*nri_runtime_register_plugin_callback)(
  const char *plugin_id,
  const nri_register_plugin_request *request
);

typedef int (*nri_runtime_update_containers_callback)(
  const char *plugin_id,
  const nri_update_containers_request *request,
  nri_update_containers_response **response
);

typedef struct nri_runtime_callbasks {
  nri_runtime_register_plugin_callback register_plugin;
  nri_runtime_update_containers_callback update_containers;
} nri_runtime_callbacks;

int nri_runtime_service_init(nri_runtime_callbacks callbacks);

void nri_runtime_service_destroy();

typedef int (*nri_external_connect_callback)(
  int fd
);

int nri_external_service_start(const char *socket_addr,
                               nri_external_connect_callback callback);

void nri_external_service_shutdown();

int nri_plugin_connect(const char *plugin_id, int fd, int64_t timeout);

int nri_plugin_disconnect(const char *plugin_id);

int nri_plugin_configure(const char *plugin_id,
                         const nri_configure_request *request,
                         nri_configure_response **response);

int nri_plugin_synchronize(const char *plugin_id,
                           const nri_synchronize_request *request,
                           nri_synchronize_response **response);

int nri_plugin_shutdown(const char *plugin_id);

int nri_plugin_create_container(const char *plugin_id,
                                const nri_create_container_request *request,
                                nri_create_container_response **response);

int nri_plugin_update_container(const char *plugin_id,
                                const nri_update_container_request *request,
                                nri_update_container_response **response);

int nri_plugin_stop_container(const char *plugin_id,
                              const nri_stop_container_request *request,
                              nri_stop_container_response **response);

int nri_plugin_state_change(const char *plugin_id,
                            const nri_state_change_event *event);

#ifdef __cplusplus
}
#endif

#endif /* LIB_NRI_CLIENT_H */