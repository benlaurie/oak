/*
 * Copyright 2019 The Project Oak Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "asylo/util/logging.h"

#include "oak/server/grpc_event_handler.h"
#include "oak/server/grpc_stream.h"

namespace oak {
namespace grpc_server {

void StreamCreationEventHandler::handle() {
  LOG(INFO) << "gRPC Event: New stream created";
  stream_->server_reader_writer().Read(&stream_->request_buffer(),
                                       new RequestReadEventHandler(stream_));
}

void RequestReadEventHandler::handle() {
  LOG(INFO) << "gRPC Event: Completed reading request";
  // Invoke the actual gRPC handler on the Oak Node.
  ::grpc::Status status = stream_->node()->HandleGrpcCall(
      &stream_->server_context(), &stream_->request_buffer(), &stream_->response_buffer());
  if (!status.ok()) {
    LOG(WARNING) << "Failed: " << status.error_message();
  }

  ::grpc::WriteOptions options;

  // Write response data.
  stream_->server_reader_writer().WriteAndFinish(stream_->response_buffer(), options, status,
                                                 new ResponseWrittenEventHandler(stream_));
}

void ResponseWrittenEventHandler::handle() {
  LOG(INFO) << "gRPC Event: Completed writing response";
}

}  // namespace grpc_server
}  // namespace oak