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

#ifndef LIB_SANDBOX_API_H
#define LIB_SANDBOX_API_H

#include <sys/types.h>
#include <stdint.h>

#include <isula_libutils/sandbox_create_request.h>
#include <isula_libutils/sandbox_create_response.h>
#include <isula_libutils/sandbox_start_request.h>
#include <isula_libutils/sandbox_start_response.h>
#include <isula_libutils/sandbox_platform_request.h>
#include <isula_libutils/sandbox_platform_response.h>
#include <isula_libutils/sandbox_stop_request.h>
#include <isula_libutils/sandbox_wait_request.h>
#include <isula_libutils/sandbox_wait_response.h>
#include <isula_libutils/sandbox_status_request.h>
#include <isula_libutils/sandbox_status_response.h>
#include <isula_libutils/sandbox_shutdown_request.h>
#include <isula_libutils/sandbox_metrics_request.h>
#include <isula_libutils/sandbox_metrics_response.h>
#include <isula_libutils/sandbox_update_request.h>

#ifdef __cplusplus
extern "C" {
#endif

struct ControllerContext;

typedef struct ControllerContext *ControllerHandle_t;

typedef int (*sandbox_api_ready_callback)(
    void *cb_context
);
typedef int (*sandbox_api_pending_callback)(
    void *cb_context
);
typedef int (*sandbox_api_exit_callback)(
    void *cb_context,
    const sandbox_wait_response *request
);

typedef struct {
    sandbox_api_ready_callback ready;
    sandbox_api_pending_callback pending;
    sandbox_api_exit_callback exit;
} sandbox_api_wait_callback;

/**
 * @brief Initialize the controller handle.
 * @param sandboxer the sandboxer name.
 * @param address the address of the sandboxer.
 * @return the controller handle.
 */
ControllerHandle_t sandbox_api_build_controller(const char *sandboxer, const char *address);

int sandbox_api_create(ControllerHandle_t chandle, const sandbox_create_request *request, sandbox_create_response *response);

int sandbox_api_start(ControllerHandle_t chandle, const sandbox_start_request *request, sandbox_start_response *response);

int sandbox_api_platform(ControllerHandle_t chandle, const sandbox_platform_request *request, sandbox_platform_response *response);

int sandbox_api_stop(ControllerHandle_t chandle, const sandbox_stop_request *request);

int sandbox_api_wait(ControllerHandle_t chandle, const sandbox_wait_request *request, sandbox_api_wait_callback callback, void *cb_context);

int sandbox_api_status(ControllerHandle_t chandle, const sandbox_status_request *request, sandbox_status_response *response);

int sandbox_api_shutdown(ControllerHandle_t chandle, const sandbox_shutdown_request *request);

int sandbox_api_metrics(ControllerHandle_t chandle, const sandbox_metrics_request *request, sandbox_metrics_response *response);

int sandbox_api_update(ControllerHandle_t chandle, const sandbox_update_request *request);

#ifdef __cplusplus
}
#endif

#endif /* LIB_SANDBOX_API_H */